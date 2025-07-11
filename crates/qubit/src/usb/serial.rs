use core::mem::MaybeUninit;

use usb_device::bus::UsbBusAllocator;
use usbd_serial::SerialPort;

use crate::setup::hal::usb::UsbBus;

static mut SERIAL_PORT: MaybeUninit<SerialPort<'static, UsbBus>> = MaybeUninit::uninit();

/// # Safety
///
/// This function must only be called **once** for the entire lifetime of the program.
pub unsafe fn init_class(usb_bus_alloc: &'static UsbBusAllocator<UsbBus>) {
	let serial_port = SerialPort::new(usb_bus_alloc);

	let ptr = &raw mut SERIAL_PORT;

	// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and properly
	// aligned. This sets the value of the MaybeUninit.
	unsafe {
		(*ptr).write(serial_port);
	}
}

/// Returns a mutable reference for the serial port.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * [`init_class`] must have been called to initialize the contents **before** this function.
/// * No other reference to the static value exists.
/// * The function needs to be called inside an **interrupt** or **interrupt-free** context
pub unsafe fn get_mut<'a>() -> &'a mut SerialPort<'static, UsbBus> {
	let ptr = &raw mut SERIAL_PORT;

	// SAFETY: The caller guarantees the value was already initialized.
	unsafe { (*ptr).assume_init_mut() }
}

pub fn write_message(serial_port: &mut SerialPort<UsbBus>, msg: &[u8]) {
	_ = serial_port.write(msg);
}
