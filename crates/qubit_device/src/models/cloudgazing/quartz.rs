use qubit_config::general::Device;
use qubit_config::keymaps;
use qubit_config::mcu;
use qubit_config::usb::Usb;
use qubit_config::version::Version;

pub type Mcu = mcu::Rp2040;
type Pin = mcu::Pin<Mcu>;

pub const NAME: &str = "Quartz";
pub const AUTHOR: &str = "cloudgazing";
pub const VERSION: Version = Version::api_zero(0, 0, 1);
pub const DEVICE: Device = Device::Keyboard;

// This VID/PID is provided by pid.codes and is reserved for testing.
// https://pid.codes/1209/0001/
pub const USB: Usb = Usb::new(0x1209, 0x0001);

pub const FLASH: u32 = 0x0080_0000;

pub const LED_PIN: Option<Pin> = Some(Pin::P25);

// Keyboard
pub const ROW_NUM: usize = 5;
pub const COL_NUM: usize = 14;
pub const ROW_PINS: [Pin; ROW_NUM] = [Pin::P16, Pin::P17, Pin::P18, Pin::P19, Pin::P20];
pub const COL_PINS: [Pin; COL_NUM] = [
	Pin::P0,
	Pin::P1,
	Pin::P2,
	Pin::P3,
	Pin::P26,
	Pin::P27,
	Pin::P6,
	Pin::P7,
	Pin::P8,
	Pin::P9,
	Pin::P10,
	Pin::P11,
	Pin::P12,
	Pin::P13,
];

keymaps! {
	pub BOOL_MATRIX, pub KEYMAPS : ROW_NUM, COL_NUM :
	0 : [
		[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
		[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
		[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
		[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
		[KC_LEFTCTRL, KC_LEFTALT, KC_LEFTMETA, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTMETA, KC_RIGHTALT, KC_RIGHTCTRL],
	],
	1: [
		[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
		[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
		[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
		[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
		[KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL],
	],
	2 : [
		[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
		[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
		[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
		[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
		[KC_LEFTCTRL, KC_LEFTALT, KC_LEFTMETA, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTMETA, KC_RIGHTALT, KC_RIGHTCTRL],
	],
	3 : [
		[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
		[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
		[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
		[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
		[KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL],
	],
	4 : [
		[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
		[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
		[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
		[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
		[KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL],
	],
}
