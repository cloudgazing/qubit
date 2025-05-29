//! Keyboard definition and related functions.

use rp2040_hal as hal;

// Column
type C0 = hal::gpio::bank0::Gpio0;
type C1 = hal::gpio::bank0::Gpio1;
type C2 = hal::gpio::bank0::Gpio2;
type C3 = hal::gpio::bank0::Gpio3;
type C4 = hal::gpio::bank0::Gpio4;
type C5 = hal::gpio::bank0::Gpio5;
type C6 = hal::gpio::bank0::Gpio6;
type C7 = hal::gpio::bank0::Gpio7;
type C8 = hal::gpio::bank0::Gpio8;
type C9 = hal::gpio::bank0::Gpio9;
type C10 = hal::gpio::bank0::Gpio10;
type C11 = hal::gpio::bank0::Gpio11;
type C12 = hal::gpio::bank0::Gpio12;
type C13 = hal::gpio::bank0::Gpio13;
// Row
type R0 = hal::gpio::bank0::Gpio16;
type R1 = hal::gpio::bank0::Gpio17;
type R2 = hal::gpio::bank0::Gpio18;
type R3 = hal::gpio::bank0::Gpio19;
type R4 = hal::gpio::bank0::Gpio20;

#[qubit_macros::kb_pin_matrix]
#[kb_pin_matrix(env(layout = "CONFIG_LAYOUT_KEYMAP"))]
pub struct KeyboardMatrix {
	rows: (R0, R1, R2, R3, R4),
	cols: (C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13),
}
