#[derive(Debug)]
pub struct Keymaps<const S: usize> {
	pub keymap_0: [u8; S],
	pub keymap_1: [u8; S],
	pub keymap_2: [u8; S],
	pub keymap_3: [u8; S],
	pub keymap_4: [u8; S],
}

impl<const S: usize> Keymaps<S> {
	pub const KEYMAP_SIZE: usize = S;
}

/// Generate keymaps using the predefined keycodes.
/// The literal '-' symbol represents an empty space.
#[macro_export]
macro_rules! keymaps {
	(
		$bool_matrix_vis:vis $bool_matrix:ident, $packed_vis:vis $packed:ident : $rows:ident, $cols:ident :
		0 : [
			$( [ $( $key0:tt ),* $(,)? ] ),* $(,)?
		],
		1 : [
			$( [ $( $key1:tt ),* $(,)? ] ),* $(,)?
		],
		2 : [
			$( [ $( $key2:tt ),* $(,)? ] ),* $(,)?
		],
		3 : [
			$( [ $( $key3:tt ),* $(,)? ] ),* $(,)?
		],
		4 : [
			$( [ $( $key4:tt ),* $(,)? ] ),* $(,)?
		] $(,)?
	) => {
		/// A matrix which shows the presence of a key at a particular coordinate.
		$bool_matrix_vis const $bool_matrix: [[bool; $cols]; $rows] = {
			$crate::keymaps!(@assert_empty_spaces [ $( [ $( $key0 ),* ] ),* ], [ $( [ $( $key1 ),* ] ),* ]);
			$crate::keymaps!(@assert_empty_spaces [ $( [ $( $key0 ),* ] ),* ], [ $( [ $( $key2 ),* ] ),* ]);
			$crate::keymaps!(@assert_empty_spaces [ $( [ $( $key0 ),* ] ),* ], [ $( [ $( $key3 ),* ] ),* ]);
			$crate::keymaps!(@assert_empty_spaces [ $( [ $( $key0 ),* ] ),* ], [ $( [ $( $key4 ),* ] ),* ]);

			[ $( [ $( $crate::keymaps!(@option_u8 $key0).is_some() ),* ] ),* ]
		};

		$packed_vis type Keymaps = $crate::keyboard::keymap::Keymaps<{
			$crate::keyboard::keymap::_count_some(&[ $( [ $( $crate::keymaps!(@option_u8 $key0) ),* ] ),* ])
		}>;

		/// The keymaps stored in a packed representation (without the empty spaces).
		$packed_vis const $packed: Keymaps = $crate::keyboard::keymap::Keymaps {
			keymap_0: $crate::keyboard::keymap::_get_some(&[ $( [ $( $crate::keymaps!(@option_u8 $key0) ),* ] ),* ]),
			keymap_1: $crate::keyboard::keymap::_get_some(&[ $( [ $( $crate::keymaps!(@option_u8 $key1) ),* ] ),* ]),
			keymap_2: $crate::keyboard::keymap::_get_some(&[ $( [ $( $crate::keymaps!(@option_u8 $key2) ),* ] ),* ]),
			keymap_3: $crate::keyboard::keymap::_get_some(&[ $( [ $( $crate::keymaps!(@option_u8 $key3) ),* ] ),* ]),
			keymap_4: $crate::keyboard::keymap::_get_some(&[ $( [ $( $crate::keymaps!(@option_u8 $key4) ),* ] ),* ]),
		};
	};

	( @option_u8 - ) => { Option::<u8>::None };
	( @option_u8 KC_NONE ) => { Some($crate::keyboard::keycodes::KC_NONE) };
	( @option_u8 $key:ident ) => { Some(::core::num::NonZeroU8::get($crate::keyboard::keycodes::$key)) };

	( @boolean $key:tt ) => { $crate::keymaps!(@option_u8 $key).is_some() };

	( @assert_empty_spaces [ $( [ $( $key0:tt ),* ] ),* ], [ $( [ $( $key1:tt ),* ] ),* ] ) => {
		$(
			$(
				assert!(
					$crate::keymaps!(@boolean $key0) == $crate::keymaps!(@boolean $key1),
					$crate::keymaps!(@assert_msg $key0, $key1)
				);
			)*
		)*
	};

	( @assert_msg -, $v:tt ) => { "Expected empty space, found value!" };
	( @assert_msg $v:tt, - ) => { "Expected value, found empty space!" };
	( @assert_msg $v0:tt, $v1:tt ) => { "Expected value or empty space, found the opposite!" };
}

#[doc(hidden)]
#[must_use]
pub const fn _count_some<const R: usize, const C: usize>(arr: &[[Option<u8>; C]; R]) -> usize {
	let mut count = 0;

	let mut i = 0;
	while i < arr.len() {
		let mut j = 0;
		while j < arr[i].len() {
			if arr[i][j].is_some() {
				count += 1;
			}

			j += 1;
		}

		i += 1;
	}

	count
}

#[doc(hidden)]
#[must_use]
pub const fn _get_some<const R: usize, const C: usize, const S: usize>(arr: &[[Option<u8>; C]; R]) -> [u8; S] {
	let mut packed_keymap = [0; S];

	let mut base = 0;

	let mut i = 0;
	while i < arr.len() {
		let mut j = 0;
		while j < arr[i].len() {
			if let Some(value) = arr[i][j] {
				packed_keymap[base] = value;

				base += 1;
			}

			j += 1;
		}

		i += 1;
	}

	assert!(base == S, "Invalid length!");

	packed_keymap
}
