use core::marker::PhantomData;
use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::{HIDClass, ReportType};
#[cfg(feature = "serial")]
use usbd_serial::SerialPort;
#[cfg(feature = "serial")]
use usbd_serial::embedded_io::WriteReady;

const USB_VID: u16 = 0x16C0;
const USB_PID: u16 = 0x27DD;

static mut HID_CLASS: MaybeUninit<HIDClass<'static, UsbBus>> = MaybeUninit::uninit();
#[cfg(feature = "serial")]
static mut SERIAL_PORT: MaybeUninit<SerialPort<'static, UsbBus>> = MaybeUninit::uninit();

static mut USB_ALLOC: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit();
static mut USB_DEVICE: MaybeUninit<UsbDevice<UsbBus>> = MaybeUninit::uninit();

pub struct UsbDeviceInstance<'a> {
	_marker: PhantomData<&'a mut ()>,
}

impl UsbDeviceInstance<'_> {
	/// # Safety
	///
	/// This function must be called **only once** for the lifetime of the program.
	/// Calling it more than once is **undefined behavior** because it writes to static
	/// mutable memory without synchronization.
	pub unsafe fn new(usb_alloc: UsbBusAllocator<UsbBus>) -> Self {
		let usb_alloc = {
			let ptr = &raw mut USB_ALLOC;

			unsafe { (*ptr).write(usb_alloc) }
		};

		// Initiate device class list.
		{
			let hid_class = HIDClass::new(usb_alloc, KeyboardReport::desc(), 1);

			let ptr = &raw mut HID_CLASS;

			unsafe {
				(*ptr).write(hid_class);
			}
		}

		#[cfg(feature = "serial")]
		{
			let serial = SerialPort::new(usb_alloc);

			let ptr = &raw mut SERIAL_PORT;

			unsafe {
				(*ptr).write(serial);
			}
		}

		// Initialize USB device.
		let vid_pid = UsbVidPid(USB_VID, USB_PID);
		let descriptors = StringDescriptors::default()
			.manufacturer("cloudgazing")
			.product("KB-RS-CLD")
			.serial_number("CLD-KB");

		let usb_device = {
			let res = UsbDeviceBuilder::new(usb_alloc, vid_pid).strings(&[descriptors]);

			// Safety: [`UsbDeviceBuilder::strings`] can take up to 16 languages and we're giving it one.
			let d = unsafe { res.unwrap_unchecked() };

			d.device_class(0x00).composite_with_iads().build()
		};

		{
			let ptr = &raw mut USB_DEVICE;

			unsafe {
				(*ptr).write(usb_device);
			}
		}

		UsbDeviceInstance { _marker: PhantomData }
	}

	/// Send a keyboard report.
	///
	/// The report is written inside interrupt-free context.
	// Allow this because I want the caller to get a device instance to send reports.
	#[allow(clippy::unused_self)]
	pub fn send_keyboard_report(&self, report: &KeyboardReport) {
		cortex_m::interrupt::free(|_| {
			// Safety: We execute this inside the critical section which prevents two mutable references
			// to the value from being created.
			let hid_class = unsafe { get_hid_class_ref() };

			_ = hid_class.push_input(report);
		});
	}

	// TODO: Maybe delegate the serial messaging to logger or something similar to make it easier
	// (and safer) to use.
	/// Send a serial message by writing bytes into the port.
	///
	/// The data written inside interrupt-free context.
	// Allow this because I want the caller to get a device instance to send serial messages.
	#[allow(clippy::unused_self)]
	#[cfg(feature = "serial")]
	pub fn send_serial_message(&self, msg: &[u8]) {
		cortex_m::interrupt::free(|_| {
			// Safety: We execute this inside the critical section which prevents two mutable references
			// to the value from being created.
			let serial_port = unsafe { get_serial_port_ref_mut() };

			_ = serial_port.write(msg);
		});
	}
}

/// # Safety
///
/// Calling this function is **undefined behavior** if:
///
///  * It is called **before** [`HID_CLASS`] has been initialized.
///  * It is called **concurrently with a mutable reference** to the same `HIDClass`
///    (e.g., while a `&mut HIDClass<'static, UsbBus>` is active).
///  * It is called from **multiple threads**.
///  * It is called outside an **interrupt** and **interrupt-free** context.
unsafe fn get_hid_class_ref<'a>() -> &'a HIDClass<'static, UsbBus> {
	let ptr = &raw mut HID_CLASS;

	unsafe { (*ptr).assume_init_ref() }
}

/// # Safety
///
/// Calling this function is **undefined behavior** if:
///
///  * It is called **before** [`HID_CLASS`] has been initialized.
///  * It is used to create **more than one mutable reference** to the same `HIDClass`
///    instance at any time.
///  * It is called from **multiple threads**.
///  * It is called outside an **interrupt** and **interrupt-free** context.
///  * A previously returned reference is still in use (even if not actively), which
///    may result in aliasing UB.
unsafe fn get_hid_class_ref_mut<'a>() -> &'a mut HIDClass<'static, UsbBus> {
	let ptr = &raw mut HID_CLASS;

	unsafe { (*ptr).assume_init_mut() }
}

/// # Safety
///
/// Calling this function is **undefined behavior** if:
///
///  * It is called **before** [`SERIAL_PORT`] has been initialized.
///  * It is used to create **more than one mutable reference** to the same `SerialPort`
///    instance at any time.
///  * It is called from **multiple threads**.
///  * It is called outside an **interrupt** and **interrupt-free** context.
///  * A previously returned reference is still in use (even if not actively), which
///    may result in aliasing UB.
#[cfg(feature = "serial")]
unsafe fn get_serial_port_ref_mut<'a>() -> &'a mut SerialPort<'static, UsbBus> {
	let ptr = &raw mut SERIAL_PORT;

	unsafe { (*ptr).assume_init_mut() }
}

/// Poll the USB device for new events.
///
/// # Safety
///
/// Calling this function is **undefined behavior** if:
///
/// * Any of the static components ([`USB_DEVICE`], [`HID_CLASS`], or [`SERIAL_PORT`] if enabled)
///   have not been initialized via [`UsbDeviceInstance::new`].
/// * More than one [`UsbDeviceInstance`] has been created.
/// * This function is called concurrently with any access to the same USB structures,
///   whether via mutable or immutable references.
/// * This function is called from multiple threads, as none of the USB components
///   are `Sync` or otherwise thread-safe.
/// * This function is called outside of an **interrupt context**.
pub unsafe fn poll_usb_device() {
	let device = {
		let ptr = &raw mut USB_DEVICE;

		unsafe { (*ptr).assume_init_mut() }
	};

	let hid_class = unsafe { get_hid_class_ref_mut() };

	#[cfg(feature = "serial")]
	let serial_port = unsafe { get_serial_port_ref_mut() };

	// The classes are passed in the same order they were configured in.
	let has_new_data = device.poll(&mut [
		hid_class,
		#[cfg(feature = "serial")]
		serial_port,
	]);

	#[cfg(feature = "serial")]
	if serial_port.write_ready().is_err() {
		return;
	}

	if has_new_data {
		let mut buf = [0_u8; 64];

		if let Ok(report_info) = hid_class.pull_raw_report(&mut buf) {
			#[cfg(feature = "serial")]
			match report_info.report_type {
				ReportType::Feature => {
					_ = serial_port.write(b"got some feature report data!!\n");
				}
				ReportType::Input => {
					_ = serial_port.write(b"got some input report data!!\n");
				}
				ReportType::Output => {
					_ = serial_port.write(b"got some output report data!!\n");
				}
				ReportType::Reserved => {
					_ = serial_port.write(b"got some reserved report data!!\n");
				}
			}
		}
	}
}
