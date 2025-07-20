// This is for now just a test device to check and implement
// multi-target compilation.

use qubit_config::general::{Api, Device, Processor, Version};
use qubit_config::keyboard::Keymap;
use qubit_config::keymap_row;

pub const NAME: &str = "Obsidian";

// Firmware
pub const AUTHOR: &str = "cloudgazing";
pub const VERSION: Version = Version::new(Api::V0, 0, 0, 1);
pub const DEVICE: Device = Device::Keyboard;
pub const LED_PIN: Option<&str> = None;

// Keyboard
pub const PROCESSOR: Processor = Processor::STM32F411;
pub const FLASH: u32 = 0x0008_0000;

// Keyboard keymap
pub const ROW_NUM: usize = 2;
pub const COL_NUM: usize = 2;
pub const ROW_PINS: [&str; ROW_NUM] = ["B12", "B13"];
pub const COL_PINS: [&str; COL_NUM] = ["B14", "B15"];

// Mac keymap
#[rustfmt::skip]
pub const LAYER0: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_0, KC_1),
	keymap_row!(KC_3, KC_4),
]);

// Win keymap
#[rustfmt::skip]
pub const LAYER1: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_ESC, KC_BACKSPACE),
	keymap_row!(KC_LEFTALT, KC_LEFTMETA),
]);

//
#[rustfmt::skip]
pub const LAYER2: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_0, KC_1),
	keymap_row!(KC_3, KC_4),
]);
#[rustfmt::skip]
pub const LAYER3: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_0, KC_1),
	keymap_row!(KC_3, KC_4),
]);
#[rustfmt::skip]
pub const LAYER4: Keymap<ROW_NUM, COL_NUM> = Keymap::new([
	keymap_row!(KC_0, KC_1),
	keymap_row!(KC_3, KC_4),
]);

// Keyboard layout

// USB
pub const USB_VID: u16 = 0x1209;
pub const USB_PID: u16 = 0x0001;

// Hardware
