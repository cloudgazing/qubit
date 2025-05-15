//! A collection of helpers and other types that don't fit in any specific category.

use core::fmt::{self, Write};

/// A buffer implemetation that makes writing strings to serial more convenient
pub struct Buffer<const N: usize>(pub [u8; N], pub usize);

impl<const N: usize> Write for Buffer<N> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let space_left = self.0.len() - self.1;
		if space_left > s.len() {
			self.0[self.1..][..s.len()].copy_from_slice(s.as_bytes());
			self.1 += s.len();
			Ok(())
		} else {
			Err(fmt::Error)
		}
	}
}

/// Parse a string to a usize.
///
/// This const function is used to parse env values from build.rs at compile time.
pub const fn parse_env_usize(s: &str) -> usize {
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
