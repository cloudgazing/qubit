use core::mem::MaybeUninit;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;
use usbd_hid::hid_class::{HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig};

pub const KEYPRESS_REPORT_IN_ID: u8 = 0x01;
pub const KEYPRESS_REPORT_OUT_ID: u8 = 0x02;

#[cfg(feature = "silverplate")]
pub use super::silverplate::REPORT_DESCRIPTOR;

// https://usb.org/document-library/hid-usage-tables-16
// https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages

#[cfg(not(feature = "silverplate"))]
#[rustfmt::skip]
pub const REPORT_DESCRIPTOR: &[u8] = &[
	0x05, 0x01, // UsagePage (Generic Desktop)
	0x09, 0x06, // UsageId(Keyboard[6])
	0xA1, 0x01, // Collection(Application)

	// -- Input Report for Key Presses --
	0x85, KEYPRESS_REPORT_IN_ID, // ReportId(1)
	// --- Modifier Keys (1 byte) ---
	0x05, 0x07,                  // UsagePage(Keyboard/Keypad)
	0x19, 0xE0,                  // UsageMinimum(Keyboard LeftControl)
	0x29, 0xE7,                  // UsageMaximum(Keyboard RightGUI)
	0x15, 0x00,                  // LogicalMinimum(0)
	0x25, 0x01,                  // LogicalMaximum(1)
	0x75, 0x01,                  // ReportSize(1)
	0x95, 0x08,                  // ReportCount(8)
	0x81, 0x02,                  // Input(Data, Variable, Absolute)
	// --- Reserved Byte (1 byte) ---
	0x75, 0x08,                  // ReportSize(8)
	0x95, 0x01,                  // ReportCount(1)
	0x81, 0x01,                  // Input(Constant)
	// --- 6-Key Rollover Keycode Array (6 bytes) ---
	0x05, 0x07,                  // UsagePage(Keyboard/Keypad)
	0x19, 0x00,                  // UsageMinimum(0x00)
	0x29, 0xFF,                  // UsageMaximum(0xFF)
	0x15, 0x00,                  // LogicalMinimum(0x00)
	0x26, 0xFF, 0x00,            // LogicalMaximum(255)
	0x75, 0x08,                  // ReportSize(8)
	0x95, 0x06,                  // ReportCount(6)
	0x81, 0x00,                  // Input(Data, Array, Absolute)

	// --- Output Report for LEDs ---
	0x85, KEYPRESS_REPORT_OUT_ID, // ReportId(2)
	// --- LEDs Byte (1 byte) ---
	0x05, 0x08,                   // UsagePage(LEDs)
	0x19, 0x01,                   // UsageMinimum(Num Lock)
	0x29, 0x05,                   // UsageMaximum(Kana)
	0x15, 0x00,                   // LogicalMinimum(0)
	0x25, 0x01,                   // LogicalMaximum(1)
	0x75, 0x01,                   // ReportSize(1)
	0x95, 0x05,                   // ReportCount(5)
	0x91, 0x02,                   // Output(Data, Variable, Absolute)
	// --- Remaining bits for padding ---
	0x75, 0x03,                   // ReportSize(3)
	0x95, 0x01,                   // ReportCount(1)
	0x91, 0x01,                   // Output(Constant)
	0xC0, // EndCollection()
];

static mut HID_CLASS: MaybeUninit<HIDClass<'static, UsbBus>> = MaybeUninit::uninit();

pub(crate) unsafe fn init_class(usb_alloc: &'static UsbBusAllocator<UsbBus>) {
	let hid_settings = HidClassSettings {
		subclass: HidSubClass::NoSubClass,
		protocol: HidProtocol::Keyboard,
		config: ProtocolModeConfig::ForceReport,
		locale: HidCountryCode::US,
	};

	let hid_class = HIDClass::new_ep_in_with_settings(usb_alloc, REPORT_DESCRIPTOR, 10, hid_settings);

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
pub(crate) unsafe fn get_class_ref<'a>() -> &'a HIDClass<'static, UsbBus> {
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
pub(crate) unsafe fn get_class_ref_mut<'a>() -> &'a mut HIDClass<'static, UsbBus> {
	let ptr = &raw mut HID_CLASS;

	unsafe { (*ptr).assume_init_mut() }
}
