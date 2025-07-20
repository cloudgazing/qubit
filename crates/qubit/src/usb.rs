use core::mem::MaybeUninit;

use usb_device::device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbVidPid};

use qubit_config::general::Device;

use crate::DEVICE_CONFIG;
use crate::codegen::{KeyboardMatrix, USB_PID, USB_VID};
use crate::setup::{UsbBus, UsbBusAllocator};

#[cfg(keyboard)]
mod keyboard;

const DEVICE_CLASS: u8 = {
	match DEVICE_CONFIG.device {
		Device::Keyboard => 0x00,
	}
};

// USB singletons.
static mut USB_BUS_ALLOC: MaybeUninit<UsbBusAllocator> = MaybeUninit::uninit();
static mut USB_DEVICE: MaybeUninit<UsbDevice<UsbBus>> = MaybeUninit::uninit();

#[derive(Debug)]
pub struct QubitDevice {
	#[cfg(keyboard)]
	pub keyboard: keyboard::KeyboardInstance,
}

impl QubitDevice {
	/// Initialize and create a new [`QubitDevice`].
	///
	/// # Safety
	///
	/// This method will initialize all the static variables the firmware needs. This must be called
	/// **only once** for the lifetime of the program AND **before** enabling the
	/// interrupts.
	pub unsafe fn new(bus_alloc: UsbBusAllocator, matrix: KeyboardMatrix) -> Self {
		let usb_bus_alloc = {
			let ptr = &raw mut USB_BUS_ALLOC;

			// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and
			// properly aligned. This sets the value of the MaybeUninit.
			unsafe { (*ptr).write(bus_alloc) }
		};

		let vid_pid = UsbVidPid(USB_VID, USB_PID);
		let descriptors = StringDescriptors::default()
			.manufacturer(DEVICE_CONFIG.author)
			.product(DEVICE_CONFIG.name);

		// Initialize classes before building the usb device.
		// The same order the classes were initialized needs to be used when polling the usb bus.
		// See the [`poll_device`] function.

		// SAFETY: Serial was initialized above and the caller guarantees this will be called only once.
		#[cfg(keyboard)]
		let keyboard = unsafe { keyboard::KeyboardInstance::new(usb_bus_alloc, matrix) };

		let usb_device = {
			let builder_res = UsbDeviceBuilder::new(usb_bus_alloc, vid_pid).strings(&[descriptors]);

			// SAFETY: [`UsbDeviceBuilder::strings`] can take up to 16 languages and we're
			// giving it one.
			let device_builder = unsafe { builder_res.unwrap_unchecked() };

			device_builder.device_class(DEVICE_CLASS).composite_with_iads().build()
		};

		{
			let ptr = &raw mut USB_DEVICE;

			// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and
			// properly aligned. This sets the value of the MaybeUninit.
			unsafe {
				(*ptr).write(usb_device);
			}
		}

		QubitDevice {
			#[cfg(keyboard)]
			keyboard,
		}
	}
}

/// Poll the USB device for new events.
///
/// # Safety
///
/// Calling this function is **undefined behavior** if:
///
/// * Any of the static components it acceses have not been initialized. The proper way to do that
///   is to use [`QubitDevice::new`].
/// * This function is called outside of an **interrupt context**.
pub unsafe fn poll_device() {
	let device = {
		let ptr = &raw mut USB_DEVICE;

		// SAFETY: The caller guarantees the value was initialized before this function call.
		unsafe { (*ptr).assume_init_mut() }
	};

	// SAFETY: The function is called inside an interrupt. The caller guarantees initialization
	// by calling the proper method.
	#[cfg(keyboard)]
	let keyboard_hid = unsafe { keyboard::get_mut() };

	// The classes are passed in the same order they were configured in.
	let may_have_data = device.poll(&mut [
		#[cfg(keyboard)]
		keyboard_hid,
	]);

	if may_have_data {
		// Check for an incoming keyboard report.
		#[cfg(keyboard)]
		keyboard::process_incoming_report(keyboard_hid);
	}
}
