const BASE: &str = env!("CARGO_MANIFEST_DIR");
const LINKER_FILENAME: &str = "memory.x";

#[must_use]
pub fn linker_file_path_custom() -> String {
	format!("{BASE}/models/custom/{LINKER_FILENAME}")
}

#[must_use]
pub fn linker_file_path(author: &str, model: &str) -> String {
	format!("{BASE}/models/{author}/{model}/{LINKER_FILENAME}")
}

/// # Errors
///
/// Returns an [`std::io::Error`] if the file was not found or there was or another error occured
/// trying to read from it.
pub fn linker_contents_custom() -> Result<Vec<u8>, std::io::Error> {
	let linker_path = format!("{BASE}/models/custom/{LINKER_FILENAME}");

	std::fs::read(linker_path)
}

/// # Errors
///
/// Returns an [`std::io::Error`] if the file was not found or there was or another error occured
/// trying to read from it.
pub fn linker_contents(author: &str, model: &str) -> Result<Vec<u8>, std::io::Error> {
	let linker_path = format!("{BASE}/models/{author}/{model}/{LINKER_FILENAME}");

	std::fs::read(linker_path)
}
