pub const KB_REP_ID_IN: u8 = 0x01;
pub const KB_REP_ID_OUT: u8 = 0x02;

// https://usb.org/document-library/hid-usage-tables-16
// https://learn.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages

const REPORT_HEADER: &[u8] = &[
	0x05, 0x01, // UsagePage (Generic Desktop)
	0x09, 0x06, // UsageId(Keyboard[6])
	0xA1, 0x01, // Collection(Application)
];

const VENDOR_REPORT: &[u8] = cfg_select! {
	feature = "silverplate" => { super::silverplate::REPORT }
	_ => { &[] }
};

#[rustfmt::skip]
pub const DESCRIPTOR_6KRO: &[u8] = constcat::concat_slices!([u8]:
	REPORT_HEADER,

	// Basic keyboard report
	&[
		0x85, KB_REP_ID_IN,          // ReportId()
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
		0x85, KB_REP_ID_OUT,         // ReportId(2)
		// --- LEDs Byte (1 byte) ---
		0x05, 0x08,                  // UsagePage(LEDs)
		0x19, 0x01,                  // UsageMinimum(Num Lock)
		0x29, 0x05,                  // UsageMaximum(Kana)
		0x15, 0x00,                  // LogicalMinimum(0)
		0x25, 0x01,                  // LogicalMaximum(1)
		0x75, 0x01,                  // ReportSize(1)
		0x95, 0x05,                  // ReportCount(5)
		0x91, 0x02,                  // Output(Data, Variable, Absolute)
		// --- Remaining bits for padding ---
		0x75, 0x03,                  // ReportSize(3)
		0x95, 0x01,                  // ReportCount(1)
		0x91, 0x01,                  // Output(Constant)
	],

	VENDOR_REPORT,

	&[0xC0] // EndCollection()
);

#[rustfmt::skip]
pub const DESCRIPTOR_NKRO: &[u8] = constcat::concat_slices!([u8]:
	REPORT_HEADER,

	// Basic keyboard report
	&[
		0x85, KB_REP_ID_IN,          // ReportId()
		// --- Modifier Keys (1 byte) ---
		0x05, 0x07,                  // UsagePage(Keyboard/Keypad)
		0x19, 0xE0,                  // UsageMinimum(Keyboard LeftControl)
		0x29, 0xE7,                  // UsageMaximum(Keyboard RightGUI)
		0x15, 0x00,                  // LogicalMinimum(0)
		0x25, 0x01,                  // LogicalMaximum(1)
		0x75, 0x01,                  // ReportSize(1)
		0x95, 0x08,                  // ReportCount(8)
		0x81, 0x02,                  // Input(Data, Variable, Absolute)
		// --- NKRO Bitmap (32 bytes) ---
		0x05, 0x07,                  // UsagePage(Keyboard/Keypad)
		0x19, 0x00,                  // UsageMinimum(0x00)
		0x29, 0xFF,                  // UsageMaximum(0xFF)
		0x15, 0x00,                  // LogicalMinimum(0x00)
		0x25, 0x01,                  // LogicalMaximum(0x01)
		0x75, 0x01,                  // ReportSize(1)
		0x96, 0x00, 0x01,            // ReportCount(256)
		0x81, 0x02,                  // Input(Data, Variable, Absolute)
		// --- Output Report for LEDs ---
		0x85, KB_REP_ID_OUT,         // ReportId(2)
		// --- LEDs Byte (1 byte) ---
		0x05, 0x08,                  // UsagePage(LEDs)
		0x19, 0x01,                  // UsageMinimum(Num Lock)
		0x29, 0x05,                  // UsageMaximum(Kana)
		0x15, 0x00,                  // LogicalMinimum(0)
		0x25, 0x01,                  // LogicalMaximum(1)
		0x75, 0x01,                  // ReportSize(1)
		0x95, 0x05,                  // ReportCount(5)
		0x91, 0x02,                  // Output(Data, Variable, Absolute)
		// --- Remaining bits for padding ---
		0x75, 0x03,                  // ReportSize(3)
		0x95, 0x01,                  // ReportCount(1)
		0x91, 0x01,                  // Output(Constant)
	],

	VENDOR_REPORT,

	&[0xC0] // EndCollection()
);
