use std::collections::HashSet;

use crate::mcu::{McuPin, McuSpec};

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

	pub fn check_and_enable_cfg(&mut self, cfg: &str) {
		self.check_cfg(cfg);
		self.enable_cfg(cfg);
	}

	pub fn check_keyboard_mcu_cfg(&mut self) {
		use crate::mcu::{Rp2040, Stm32f411};

		let mcus = [Rp2040::CFG_STR, Stm32f411::CFG_STR];

		let mcu_values = mcus.map(|mcu| format!("\"{mcu}\"")).join(", ");

		self.check_cfg(&format!("mcu, values({mcu_values})"));
	}

	pub fn check_and_enable_keyboard_cfgs<M: McuSpec>(&mut self, collected_pins: &HashSet<&M::Pin>) {
		use crate::mcu::{Rp2040, Stm32f411};

		self.check_cfg("keyboard");
		self.enable_cfg("keyboard");

		let mcus = [Rp2040::CFG_STR, Stm32f411::CFG_STR];

		let mcu_values = mcus.map(|mcu| format!("\"{mcu}\"")).join(", ");

		self.check_cfg(&format!("mcu, values({mcu_values})"));

		M::enable_cfgs(collected_pins, self);
	}
}

#[derive(Debug)]
pub enum ErrReason {
	Duplicate,
	Reserved,
}

#[derive(Debug)]
pub struct PinCollectError<'a, P: McuPin> {
	pub reason: ErrReason,
	pub pin: &'a P,
}

impl<'a, P: McuPin> PinCollectError<'a, P> {
	#[must_use]
	pub fn reserved(pin: &'a P) -> Self {
		Self {
			reason: ErrReason::Reserved,
			pin,
		}
	}

	#[must_use]
	pub fn duplicate(pin: &'a P) -> Self {
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
/// Returns an error if a pin is used more than once or is already reserved.
pub fn collect_pins<'a, P: McuPin, const R: usize, const C: usize>(
	row: &'a [P; R],
	col: &'a [P; C],
	led: &'a Option<P>,
) -> Result<HashSet<&'a P>, PinCollectError<'a, P>> {
	let mut pins = HashSet::new();

	for pin in row {
		if !pins.insert(pin) {
			return Err(PinCollectError::duplicate(pin));
		}
	}

	for pin in col {
		if !pins.insert(pin) {
			return Err(PinCollectError::duplicate(pin));
		}
	}

	if let Some(pin) = led
		&& !pins.insert(pin)
	{
		return Err(PinCollectError::duplicate(pin));
	}

	Ok(pins)
}
