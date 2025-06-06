use core::mem::size_of_val;

use rp2040_hal as hal;

use hal::usb::UsbBus;
use qubit_macros::date_now_bitmap;
use usbd_hid::hid_class::HIDClass;
use usbd_serial::SerialPort;

use super::hid::{KEYPRESS_REPORT_IN_ID, KEYPRESS_REPORT_OUT_ID};
use super::serial::write_serial_message;
use crate::CONFIGURATION;

const BUILD_DATE: u16 = date_now_bitmap!();

pub const VENDOR_REPORT_OUT_ID: u8 = 0x03;
pub const VENDOR_REPORT_IN_ID: u8 = 0x04;

// Vendor reports coming from the host
const REQ_GET_FIRMWARE_INFO: u8 = 0x01;

// Input reports must begin with the report ID, followed by the payload.
// Although the descriptor defines the report size as the payload size, the actual data passed
// to the push function must include the report ID, making the total size payload_size + 1.

// The firmware report contains:
// * the build date, packed into a bitmap
// * the firmware version, packed into a bitmap
const FW_REP_LEN: usize = size_of_val(&BUILD_DATE) + size_of_val(&CONFIGURATION.version);

// Check the reports are less than 64 bytes (max size for usb full speed).
const _: () = assert!(FW_REP_LEN < 64);

// Allow this for now because clippy shows a non const suggestion.
#[allow(clippy::cast_possible_truncation)]
#[rustfmt::skip]
pub const REPORT_DESCRIPTOR: &[u8] = &[
	0x05, 0x01, // UsagePage (Generic Desktop)
	0x09, 0x06, // UsageId(Keyboard[6])
	0xA1, 0x01, // Collection(Application)

	// -- [Start with the basic keyboard report used for key presses] --

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

	// -- [Define the custom report used for configuration] --

	// --- Output Report ---
	0x85, VENDOR_REPORT_OUT_ID, // ReportId()
	0x06, 0x00, 0xFF,           // UsagePage(VendorDefined)
	0x09, 0x01,                 // UsageId(VendorDefined 1)
	0x15, 0x00,                 // LogicalMinimum(0)
	0x26, 0x64, 0x00,           // LogicalMaximum(100)
	0x75, 0x08,                 // ReportSize(8)
	0x95, 0x01,                 // ReportCount(1)
	0x91, 0x02,                 // Output(Data, Variable, Absolute)

	// --- Firmware input report ---
	0x85, VENDOR_REPORT_IN_ID, // ReportId()
	0x06, 0x00, 0xFF,          // UsagePage(VendorDefined)
	0x09, 0x02,                // UsageId(VendorDefined 2)
	0x15, 0x00,                // LogicalMinimum(0)
	0x26, 0x64, 0x00,          // LogicalMaximum(100)
	0x75, 0x08,                // ReportSize(8)
	0x95, FW_REP_LEN as u8,    // ReportCount()
	0x81, 0x00,                // Input(Data, Array, Absolute)

	0xC0, // EndCollection()
];

#[cfg(feature = "serial")]
pub fn process_vendor_report(hid_class: &HIDClass<UsbBus>, req_byte: u8, serial: &mut SerialPort<UsbBus>) {
	internal_process_vendor_report(hid_class, req_byte);

	_ = write_serial_message(serial, b"got some vendor output report data!!\n".as_ref());
}

#[cfg(not(feature = "serial"))]
pub fn process_vendor_report(hid_class: &HIDClass<UsbBus>, req_byte: u8) {
	internal_process_vendor_report(hid_class, req_byte);
}

#[inline]
fn internal_process_vendor_report(hid_class: &HIDClass<UsbBus>, req_byte: u8) {
	match req_byte {
		REQ_GET_FIRMWARE_INFO => {
			const BUILD_DATE_BYTES: [u8; 2] = BUILD_DATE.to_le_bytes();
			const VERSION_BYTES: [u8; 4] = CONFIGURATION.version.to_le_bytes();

			const RESPONSE: [u8; FW_REP_LEN + 1] = [
				VENDOR_REPORT_IN_ID,
				BUILD_DATE_BYTES[0],
				BUILD_DATE_BYTES[1],
				VERSION_BYTES[0],
				VERSION_BYTES[1],
				VERSION_BYTES[2],
				VERSION_BYTES[3],
			];

			_ = hid_class.push_raw_input(&RESPONSE).is_ok();
		}
		_ => {}
	}
}
