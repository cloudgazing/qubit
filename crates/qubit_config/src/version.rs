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

	#[must_use]
	pub const fn new_zero(major: u16, minor: u16, patch: u16) -> Self {
		Self {
			api: Api::V0,
			major,
			minor,
			patch,
		}
	}

	/// The version represented as a bitmap.
	///
	/// # Layout
	///
	/// * API - 2 bits (0-1)
	/// * Major - 10 bits (2-11)
	/// * Minor - 10 bits (12-21)
	/// * Patch - 10 bits (22-31)
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
