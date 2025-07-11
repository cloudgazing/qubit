use core::num::NonZeroU8;

use qubit_config::keyboard::keycodes::{KC_A, KC_LEFTCTRL, KC_RIGHTMETA, RESERVED};

use super::PRESSED_KEYS_BITMAPS_LEN;
use super::descriptor::KB_REP_ID_IN;
use super::keymaps::get_keymap_keycode;

// id + modifier + reserved + 6 keys
pub type Keyboard6kroReport = [u8; 9];

/// Checks the keycode is within the range of "normal" codes.
fn is_normal_key(key_code: NonZeroU8) -> bool {
	// 0xdd  Keypad Hexadecimal
	const KEYPAD_HEXDEC: NonZeroU8 = NonZeroU8::new(0xDD).unwrap();

	key_code >= KC_A && key_code <= KEYPAD_HEXDEC
}

/// Checks if the keycode matches a modifier scan code and turns it into it's modifier mask
/// counterpart.
fn is_modifier_key(key_code: NonZeroU8) -> Option<NonZeroU8> {
	if key_code >= KC_LEFTCTRL && key_code <= KC_RIGHTMETA {
		let modifier_mask: u8 = 1 << (key_code.get() & 0x07);

		// SAFETY: This is guaranteed to be non-zero since a bitand with the 0x07 mask will always result in
		// a value between 0 and 7.
		let mask = unsafe { NonZeroU8::new_unchecked(modifier_mask) };

		Some(mask)
	} else {
		None
	}
}

/// # Safety
///
/// Calling this function before the active keymap was initiated is **undefined behavior**.
pub unsafe fn construct_6kro_report(pressed_keys: [usize; PRESSED_KEYS_BITMAPS_LEN]) -> Keyboard6kroReport {
	const USIZE_BITS: usize = usize::BITS as usize;
	const REPORT_LEN: usize = core::mem::size_of::<Keyboard6kroReport>();

	let mut report: Keyboard6kroReport = [KB_REP_ID_IN, 0, RESERVED, 0, 0, 0, 0, 0, 0];

	let mut i = 3;

	for (index, mut bitmap) in pressed_keys.into_iter().enumerate() {
		let offset = index * USIZE_BITS;

		while bitmap != 0 {
			let pressed_bit = bitmap.trailing_zeros() as usize;

			let flat_index = offset + pressed_bit;

			// SAFETY: The caller gurantees the keymap was initiated.
			let code = unsafe { get_keymap_keycode(flat_index) };

			if let Some(code) = NonZeroU8::new(code) {
				if i < REPORT_LEN && is_normal_key(code) {
					report[i] = code.get();

					i += 1;
				} else if let Some(mod_code) = is_modifier_key(code) {
					report[1] |= mod_code.get();
				}
			}

			// Clear the bit
			bitmap &= !(1 << pressed_bit);
		}
	}

	report
}

// /// # Safety
// ///
// /// Calling this function before the active keymap was initiated is **undefined behavior**.
// pub unsafe fn _construct_nkro_report(pressed_keys: [usize; PRESSED_BITMAPS_SIZE]) -> RawKeyboardReport {
// 	const USIZE_BITS: usize = usize::BITS as usize;

// 	let mut modifier = 0_u8;

// 	let mut keycodes = [0_u8; USIZE_BITS];

// 	let mut i = 0;

// 	for (index, mut bitmap) in pressed_keys.into_iter().enumerate() {
// 		let offset = index * USIZE_BITS;

// 		while bitmap != 0 {
// 			let pressed_bit = bitmap.trailing_zeros() as usize;

// 			let flat_index = offset + pressed_bit;

// 			// SAFETY: The caller gurantees the keymap was initiated.
// 			let code = unsafe { keymap::get_keymap_keycode(flat_index) };

// 			if let Some(code) = NonZeroU8::new(code) {
// 				if is_normal_key(code) {
// 					keycodes[i] = code.get();

// 					i += 1;
// 				} else if let Some(mod_code) = is_modifier_key(code) {
// 					modifier |= mod_code.get();
// 				}
// 			}

// 			// Clear the bit
// 			bitmap &= !(1 << pressed_bit);
// 		}
// 	}
// }
