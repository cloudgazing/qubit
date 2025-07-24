use serde::{Deserialize, Deserializer, de};

use crate::general::Device;

mod raw {
	use semver::Version;
	use serde::{Deserialize, Serialize};

	use crate::general::Device;
	use crate::mcu::Mcu;

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Firmware {
		pub author: String,
		pub id: String,
		pub version: Version,
		pub device: Device,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Keymap {
		pub rows: usize,
		pub cols: usize,
		pub row_pins: Vec<u8>,
		pub col_pins: Vec<u8>,
		pub layer0: Vec<Vec<u8>>,
		pub layer1: Vec<Vec<u8>>,
		pub layer2: Option<Vec<Vec<u8>>>,
		pub layer3: Option<Vec<Vec<u8>>>,
		pub layer4: Option<Vec<Vec<u8>>>,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Keyboard {
		pub mcu: Mcu,
		pub keymap: Keymap,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct RawConfig {
		pub name: String,
		pub firmware: Firmware,
		pub keyboard: Keyboard,
		pub pins: Vec<usize>,
	}
}

mod keyboard {
	use crate::mcu::Mcu;

	#[derive(Debug)]
	pub struct Keymap {
		/// The number of rows.
		pub rows: usize,
		/// The number of columns for each row.
		pub cols: usize,
		pub row_pins: Vec<u8>,
		pub col_pins: Vec<u8>,
		pub layer0: Vec<Vec<u8>>,
		pub layer1: Vec<Vec<u8>>,
	}

	#[derive(Debug)]
	pub struct KeyboardConfig {
		pub mcu: Mcu,
		pub keymap: Keymap,
	}
}

#[derive(Debug)]
pub struct TomlConfiguration {
	/// Tha name of the board.
	pub name: String,
	/// The board author or manufacturer.
	pub author: String,
	/// The id diferentates the board from the others from the same author.
	pub id: String,
	/// The version of the compiled firmware.
	pub version: u32,

	pub device: Device,

	pub keyboard: keyboard::KeyboardConfig,
}

impl<'de> Deserialize<'de> for TomlConfiguration {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		// First deserialize into a raw version before parsing for errors.
		let raw_config = raw::RawConfig::deserialize(deserializer)?;

		// Parse the version, checking if the values are within the supported bounds.
		if raw_config.firmware.version.pre != semver::Prerelease::EMPTY {
			return Err(de::Error::custom("Pre-release format not supported."));
		}

		if raw_config.firmware.version.build != semver::BuildMetadata::EMPTY {
			return Err(de::Error::custom("Build metadata format not supported."));
		}

		let Ok(major) = u16::try_from(raw_config.firmware.version.major) else {
			return Err(de::Error::custom("Major version out of supported bounds."));
		};

		let Ok(minor) = u16::try_from(raw_config.firmware.version.minor) else {
			return Err(de::Error::custom("Minor version out of supported bounds."));
		};

		let Ok(patch) = u16::try_from(raw_config.firmware.version.patch) else {
			return Err(de::Error::custom("Patch version out of supported bounds."));
		};

		let version = crate::version::Version::new(crate::version::Api::V0, major, minor, patch);

		// Check if the rows and cols size match
		let rows_count = raw_config.keyboard.keymap.rows;
		let cols_count = raw_config.keyboard.keymap.cols;

		let row_pins_count = raw_config.keyboard.keymap.row_pins.len();
		let col_pins_count = raw_config.keyboard.keymap.col_pins.len();

		if row_pins_count != rows_count {
			return Err(de::Error::custom(format!(
				"Expected {rows_count} row pins, got {row_pins_count}",
			)));
		}

		if col_pins_count != cols_count {
			return Err(de::Error::custom(format!(
				"Expected {cols_count} col pins, got {col_pins_count}",
			)));
		}

		let layer0 = &raw_config.keyboard.keymap.layer0;

		if layer0.len() != rows_count {
			return Err(de::Error::custom(format!(
				"Expected {rows_count} rows for layer0, got {}",
				layer0.len()
			)));
		}

		for (i, row) in layer0.iter().enumerate() {
			if row.len() != cols_count {
				return Err(de::Error::custom(format!(
					"Expected {} cols for layer0's row {i}, got {}",
					cols_count,
					row.len()
				)));
			}
		}

		let layer1 = &raw_config.keyboard.keymap.layer1;

		if layer1.len() != rows_count {
			return Err(de::Error::custom(format!(
				"Expected {} rows for layer1, got {}",
				rows_count,
				layer1.len()
			)));
		}

		for (i, row) in layer1.iter().enumerate() {
			if row.len() != cols_count {
				return Err(de::Error::custom(format!(
					"Expected {} cols for layer1's row {i}, got {}",
					cols_count,
					row.len()
				)));
			}
		}

		let keymap = keyboard::Keymap {
			rows: raw_config.keyboard.keymap.rows,
			cols: raw_config.keyboard.keymap.cols,
			row_pins: raw_config.keyboard.keymap.row_pins,
			col_pins: raw_config.keyboard.keymap.col_pins,
			layer0: raw_config.keyboard.keymap.layer0,
			layer1: raw_config.keyboard.keymap.layer1,
		};

		let keyboard_config = keyboard::KeyboardConfig {
			mcu: raw_config.keyboard.mcu,
			keymap,
		};

		Ok(Self {
			name: raw_config.name,
			author: raw_config.firmware.author,
			id: raw_config.firmware.id,
			version: version.as_bitmap(),
			device: raw_config.firmware.device,
			keyboard: keyboard_config,
		})
	}
}

/// # Errors
///
/// Returns an error if the path does not exist. Other errors might also be returned, check
/// the error docs for [`read_to_string`](std::fs::read_to_string).
pub fn parse_file(path: &str) -> Result<Option<TomlConfiguration>, std::io::Error> {
	let path = std::path::Path::new(path);

	let contents = std::fs::read_to_string(path)?;

	Ok(toml::from_str(&contents).ok())
}
