use core::mem::size_of_val;

use usbd_hid::hid_class::HIDClass;

use super::CONFIG;
use crate::DEVICE_CONFIG;
use crate::setup::UsbBus;

const BUILD_DATE: u16 = qubit_macros::build_date_bitmap!();

pub const VEND_REP_ID_OUT: u8 = 0x03;
pub const VEND_REP_ID_IN: u8 = 0x04;

// Vendor reports coming from the host
const REQ_GET_FIRMWARE_INFO: u8 = 0x01;

// Input reports must begin with the report ID, followed by the payload.
// Although the descriptor defines the report size as the payload size, the actual data passed
// to the push function must include the report ID, making the total size payload_size + 1.

// The firmware report contains:
// * the build date, packed into a bitmap
// * the firmware version, packed into a bitmap
const FW_REP_LEN: u8 = {
	let size = size_of_val(&BUILD_DATE) + size_of_val(&DEVICE_CONFIG.version);

	assert!(size < 64);

	#[allow(
		clippy::cast_possible_truncation,
		reason = "Clippy suggestion does not work in const contexts.
		The assertion above also guarantees the value will not be truncated."
	)]
	{
		size as u8
	}
};

#[rustfmt::skip]
pub const REPORT: &[u8] = &[
	0x85, VEND_REP_ID_OUT,     // ReportId()
	0x06, 0x00, 0xFF,          // UsagePage(VendorDefined)
	0x09, 0x01,                // UsageId(VendorDefined 1)
	0x15, 0x00,                // LogicalMinimum(0)
	0x26, 0x64, 0x00,          // LogicalMaximum(100)
	0x75, 0x08,                // ReportSize(8)
	0x95, 0x01,                // ReportCount(1)
	0x91, 0x02,                // Output(Data, Variable, Absolute)

	0x85, VEND_REP_ID_IN,      // ReportId()
	0x06, 0x00, 0xFF,          // UsagePage(VendorDefined)
	0x09, 0x02,                // UsageId(VendorDefined 2)
	0x15, 0x00,                // LogicalMinimum(0)
	0x26, 0x64, 0x00,          // LogicalMaximum(100)
	0x75, 0x08,                // ReportSize(8)
	0x95, FW_REP_LEN,          // ReportCount()
	0x81, 0x00,                // Input(Data, Array, Absolute)
];

pub fn process_vendor_report(hid_class: &HIDClass<UsbBus>, req_byte: u8) {
	// check the "command byte"
	match req_byte {
		REQ_GET_FIRMWARE_INFO => {
			const BUILD_DATE_BYTES: [u8; 2] = BUILD_DATE.to_le_bytes();
			const VERSION_BYTES: [u8; 4] = DEVICE_CONFIG.version.to_le_bytes();

			const RESPONSE: [u8; (FW_REP_LEN + 1) as usize] = [
				VEND_REP_ID_IN,
				BUILD_DATE_BYTES[0],
				BUILD_DATE_BYTES[1],
				VERSION_BYTES[0],
				VERSION_BYTES[1],
				VERSION_BYTES[2],
				VERSION_BYTES[3],
			];

			_ = hid_class.push_raw_input(&RESPONSE).is_ok();
		}
		0x02 => {
			// req info

			{
				let keymap = &CONFIG.keymaps.keymap_0;

				let size = keymap.len() as u64;
				let size: [u8; 8] = size.to_le_bytes();

				let response: [u8; 10 + 1] = [
					0x05, // report id
					0x00, // number of rows
					0x00, // number of cols
					size[0], size[1], size[2], size[3], size[4], size[5], size[6], size[7],
				];

				_ = hid_class.push_raw_input(&response).is_ok();
			}

			// {
			// 	let keymap = unsafe { temp_get_active_keymap_ref() };

			// 	let chunk_size = 59;

			// 	let chunk_count = keymap.len().div_ceil(chunk_size);

			// 	for index in 0..chunk_count {
			// 		let start = index * chunk_size;
			// 		let end = (start + chunk_size).min(keymap.len());

			// 		let mut response: [u8; 61] = [0; 61];
			// 		response[0] = 0x06; // reportID
			// 		response[1] = index as u8 + 1; // chunk index

			// 		let data_slice = &keymap[start..end];

			// 		response[2..(2 + data_slice.len())].copy_from_slice(data_slice);

			// 		_ = hid_class.push_raw_input(&response).is_ok();
			// 	}
			// }
		}
		_ => {}
	}
}
