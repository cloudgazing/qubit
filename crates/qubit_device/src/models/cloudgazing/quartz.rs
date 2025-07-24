use qubit_config::general::Device;
use qubit_config::keyboard::Keymap;
use qubit_config::keymap;
use qubit_config::mcu::Mcu;
use qubit_config::usb::Usb;
use qubit_config::version::Version;

pub const NAME: &str = "Quartz";

// Firmware
pub const AUTHOR: &str = "cloudgazing";
pub const VERSION: Version = Version::new_zero(0, 0, 1);
pub const DEVICE: Device = Device::Keyboard;
pub const LED_PIN: Option<&str> = Some("25");

// Keyboard
pub const MCU: Mcu = Mcu::RP2040;
pub const FLASH: u32 = 0x0080_0000;

// Keyboard keymap
pub const ROW_NUM: usize = 5;
pub const COL_NUM: usize = 14;
pub const ROW_PINS: [&str; ROW_NUM] = ["16", "17", "18", "19", "20"];
pub const COL_PINS: [&str; COL_NUM] = [
	"0", "1", "2", "3", "26", "27", "6", "7", "8", "9", "10", "11", "12", "13",
];

// Mac keymap
#[rustfmt::skip]
pub const LAYER0: Keymap<ROW_NUM, COL_NUM> = keymap! [
	[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
	[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
	[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
	[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
	[KC_LEFTCTRL, KC_LEFTALT, KC_LEFTMETA, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTMETA, KC_RIGHTALT, KC_RIGHTCTRL],
];

// Win keymap
#[rustfmt::skip]
pub const LAYER1: Keymap<ROW_NUM, COL_NUM> = keymap! [
	[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
	[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
	[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
	[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
	[KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL],
];

//
#[rustfmt::skip]
pub const LAYER2: Keymap<ROW_NUM, COL_NUM> = keymap! [
	[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
	[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
	[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
	[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
	[KC_LEFTCTRL, KC_LEFTALT, KC_LEFTMETA, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTMETA, KC_RIGHTALT, KC_RIGHTCTRL],
];
#[rustfmt::skip]
pub const LAYER3: Keymap<ROW_NUM, COL_NUM> = keymap! [
	[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
	[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
	[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
	[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
	[KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL],
];
#[rustfmt::skip]
pub const LAYER4: Keymap<ROW_NUM, COL_NUM> = keymap! [
	[KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE],
	[KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH],
	[KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER],
	[KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT],
	[KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL],
];

// Keyboard layout

// This VID/PID is provided by pid.codes and is reserved for testing.
// https://pid.codes/1209/0001/
pub const USB: Usb = Usb::new(0x1209, 0x0001);

// Hardware
