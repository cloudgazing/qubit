#[cfg(feature = "std")]
pub mod parse;

pub type Keymap<const S: usize> = [u8; S];

#[derive(Debug)]
pub enum ApiVersion {
	V0,
	V1,
	V2,
	V3,
}

impl ApiVersion {
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
pub struct SemVer {
	pub api: ApiVersion,
	pub major: u16,
	pub minor: u16,
	pub patch: u16,
}

impl SemVer {
	#[must_use]
	pub fn new(api: ApiVersion, major: u16, minor: u16, patch: u16) -> Self {
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

#[derive(Debug)]
pub struct Configuration<const S: usize> {
	/// Tha name of the board.
	pub name: &'static str,
	/// The board author or manufacturer.
	pub author: &'static str,
	/// The id diferentates the board from the others from the same author.
	pub id: &'static str,
	/// A [`SemVer`] represented as a bitmap.
	pub version: u32,
	pub keymap: Keymap<S>,
}
