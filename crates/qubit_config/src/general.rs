use core::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "std", serde(rename_all = "lowercase"))]
pub enum Device {
	Keyboard,
}

impl Device {
	#[must_use]
	pub fn region_name(self) -> &'static str {
		match self {
			Self::Keyboard => "KEYBOARD",
		}
	}
}

#[derive(Debug)]
pub enum Api {
	V0,
	V1,
	V2,
	V3,
}

impl Api {
	#[must_use]
	pub const fn as_u8(&self) -> u8 {
		match self {
			Self::V0 => 0,
			Self::V1 => 1,
			Self::V2 => 2,
			Self::V3 => 3,
		}
	}
}

#[derive(Debug)]
pub struct Version {
	pub api: Api,
	pub major: u16,
	pub minor: u16,
	pub patch: u16,
}

impl Version {
	#[must_use]
	pub const fn new(api: Api, major: u16, minor: u16, patch: u16) -> Self {
		Self {
			api,
			major,
			minor,
			patch,
		}
	}

	/// The bitmap representation of the version.
	///
	/// # Layout
	///
	/// * API version - 2 bits (0-1)
	/// * Major version - 10 bits (2-11)
	/// * Minor version - 10 bits (12-21)
	/// * Patch version - 10 bits (22-31)
	#[must_use]
	pub const fn as_bitmap(&self) -> u32 {
		let api_v = self.api.as_u8() as u32;
		let major = self.major as u32;
		let minor = self.minor as u32;
		let patch = self.patch as u32;

		let mut bitmap: u32 = 0;
		bitmap |= api_v & 0x03;
		bitmap |= (major & 0x3FF) << 2;
		bitmap |= (minor & 0x3FF) << 11;
		bitmap |= (patch & 0x3FF) << 22;

		bitmap
	}
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub enum Processor {
	RP2040,
	STM32F411,
}

impl FromStr for Processor {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"RP2040" => Ok(Self::RP2040),
			"STM32F411" => Ok(Self::STM32F411),
			_ => Err(()),
		}
	}
}

impl Processor {
	#[must_use]
	pub const fn as_str(&self) -> &'static str {
		match self {
			Self::RP2040 => "RP2040",
			Self::STM32F411 => "STM32F411",
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
pub struct Configuration {
	/// Tha name of the board.
	pub name: &'static str,
	/// The board author or manufacturer.
	pub author: &'static str,
	/// A [`Version`] represented as a bitmap.
	pub version: u32,

	pub device: Device,
}
