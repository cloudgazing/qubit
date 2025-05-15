#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "std")]
use std::prelude::rust_2024::*;

pub mod codes;
pub mod config;

#[must_use]
#[cfg(feature = "std")]
pub fn memory_x_path(model_name: &str) -> String {
	format!("{}/models/{model_name}/memory.x", env!("CARGO_MANIFEST_DIR"))
}

/// # Errors
///
/// Returns an [`std::io::Error`] if the file was not found or there was or another error occured
/// trying to read from it.
#[cfg(feature = "std")]
pub fn memory_x_contents(model_name: &str) -> Result<Vec<u8>, std::io::Error> {
	let memory_x_path = format!("{}/models/{model_name}/memory.x", env!("CARGO_MANIFEST_DIR"));

	std::fs::read(memory_x_path)
}
