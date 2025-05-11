use core::fmt::{self, Write};

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
