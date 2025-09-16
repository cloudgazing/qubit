#[cfg(feature = "build")]
use std::collections::HashSet;
#[cfg(feature = "build")]
use std::fs::File;
#[cfg(feature = "build")]
use std::io::Write;

use super::McuSpec;
#[cfg(feature = "build")]
use crate::cargo::BuildCfgs;

macro_rules! define_pins {
	( $( $num:literal ),* ) => {
		::paste::paste! {
			#[derive(Debug, Eq, Hash, PartialEq)]
			pub enum Pin {
				$(
					[<P $num>],
				)*
			}

			impl $crate::mcu::McuPin for Pin {
				fn pin_str(&self) -> &'static str {
					match self {
						$(
							Self::[<P $num>] => stringify!($num),
						)*
					}
				}
			}
		}
	};
}

define_pins![
	0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29
];

#[derive(Debug)]
pub struct Rp2040;

impl Rp2040 {
	/// # Errors
	///
	/// This function returns the error from the [`std::io::Write::write_all`] method.
	#[cfg(feature = "build")]
	pub fn linker_layout(mem_x_file: &mut File, flash_len: u32, qubit_len: u32) -> Result<(), std::io::Error> {
		// https://docs.rs/cortex-m-rt/latest/cortex_m_rt

		const BOOT2_ORIGIN: u32 = 0x1000_0000;
		const BOOT2_LENGTH: u32 = 0x100;

		const RAM_ORIGIN: u32 = 0x2000_0000;
		const RAM_LENGTH: u32 = 0x42000;

		let flash_origin = BOOT2_ORIGIN + BOOT2_LENGTH;
		let remaining_flash = flash_len.strict_sub(BOOT2_LENGTH).strict_sub(qubit_len);

		let qubit_origin = flash_origin + remaining_flash;

		let contents = indoc::formatdoc! {"
			MEMORY {{
				BOOT2 (rx) : ORIGIN = {BOOT2_ORIGIN}, LENGTH = {BOOT2_LENGTH}
				FLASH (rx) : ORIGIN = {flash_origin}, LENGTH = {remaining_flash}
				QUBIT (r) : ORIGIN = {qubit_origin}, LENGTH = {qubit_len}
				RAM (rw) : ORIGIN = {RAM_ORIGIN}, LENGTH = {RAM_LENGTH}
			}}

			EXTERN(BOOT2_FIRMWARE)

			SECTIONS {{
				.boot2 ORIGIN(BOOT2) :
				{{
					KEEP(*(.boot2));
				}} > BOOT2

				.qubit ORIGIN(QUBIT) :
				{{
					KEEP(*(.qubit.*))
				}} > QUBIT
			}} INSERT BEFORE .text;
		"};

		mem_x_file.write_all(contents.as_bytes())
	}
}

impl McuSpec for Rp2040 {
	type Pin = Pin;

	const STR: &str = "RP2040";
	const CFG_STR: &str = "rp2040";
	const TARGET_TRIPLE: &str = "thumbv6m-none-eabi";

	#[cfg(feature = "build")]
	fn enable_cfgs(_collected_pins: &HashSet<&Self::Pin>, _build_cfgs: &mut BuildCfgs) {}
}
