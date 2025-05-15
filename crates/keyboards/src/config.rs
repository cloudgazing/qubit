#[cfg(feature = "std")]
pub mod parse {
	use std::{collections::HashMap, prelude::rust_2024::*};

	use serde::{Deserialize, Deserializer, Serialize, de};

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Keymap {
		pub use_scan_codes: bool,
		pub rows: usize,
		pub cols: usize,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Firmware {
		pub author: String,
		pub id: String,
		pub version: String,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub enum Row {
		Value(Vec<String>),
		Codes(Vec<u8>),
	}

	#[derive(Debug, Serialize)]
	pub struct Config {
		pub name: String,
		pub firmware: Firmware,
		pub keymap: Keymap,
		pub rows: Vec<Vec<u8>>,
	}

	impl<'de> Deserialize<'de> for Config {
		fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
			#[derive(Deserialize)]
			struct RawConfig {
				name: String,
				firmware: Firmware,
				keymap: Keymap,
				rows: HashMap<String, toml::Value>,
			}

			let raw_config = RawConfig::deserialize(deserializer)?;

			let rows_count = raw_config.keymap.rows;
			let cols_count = raw_config.keymap.cols;

			if raw_config.rows.len() != rows_count {
				return Err(de::Error::custom(format!(
					"Expected {} rows, got {}",
					raw_config.keymap.rows,
					raw_config.rows.len()
				)));
			}

			let mut rows = Vec::new();

			for i in 0..rows_count {
				let key = format!("{i}");

				let Some(value) = raw_config.rows.get(&key) else {
					panic!("Missing row: {key}");
				};

				let row = if raw_config.keymap.use_scan_codes {
					let vals: Vec<u8> = value
						.as_array()
						.unwrap()
						.iter()
						.map(|v| {
							v.as_integer()
								.expect("Expected integer value because 'use_scan_codes' is true.")
						})
						.collect::<Vec<i64>>()
						.into_iter()
						.map(|v| u8::try_from(v).unwrap())
						.collect();

					Row::Codes(vals)
				} else {
					let vals: Vec<String> = value
						.as_array()
						.unwrap()
						.iter()
						.map(|v| {
							v.as_str()
								.expect("Expected string value because 'use_scan_codes' is false.")
								.to_string()
						})
						.collect();

					Row::Value(vals)
				};

				let col_len = match row {
					Row::Value(ref v) => v.len(),
					Row::Codes(ref v) => v.len(),
				};

				if col_len != raw_config.keymap.cols {
					return Err(de::Error::custom(format!(
						"Row {i} expected {cols_count} columns, got {col_len}"
					)));
				}

				// TODO: Add support for parsing the value option, for now this panics.
				let Row::Codes(row) = row else {
					panic!("expected codes row")
				};

				rows.push(row);
			}

			Ok(Self {
				name: raw_config.name,
				firmware: raw_config.firmware,
				keymap: raw_config.keymap,
				rows,
			})
		}
	}

	/// # Errors
	///
	/// Can return a file or parsing error.
	pub fn parse_model_config(model_name: &str) -> Result<Config, anyhow::Error> {
		let config_path = format!("{}/models/{model_name}/config.toml", env!("CARGO_MANIFEST_DIR"));

		let toml_config = std::fs::read_to_string(config_path)?;

		let config: Config = toml::de::from_str(&toml_config)?;

		Ok(config)
	}
}

pub type Keymap<const R: usize, const C: usize> = [[u8; C]; R];

pub struct Configuration<const R: usize, const C: usize> {
	/// Tha name of the board.
	pub name: &'static str,
	/// The board author or manufacturer.
	pub author: &'static str,
	/// The id diferentates the board from the others.
	pub id: &'static str,
	pub version: &'static str,
	pub keymap: Keymap<R, C>,
}
