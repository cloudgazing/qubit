/// Parse a string to a usize.
///
/// This const function is used to parse env values from build.rs at compile time.
pub const fn str_to_usize(s: &str) -> usize {
	let bytes = s.as_bytes();

	let mut i = 0;
	let mut result = 0;

	while i < bytes.len() {
		let digit = bytes[i] - b'0';

		assert!(digit <= 9);

		result = result * 10 + digit as usize;

		i += 1;
	}

	result
}
