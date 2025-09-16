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
	(
		$(
			$bank:ident : [ $( $num:literal ),* ]
		),* $(,)?
	) => {
		::paste::paste! {
			#[derive(Debug, Eq, Hash, PartialEq)]
			pub enum Pin {
				$(
					$(
						[<P $bank $num>],
					)*
				)*
			}

			impl $crate::mcu::McuPin for Pin {
				fn pin_str(&self) -> &'static str {
					match self {
						$(
							$(
								Self::[<P $bank $num>] => concat!(stringify!($bank), stringify!($num)),
							)*
						)*
					}
				}
			}
		}
	};
}

define_pins! {
	A: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	B: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	C: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	D: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	E: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	H: [0, 1],
}

#[derive(Debug)]
pub struct Stm32f411;

impl Stm32f411 {
	/// # Errors
	///
	/// This function returns the error from the [`std::io::Write::write_all`] method.
	#[cfg(feature = "build")]
	pub fn linker_layout(mem_x_file: &mut File, flash_len: u32, qubit_len: u32) -> Result<(), std::io::Error> {
		// https://docs.rs/cortex-m-rt/latest/cortex_m_rt

		const FLASH_ORIGIN: u32 = 0x0800_0000;

		const RAM_ORIGIN: u32 = 0x2000_0000;
		const RAM_LENGTH: u32 = 0x20000;

		let remaining_flash = flash_len.strict_sub(qubit_len);

		let qubit_origin = FLASH_ORIGIN + remaining_flash;

		let contents = indoc::formatdoc! {"
			MEMORY {{
				FLASH (rx) : ORIGIN = {FLASH_ORIGIN}, LENGTH = {remaining_flash}
				QUBIT (r) : ORIGIN = {qubit_origin}, LENGTH = {qubit_len}
				RAM (rw) : ORIGIN = {RAM_ORIGIN}, LENGTH = {RAM_LENGTH}
			}}

			SECTIONS {{
				.qubit ORIGIN(QUBIT) :
				{{
					KEEP(*(.qubit.*))
				}} > QUBIT
			}} INSERT BEFORE .text;

			_stack_start = ORIGIN(RAM) + LENGTH(RAM);
		"};

		mem_x_file.write_all(contents.as_bytes())
	}
}

impl McuSpec for Stm32f411 {
	type Pin = Pin;

	const STR: &str = "STM32F411";
	const CFG_STR: &str = "stm32f411";
	const TARGET_TRIPLE: &str = "thumbv7em-none-eabihf";

	#[cfg(feature = "build")]
	fn enable_cfgs(collected_pins: &HashSet<&Self::Pin>, build_cfgs: &mut BuildCfgs) {
		use super::McuPin as _;

		build_cfgs.check_cfgs(&[
			"stm32f411_bank_b",
			"stm32f411_bank_c",
			"stm32f411_bank_d",
			"stm32f411_bank_e",
			"stm32f411_bank_h",
		]);

		let bank_enabled = collected_pins.iter().any(|pin| pin.pin_str().starts_with('B'));
		build_cfgs.if_enable_cfg("stm32f411_bank_b", bank_enabled);

		let bank_enabled = collected_pins.iter().any(|pin| pin.pin_str().starts_with('C'));
		build_cfgs.if_enable_cfg("stm32f411_bank_c", bank_enabled);

		let bank_enabled = collected_pins.iter().any(|pin| pin.pin_str().starts_with('D'));
		build_cfgs.if_enable_cfg("stm32f411_bank_d", bank_enabled);

		let bank_enabled = collected_pins.iter().any(|pin| pin.pin_str().starts_with('E'));
		build_cfgs.if_enable_cfg("stm32f411_bank_e", bank_enabled);

		let bank_enabled = collected_pins.iter().any(|pin| pin.pin_str().starts_with('H'));
		build_cfgs.if_enable_cfg("stm32f411_bank_h", bank_enabled);
	}
}
