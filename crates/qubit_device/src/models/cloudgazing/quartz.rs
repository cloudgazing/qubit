use qubit_config::general::{Api, Device, Processor, Version};
use qubit_config::keyboard::Keymap;
use qubit_config::keymap_row;

pub const NAME: &str = "Quartz";

// Firmware
pub const AUTHOR: &str = "cloudgazing";
pub const VERSION: Version = Version::new(Api::V0, 0, 0, 1);
pub const DEVICE: Device = Device::Keyboard;
pub const LED_PIN: Option<usize> = Some(25);

// Keyboard
pub const PROCESSOR: Processor = Processor::RP2040;
pub const FLASH: u32 = 0x0080_0000;

// Keyboard keymap
pub const ROW_NUM: usize = 5;
pub const COL_NUM: usize = 14;
pub const ROW_PINS: [usize; ROW_NUM] = [16, 17, 18, 19, 20];
pub const COL_PINS: [usize; COL_NUM] = [0, 1, 2, 3, 26, 27, 6, 7, 8, 9, 10, 11, 12, 13];

// Mac keymap
#[rustfmt::skip]
pub const LAYER0: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE),
	keymap_row!(KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH),
	keymap_row!(KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER),
	keymap_row!(KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT),
	keymap_row!(KC_LEFTCTRL, KC_LEFTALT, KC_LEFTMETA, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTMETA, KC_RIGHTALT, KC_RIGHTCTRL),
]);

// Win keymap
#[rustfmt::skip]
pub const LAYER1: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE),
	keymap_row!(KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH),
	keymap_row!(KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER),
	keymap_row!(KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT),
	keymap_row!(KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL),
]);

//
#[rustfmt::skip]
pub const LAYER2: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE),
	keymap_row!(KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH),
	keymap_row!(KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER),
	keymap_row!(KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT),
	keymap_row!(KC_LEFTCTRL, KC_LEFTALT, KC_LEFTMETA, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTMETA, KC_RIGHTALT, KC_RIGHTCTRL),
]);
#[rustfmt::skip]
pub const LAYER3: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE),
	keymap_row!(KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH),
	keymap_row!(KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER),
	keymap_row!(KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT),
	keymap_row!(KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL),
]);
#[rustfmt::skip]
pub const LAYER4: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINUS, KC_EQUAL, KC_BACKSPACE),
	keymap_row!(KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LEFTBRACE, KC_RIGHTBRACE, KC_BACKSLASH),
	keymap_row!(KC_CAPSLOCK, KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SEMICOLON, KC_APOSTROPHE, -, KC_ENTER),
	keymap_row!(KC_LEFTSHIFT, -, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMMA, KC_DOT, KC_SLASH, -, KC_RIGHTSHIFT),
	keymap_row!(KC_LEFTCTRL, KC_LEFTMETA, KC_LEFTALT, -, -, -, KC_SPACE, -, -, -, -, KC_RIGHTALT, KC_RIGHTMETA, KC_RIGHTCTRL),
]);

// Keyboard layout

// USB
pub const USB_VID: u16 = 0x1209;
pub const USB_PID: u16 = 0x0001;

// Hardware
