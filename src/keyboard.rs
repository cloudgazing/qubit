use rp2040_hal as hal;

use hal::gpio::bank0;
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage};

// pub mod key;

// Column
type C0 = bank0::Gpio0;
type C1 = bank0::Gpio1;
type C2 = bank0::Gpio2;
type C3 = bank0::Gpio3;
type C4 = bank0::Gpio4;
type C5 = bank0::Gpio5;
type C6 = bank0::Gpio6;
type C7 = bank0::Gpio7;
type C8 = bank0::Gpio8;
type C9 = bank0::Gpio9;
type C10 = bank0::Gpio10;
type C11 = bank0::Gpio11;
type C12 = bank0::Gpio12;
type C13 = bank0::Gpio13;

// Row
type R0 = bank0::Gpio16;
type R1 = bank0::Gpio17;
type R2 = bank0::Gpio18;
type R3 = bank0::Gpio19;
type R4 = bank0::Gpio20;

kb_rs_derive::define_keyboard_matrix!(
	[C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13],
	[R0, R1, R2, R3, R4]
);

fn get_key_report() -> KeyboardReport {
	let first_letter = KeyboardUsage::KeyboardQq as u8;

	KeyboardReport {
		modifier: 0x00,
		reserved: 0,
		leds: 0,
		keycodes: [first_letter, 0x00, 0x00, 0x00, 0x00, 0x00],
	}
}

#[must_use]
pub fn get_end_keyboard_report() -> KeyboardReport {
	KeyboardReport {
		modifier: 0x00,
		reserved: 0,
		leds: 0,
		keycodes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
	}
}
