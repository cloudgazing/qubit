use std::collections::HashSet;

use crate::mcu::Mcu;

#[derive(Debug, Default)]
pub struct BuildCfgs {
	enabled: HashSet<String>,
	checked: HashSet<String>,
}

impl BuildCfgs {
	#[must_use]
	pub fn new() -> Self {
		Self {
			enabled: HashSet::new(),
			checked: HashSet::new(),
		}
	}

	pub fn if_enable_cfg(&mut self, cfg: &str, is_enabled: bool) {
		if is_enabled && self.enabled.insert(cfg.to_string()) {
			println!("cargo::rustc-cfg={cfg}");
		}
	}

	pub fn enable_cfg(&mut self, cfg: &str) {
		if self.enabled.insert(cfg.to_string()) {
			println!("cargo::rustc-cfg={cfg}");
		}
	}

	pub fn enable_cfgs(&mut self, cfgs: &[&str]) {
		for cfg in cfgs {
			self.enable_cfg(cfg);
		}
	}

	pub fn check_cfg(&mut self, cfg: &str) {
		if self.checked.insert(cfg.to_string()) {
			println!("cargo::rustc-check-cfg=cfg({cfg})");
		}
	}

	pub fn check_cfgs(&mut self, cfgs: &[&str]) {
		for cfg in cfgs {
			self.check_cfg(cfg);
		}
	}

	pub fn check_keyboard_mcu_cfg(&mut self) {
		let mcus = ["rp2040", "stm32f411"];

		let values = mcus.map(|mcu| format!("\"{mcu}\"")).join(", ");

		self.check_cfg(&format!("mcu, values({values})"));
	}
}

/// # Panics
///
/// Panics if a pin is used more than once.
pub fn output_cargo_instructions<const R: usize, const C: usize>(
	mcu: Mcu,
	row: &[&str; R],
	col: &[&str; C],
	led: Option<&str>,
	build_cfgs: &mut BuildCfgs,
) {
	match mcu {
		Mcu::RP2040 => {}
		Mcu::STM32F411 => {
			let pins = collect_pins(row, col, led).unwrap();

			let bank_enabled = pins.iter().any(|pin| pin.starts_with('B'));
			build_cfgs.if_enable_cfg("stm32f411_bank_b", bank_enabled);

			let bank_enabled = pins.iter().any(|pin| pin.starts_with('C'));
			build_cfgs.if_enable_cfg("stm32f411_bank_c", bank_enabled);

			let bank_enabled = pins.iter().any(|pin| pin.starts_with('D'));
			build_cfgs.if_enable_cfg("stm32f411_bank_d", bank_enabled);

			let bank_enabled = pins.iter().any(|pin| pin.starts_with('E'));
			build_cfgs.if_enable_cfg("stm32f411_bank_e", bank_enabled);

			let bank_enabled = pins.iter().any(|pin| pin.starts_with('H'));
			build_cfgs.if_enable_cfg("stm32f411_bank_h", bank_enabled);

			build_cfgs.check_cfgs(&[
				"stm32f411_bank_b",
				"stm32f411_bank_c",
				"stm32f411_bank_d",
				"stm32f411_bank_e",
				"stm32f411_bank_h",
			]);
		}
	}
}

///////////

#[derive(Debug)]
pub enum ErrReason {
	Duplicate,
	Reserved,
}

#[derive(Debug)]
pub struct PinCollectError<'a> {
	pub reason: ErrReason,
	pub pin: &'a str,
}

impl<'a> PinCollectError<'a> {
	#[must_use]
	pub fn reserved(pin: &'a str) -> Self {
		Self {
			reason: ErrReason::Reserved,
			pin,
		}
	}

	#[must_use]
	pub fn duplicate(pin: &'a str) -> Self {
		Self {
			reason: ErrReason::Duplicate,
			pin,
		}
	}
}

/// Collect all pins used.
///
/// # Errors
///
/// Returns an error if a pin is used more than once.
pub fn collect_pins<'a, const R: usize, const C: usize>(
	row: &[&'a str; R],
	col: &[&'a str; C],
	led: Option<&'a str>,
) -> Result<HashSet<&'a str>, PinCollectError<'a>> {
	let mut pins = HashSet::new();

	for p in row {
		let is_new = pins.insert(*p);

		if !is_new {
			return Err(PinCollectError::duplicate(p));
		}
	}

	for p in col {
		let is_new = pins.insert(*p);

		if !is_new {
			return Err(PinCollectError::duplicate(p));
		}
	}

	if let Some(p) = led {
		let is_new = pins.insert(p);

		if !is_new {
			return Err(PinCollectError::duplicate(p));
		}
	}

	Ok(pins)
}
