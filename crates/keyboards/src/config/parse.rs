use serde::{Deserialize, Deserializer, de};

mod raw {
	use std::collections::HashMap;

	use serde::{Deserialize, Serialize};

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Firmware {
		pub author: String,
		pub id: String,
		pub version: String,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Keymap {
		pub rows: usize,
		pub cols: usize,
	}

	#[derive(Debug, Deserialize)]
	pub struct RawConfig {
		pub name: String,
		pub firmware: Firmware,
		pub keymap: Keymap,
		pub rows: HashMap<String, toml::Value>,
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
	/// This is a more literal representation of the layout, which contains every position value,
	/// inlcuding the ones not used. This keymap version is used by the macro when generating
	/// the checks for pressed keys.
	pub layout_keymap: Vec<Vec<u8>>,
	/// The number of rows.
	pub row_len: usize,
	/// The number of columns for each row.
	pub col_len: usize,
	/// A non static representation of [`super::Keymap`].
	pub keymap: Vec<u8>,
	/// The size of the keymap. This is used because the actual keymap
	/// used by the firmware has a static size determined at compile time.
	pub keymap_size: usize,
}

impl<'de> Deserialize<'de> for TomlConfiguration {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		// First deserialize into a raw version before parsing for errors.
		let raw_config = raw::RawConfig::deserialize(deserializer)?;

		// Parse the version, checking if the values are within the supported bounds.
		let parsed_version = match semver::Version::parse(&raw_config.firmware.version) {
			Ok(v) => v,
			Err(e) => {
				return Err(de::Error::custom(format!("Error parsing version: {e}",)));
			}
		};

		let Ok(major) = u16::try_from(parsed_version.major) else {
			return Err(de::Error::custom("Major version out of supported bounds."));
		};

		let Ok(minor) = u16::try_from(parsed_version.minor) else {
			return Err(de::Error::custom("Minor version out of supported bounds."));
		};

		let Ok(patch) = u16::try_from(parsed_version.patch) else {
			return Err(de::Error::custom("Patch version out of supported bounds."));
		};

		if parsed_version.pre != semver::Prerelease::EMPTY {
			return Err(de::Error::custom("Pre-release format not supported."));
		}

		if parsed_version.build != semver::BuildMetadata::EMPTY {
			return Err(de::Error::custom("Build metadata format not supported."));
		}

		let version = super::SemVer::new(super::ApiVersion::V0, major, minor, patch);

		let rows_count = raw_config.keymap.rows;
		let cols_count = raw_config.keymap.cols;

		if raw_config.rows.len() != rows_count {
			return Err(de::Error::custom(format!(
				"Expected {} rows, got {}",
				raw_config.keymap.rows,
				raw_config.rows.len()
			)));
		}

		let mut layout_keymap: Vec<Vec<u8>> = Vec::new();

		// This iterates over each row and check a couple of things. We check if the row key
		// has the expected name, if the row is an array of integers, more specifically an
		// array of u8 values. Then finally if the length matches the number of columns.
		for i in 0..rows_count {
			let key = format!("{i}");

			let Some(toml::Value::Array(key_array)) = raw_config.rows.get(&key) else {
				return Err(de::Error::custom(format!("Missing row: {key}")));
			};

			let keys: Vec<u8> = key_array
				.iter()
				.enumerate()
				.map(|(i, val)| {
					val.as_integer()
						.unwrap_or_else(|| panic!("Expected integer value on row {key}, position {i}."))
				})
				.collect::<Vec<i64>>()
				.into_iter()
				.enumerate()
				.map(|(i, val)| {
					u8::try_from(val)
						.unwrap_or_else(|_| panic!("Expected u8 value on row {key}, position {i}, got value {val}."))
				})
				.collect();

			let keys_len = keys.len();

			if keys_len != cols_count {
				return Err(de::Error::custom(format!(
					"Row {i} expected {cols_count} columns, got {keys_len}"
				)));
			}

			layout_keymap.push(keys);
		}

		// We create the keymap and it's size by iterating over each layout_keymap value,
		// storing only the ones that are not 0 and incrementing the size each time.

		let (keymap, keymap_size) = {
			let mut keymap = Vec::<u8>::new();
			let mut size = 0_usize;

			for row in &layout_keymap {
				for key in row {
					if *key != 0 {
						keymap.push(*key);
						size += 1;
					}
				}
			}

			(keymap, size)
		};

		Ok(Self {
			name: raw_config.name,
			author: raw_config.firmware.author,
			id: raw_config.firmware.id,
			version: version.as_bitmap(),
			layout_keymap,
			row_len: rows_count,
			col_len: cols_count,
			keymap,
			keymap_size,
		})
	}
}

/// # Errors
///
/// Can return a file or parsing error.
pub fn parse_model_config(author: &str, model: &str) -> Result<TomlConfiguration, anyhow::Error> {
	const BASE: &str = env!("CARGO_MANIFEST_DIR");

	let config_path = format!("{BASE}/models/{author}/{model}/config.toml");

	let toml_config = std::fs::read_to_string(config_path)?;

	let config: TomlConfiguration = toml::de::from_str(&toml_config)?;

	Ok(config)
}
