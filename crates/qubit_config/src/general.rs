#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "std", serde(rename_all = "lowercase"))]
pub enum Device {
	Keyboard,
}

impl Device {
	#[must_use]
	pub const fn region_name(self) -> &'static str {
		match self {
			Self::Keyboard => "KEYBOARD",
		}
	}

	#[must_use]
	pub const fn usb_class(self) -> u8 {
		match self {
			Self::Keyboard => 0x00,
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
