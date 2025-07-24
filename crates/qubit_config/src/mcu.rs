use core::str::FromStr;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
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
