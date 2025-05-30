use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use usbd_hid::hid_class::HIDClass;

static mut HID_CLASS: MaybeUninit<HIDClass<'static, UsbBus>> = MaybeUninit::uninit();

pub unsafe fn init_class(usb_alloc: &'static UsbBusAllocator<UsbBus>) {
	let hid_class = HIDClass::new(usb_alloc, KeyboardReport::desc(), 1);

	let ptr = &raw mut HID_CLASS;

	unsafe {
		(*ptr).write(hid_class);
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
pub unsafe fn get_class_ref<'a>() -> &'a HIDClass<'static, UsbBus> {
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
pub unsafe fn get_class_ref_mut<'a>() -> &'a mut HIDClass<'static, UsbBus> {
	let ptr = &raw mut HID_CLASS;

	unsafe { (*ptr).assume_init_mut() }
}
