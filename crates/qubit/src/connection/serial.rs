use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usbd_hid::UsbError;
use usbd_hid::descriptor::KeyboardReport;
use usbd_serial::SerialPort;

static mut SERIAL_PORT: MaybeUninit<SerialPort<'static, UsbBus>> = MaybeUninit::uninit();

pub unsafe fn init_class(usb_alloc: &'static UsbBusAllocator<UsbBus>) {
	let serial = SerialPort::new(usb_alloc);

	let ptr = &raw mut SERIAL_PORT;

	unsafe {
		(*ptr).write(serial);
	}
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
pub unsafe fn get_class_ref_mut<'a>() -> &'a mut SerialPort<'static, UsbBus> {
	let ptr = &raw mut SERIAL_PORT;

	unsafe { (*ptr).assume_init_mut() }
}

pub fn write_serial_message<'a, T: Into<&'a [u8]>>(port: &mut SerialPort<UsbBus>, data: T) -> Result<usize, UsbError> {
	port.write(data.into())
}

pub fn write_report_message(port: &mut SerialPort<UsbBus>, report: &KeyboardReport) {
	_ = write_serial_message(port, b"-- KEY REPORT --\n".as_ref());

	_ = write_serial_message(port, b"[".as_ref());
	for code in report.keycodes {
		if code != 0 {
			_ = write_serial_message(port, b"KEY, ".as_ref());
		}
	}
	_ = write_serial_message(port, b"]\n".as_ref());

	_ = write_serial_message(port, b"----------------\n".as_ref());
}
