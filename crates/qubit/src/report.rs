use core::num::NonZeroU8;

use keyboards::keycodes::{KEY_A, KEY_LEFTCTRL, KEY_RIGHTMETA};
use usbd_hid::descriptor::KeyboardReport;

use crate::keymap;

/// Checks the keycode is within the range of "normal" codes.
fn is_normal_key(key_code: NonZeroU8) -> bool {
	// 0xdd  Keypad Hexadecimal
	const KEYPAD_HEXDEC: NonZeroU8 = NonZeroU8::new(0xdd).unwrap();

	key_code >= KEY_A && key_code <= KEYPAD_HEXDEC
}

/// Checks if the keycode matches a modifier scan code and turns it into it's modifier mask
/// counterpart.
fn is_modifier_key(key_code: NonZeroU8) -> Option<NonZeroU8> {
	if key_code >= KEY_LEFTCTRL && key_code <= KEY_RIGHTMETA {
		let modifier_mask: u8 = 1 << (key_code.get() & 0x07);

		// Safety: This is guaranteed to be non-zero since a bitand with the 0x07 mask will always result in
		// a value between 0 and 7.
		let mask = unsafe { NonZeroU8::new_unchecked(modifier_mask) };

		Some(mask)
	} else {
		None
	}
}

const PRESSED_BITMAPS_SIZE: usize = crate::KM_SIZE.div_ceil(usize::BITS as usize);

/// # Safety
///
/// Calling this function before the active keymap was initiated is **undefined behavior**.
pub unsafe fn construct_keyboard_report(pressed_bitmaps: [usize; PRESSED_BITMAPS_SIZE]) -> KeyboardReport {
	const SIZE: usize = usize::BITS as usize;

	let mut modifier = 0_u8;

	let mut keycodes = [0_u8; SIZE];
	let mut i = 0;

	for (index, mut bitmap) in pressed_bitmaps.into_iter().enumerate() {
		let offset = index * SIZE;

		while bitmap != 0 {
			let pressed_bit = bitmap.trailing_zeros() as usize;

			let flat_index = offset + pressed_bit;

			// SAFETY: The caller gurantees the keymap was initiated.
			let code = unsafe { keymap::get_keymap_keycode(flat_index) };

			if let Some(code) = NonZeroU8::new(code) {
				if is_normal_key(code) {
					keycodes[i] = code.get();

					i += 1;
				} else if let Some(mod_code) = is_modifier_key(code) {
					modifier |= mod_code.get();
				}
			}

			// Clear the bit
			bitmap &= !(1 << pressed_bit);
		}
	}

	// For now just take the first 6 keys until N-Key rollover is implemented.
	let keycodes = keycodes[0..6].try_into().unwrap();

	KeyboardReport {
		modifier,
		reserved: 0,
		leds: 0,
		keycodes,
	}
}
// hnu6 ab
