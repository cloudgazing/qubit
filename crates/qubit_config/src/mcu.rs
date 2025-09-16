use core::fmt::Debug;
use core::hash::Hash;
use core::str::FromStr;
#[cfg(feature = "build")]
use std::collections::HashSet;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "build")]
use crate::cargo::BuildCfgs;

pub mod rp2040;
pub mod stm32f411;

pub use rp2040::Rp2040;
pub use stm32f411::Stm32f411;

#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, Copy)]
pub enum Mcu {
	RP2040,
	STM32F411,
}

impl Mcu {
	#[must_use]
	pub const fn as_str(&self) -> &'static str {
		match self {
			Self::RP2040 => "RP2040",
			Self::STM32F411 => "STM32F411",
		}
	}

	#[must_use]
	pub const fn as_cfg_str(&self) -> &'static str {
		match self {
			Self::RP2040 => "rp2040",
			Self::STM32F411 => "stm32f411",
		}
	}

	#[must_use]
	pub const fn target_triple(&self) -> &'static str {
		match self {
			Self::RP2040 => "thumbv6m-none-eabi",
			Self::STM32F411 => "thumbv7em-none-eabihf",
		}
	}
}

#[derive(Debug)]
pub enum ParseMcuError {
	InvalidMcu,
}

impl FromStr for Mcu {
	type Err = ParseMcuError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"RP2040" => Ok(Self::RP2040),
			"STM32F411" => Ok(Self::STM32F411),
			_ => Err(Self::Err::InvalidMcu),
		}
	}
}

////////////////

mod private {
	pub trait Private {}

	impl Private for super::rp2040::Rp2040 {}
	impl Private for super::stm32f411::Stm32f411 {}
}

pub trait McuPin: Debug + Eq + Hash {
	fn pin_str(&self) -> &'static str;
}

pub trait McuSpec: private::Private {
	type Pin: McuPin;

	const STR: &str;
	const CFG_STR: &str;
	const TARGET_TRIPLE: &str;

	#[cfg(feature = "build")]
	fn enable_cfgs(collected_pins: &HashSet<&Self::Pin>, build_cfgs: &mut BuildCfgs);
}

pub type Pin<T> = <T as McuSpec>::Pin;
