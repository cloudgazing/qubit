use std::prelude::rust_2024::*;

use serde::{Deserialize, Deserializer, de};

type LayoutKeymap = Vec<Vec<u8>>;
type RowLen = usize;
type ColLen = usize;

type Keymap = Vec<u8>;
type KeymapSize = usize;

mod raw {
	use std::prelude::rust_2024::*;

	use std::collections::HashMap;

	use serde::{Deserialize, Serialize};

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

	#[derive(Deserialize)]
	pub struct RawConfig {
		pub name: String,
		pub firmware: Firmware,
		pub keymap: Keymap,
		pub rows: HashMap<String, toml::Value>,
	}
}

pub struct FullConfiguration {
	pub name: String,
	pub author: String,
	pub id: String,
	pub version: String,
	pub layout_keymap: LayoutKeymap,
	pub row_len: RowLen,
	pub col_len: ColLen,
	pub keymap: Keymap,
	pub keymap_size: KeymapSize,
}

impl<'de> Deserialize<'de> for FullConfiguration {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let raw_config = raw::RawConfig::deserialize(deserializer)?;

		let rows_count = raw_config.keymap.rows;
		let cols_count = raw_config.keymap.cols;

		if raw_config.rows.len() != rows_count {
			return Err(de::Error::custom(format!(
				"Expected {} rows, got {}",
				raw_config.keymap.rows,
				raw_config.rows.len()
			)));
		}

		assert!(
			raw_config.keymap.use_scan_codes,
			"Only scan codes layout is supported for now!"
		);

		let mut layout_keymap: LayoutKeymap = Vec::new();

		for i in 0..rows_count {
			let key = format!("{i}");

			let Some(toml::Value::Array(key_array)) = raw_config.rows.get(&key) else {
				panic!("Missing row: {key}");
			};

			let keys: Vec<u8> = key_array
				.iter()
				.map(|v| {
					v.as_integer()
						.expect("Expected integer value because 'use_scan_codes' is true.")
				})
				.collect::<Vec<i64>>()
				.into_iter()
				.map(|v| u8::try_from(v).unwrap())
				.collect();

			let keys_len = keys.len();

			if keys_len != cols_count {
				return Err(de::Error::custom(format!(
					"Row {i} expected {cols_count} columns, got {keys_len}"
				)));
			}

			layout_keymap.push(keys);
		}

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
			version: raw_config.firmware.version,
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
pub fn parse_model_config(model_name: &str) -> Result<FullConfiguration, anyhow::Error> {
	let config_path = format!("{}/models/{model_name}/config.toml", env!("CARGO_MANIFEST_DIR"));

	let toml_config = std::fs::read_to_string(config_path)?;

	let config: FullConfiguration = toml::de::from_str(&toml_config)?;

	Ok(config)
}
