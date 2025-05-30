use core::marker::PhantomData;
use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::ReportType;

mod hid;
#[cfg(feature = "serial")]
pub mod serial;
#[cfg(feature = "silverplate")]
pub mod silverplate;

// TODO: These IDs should be unique per device and moved somewhere else.
const USB_VID: u16 = 0x16C0;
const USB_PID: u16 = 0x27DD;

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

		unsafe {
			hid::init_class(usb_alloc);

			#[cfg(feature = "serial")]
			serial::init_class(usb_alloc);
		}

		// Initialize USB device.
		let vid_pid = UsbVidPid(USB_VID, USB_PID);
		let descriptors = StringDescriptors::default()
			.manufacturer("cloudgazing")
			.product("KB-RS-CLD")
			.serial_number("CLD-KB");

		let usb_device = {
			let res = UsbDeviceBuilder::new(usb_alloc, vid_pid).strings(&[descriptors]);

			// Safety: [`UsbDeviceBuilder::strings`] can take up to 16 languages and we're
			// giving it one.
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
			let hid_class = unsafe { hid::get_class_ref() };

			_ = hid_class.push_input(report);

			#[cfg(feature = "serial")]
			{
				let serial_port = unsafe { serial::get_class_ref_mut() };

				serial::write_report_message(serial_port, report);
			}
		});
	}
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

	let hid_class = unsafe { hid::get_class_ref_mut() };
	#[cfg(feature = "serial")]
	let serial_port = unsafe { serial::get_class_ref_mut() };

	// The classes are passed in the same order they were configured in.
	let has_new_data = device.poll(&mut [
		hid_class,
		#[cfg(feature = "serial")]
		serial_port,
	]);

	#[cfg(feature = "silverplate")]
	if has_new_data {
		let mut buf = [0_u8; 64];

		if let Ok(report_info) = hid_class.pull_raw_report(&mut buf) {
			#[cfg(feature = "serial")]
			{
				let message = match report_info.report_type {
					ReportType::Feature => "got some feature report data!!\n",
					ReportType::Input => "got some input report data!!\n",
					ReportType::Output => "got some output report data!!\n",
					ReportType::Reserved => "got some reserved report data!!\n",
				};

				_ = serial::write_serial_message(serial_port, message.as_bytes());
			}
		}
	}
}
