use core::marker::PhantomData;
use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usbd_hid::hid_class::ReportType;

use crate::report::RawKeyboardReport;

pub mod hid;
#[cfg(feature = "serial")]
pub mod serial;
#[cfg(feature = "silverplate")]
pub mod silverplate;

// TODO: These IDs should be unique per device and moved somewhere else.
const USB_VID: u16 = 0x16C0;
const USB_PID: u16 = 0x27DD;

static mut USB_ALLOC: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit();
static mut USB_DEVICE: MaybeUninit<UsbDevice<UsbBus>> = MaybeUninit::uninit();

#[derive(Debug)]
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
			.product("MoonQuartz")
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
	pub fn send_keyboard_report(&self, report: &RawKeyboardReport) {
		cortex_m::interrupt::free(|_| {
			// Safety: We execute this inside the critical section which prevents two mutable references
			// to the value from being created.
			let hid_class = unsafe { hid::get_class_ref() };

			_ = hid_class.push_raw_input(report.as_ref());

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

	if has_new_data {
		let mut buf = [0_u8; 64];

		let Ok(rep_info) = hid_class.pull_raw_report(&mut buf) else {
			return;
		};

		if rep_info.report_type == ReportType::Output {
			#[cfg(not(feature = "silverplate"))]
			{
				// Handle LED here.

				#[cfg(feature = "serial")]
				let _ = serial::write_serial_message(serial_port, b"got some other output report data!!\n".as_ref());
			}

			#[cfg(feature = "silverplate")]
			match buf[0] {
				hid::KEYPRESS_REPORT_OUT_ID => {
					// Handle LED here.

					#[cfg(feature = "serial")]
					let _ = serial::write_serial_message(serial_port, b"got some output report data!!\n".as_ref());
				}
				silverplate::VENDOR_REPORT_OUT_ID => {
					let vendor_byte = buf[1];

					silverplate::process_vendor_report(
						hid_class,
						vendor_byte,
						#[cfg(feature = "serial")]
						serial_port,
					);
				}
				_ => {}
			}
		}
	}
}
