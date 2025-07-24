pub mod keycodes;

pub type PackedKeymap<const S: usize> = [u8; S];

pub struct Keymap<const R: usize, const C: usize>(pub [[u8; C]; R]);

impl<const R: usize, const C: usize> Keymap<R, C> {
	#[must_use]
	pub const fn new(keymap: [[u8; C]; R]) -> Self {
		Self(keymap)
	}

	#[must_use]
	pub const fn get_packed_size(&self) -> usize {
		let keymap = &self.0;

		let mut size = 0;

		let mut i = 0;
		while i < keymap.len() {
			let row = keymap[i];

			let mut j = 0;
			while j < row.len() {
				let key = row[j];

				if key != 0 {
					size += 1;
				}

				j += 1;
			}

			i += 1;
		}

		size
	}

	/// # Panics
	///
	/// Panics if there is a logical error and the length assertion fails.
	#[must_use]
	pub const fn get_packed<const S: usize>(&self) -> PackedKeymap<S> {
		let keymap = &self.0;

		let mut packed_keymap = [0; S];

		let mut base = 0;
		let mut i = 0;

		while i < keymap.len() {
			let row = keymap[i];

			let mut j = 0;
			while j < row.len() {
				let key = row[j];

				if key != 0 {
					packed_keymap[base] = key;

					base += 1;
				}

				j += 1;
			}

			i += 1;
		}

		assert!(base == S, "Invalid length!");

		packed_keymap
	}
}

#[derive(Debug)]
pub struct Keymaps<const S: usize> {
	pub keymap_0: PackedKeymap<S>,
	pub keymap_1: PackedKeymap<S>,
	pub keymap_2: PackedKeymap<S>,
	pub keymap_3: PackedKeymap<S>,
	pub keymap_4: PackedKeymap<S>,
}

#[derive(Debug)]
pub struct KeyboardConfiguration<const S: usize> {
	pub keymaps: Keymaps<S>,
}

/// Generate a keymap using the predefined keycodes.
/// The literal '-' can be passed to represent an empty space.
#[macro_export]
macro_rules! keymap {
	($( [ $($key:tt),* $(,)? ] ),* $(,)?) => {
		$crate::keyboard::Keymap::new([
			$(
				[
					$( $crate::keymap!(@internal $key) ),*
				]
			),*
		])
	};
	(@internal -) => { 0 };
	(@internal $key:ident) => {{
		::core::num::NonZeroU8::get($crate::keyboard::keycodes::$key)
	}};
}
