// This is for now just a test device to check and implement
// multi-target compilation.

use qubit_config::general::Device;
use qubit_config::keymaps;
use qubit_config::mcu;
use qubit_config::usb::Usb;
use qubit_config::version::Version;

pub type Mcu = mcu::Stm32f411;
type Pin = mcu::Pin<Mcu>;

pub const NAME: &str = "Obsidian";
pub const AUTHOR: &str = "cloudgazing";
pub const VERSION: Version = Version::api_zero(0, 0, 1);
pub const DEVICE: Device = Device::Keyboard;

// This VID/PID is provided by pid.codes and is reserved for testing.
// https://pid.codes/1209/0001/
pub const USB: Usb = Usb::new(0x1209, 0x0001);

pub const FLASH: u32 = 0x0008_0000;

pub const LED_PIN: Option<Pin> = None;

// Keyboard
pub const ROW_NUM: usize = 2;
pub const COL_NUM: usize = 2;
pub const ROW_PINS: [Pin; ROW_NUM] = [Pin::PB12, Pin::PB13];
pub const COL_PINS: [Pin; COL_NUM] = [Pin::PB14, Pin::PB15];

keymaps! {
	pub BOOL_MATRIX, pub KEYMAPS : ROW_NUM, COL_NUM :
	0 : [
		[KC_0, KC_1],
		[KC_2, KC_LAYER_2],
	],
	1: [
		[KC_ESC, KC_BACKSPACE],
		[KC_LEFTALT, KC_LEFTMETA],
	],
	2 : [
		[KC_1, KC_6],
		[KC_LAYER_3, KC_8],
	],
	3 : [
		[KC_A, KC_B],
		[KC_C, KC_D],
	],
	4 : [
		[KC_E, KC_F],
		[KC_G, KC_H],
	],
}
