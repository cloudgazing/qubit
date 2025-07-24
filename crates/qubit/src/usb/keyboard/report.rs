use core::num::NonZeroU8;

use qubit_config::keyboard::keycodes::{KC_A, KC_LEFTCTRL, KC_RIGHTMETA, RESERVED};

use super::PRESSED_KEYS_BITMAPS_LEN;
use super::descriptor::KB_REP_ID_IN;
use super::keymaps::get_keymap_keycode;

// id + modifier + reserved + 6 keys
pub type Keyboard6kroReport = [u8; 9];
// id + modifier + 32 bytes bitmap
pub type KeyboardNkroReport = [u8; 34];

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

/// # Safety
///
/// Calling this function before the active keymap was initiated is **undefined behavior**.
pub unsafe fn construct_nkro_report(pressed_keys: [usize; PRESSED_KEYS_BITMAPS_LEN]) -> KeyboardNkroReport {
	const USIZE_BITS: usize = usize::BITS as usize;
	const NKRO_REP_LEN: usize = 34;

	// [report_id, modifier, keys...]
	let mut report = [0_u8; NKRO_REP_LEN];

	report[0] = KB_REP_ID_IN;

	for (index, mut bitmap) in pressed_keys.into_iter().enumerate() {
		let offset = index * USIZE_BITS;

		while bitmap != 0 {
			let pressed_bit = bitmap.trailing_zeros() as usize;

			let flat_index = offset + pressed_bit;

			// SAFETY: The caller gurantees the keymap was initiated.
			let code = unsafe { get_keymap_keycode(flat_index) };

			if let Some(code) = NonZeroU8::new(code) {
				if is_normal_key(code) {
					let key_code = code.get();

					let byte_index = (key_code / 8) as usize + 2;
					let bit_index = (key_code % 8) as usize;

					if byte_index < NKRO_REP_LEN {
						report[byte_index] |= 1 << bit_index;
					}
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

#[cfg(feature = "defmt")]
pub fn log_6kro_report(report: Keyboard6kroReport) {
	use core::fmt::Write;

	let mut msg = heapless::String::<500>::new();

	let mut keys = report.iter();

	// remove report id
	keys.next();

	let modifier_msg = if keys.next().is_some_and(|&v| v == 0) {
		"none"
	} else {
		"??"
	};

	// remove reserved
	keys.next();

	let mut pressed_keys = heapless::String::<128>::new();

	for (i, key) in keys.enumerate() {
		if *key == 0 {
			break;
		}

		if i != 0 {
			write!(pressed_keys, ", ").unwrap();
		}

		write!(pressed_keys, "0x{key:02}").unwrap();
	}

	writeln!(msg, "6KRO report sent:").ok();
	writeln!(msg, "Modifiers: {modifier_msg}").ok();
	writeln!(msg, "Keys: [{pressed_keys}]").ok();
	write!(msg, "---").ok();

	defmt::info!("{}", msg);
}

#[cfg(feature = "defmt")]
pub fn log_nkro_report(report: KeyboardNkroReport) {
	use core::fmt::Write;

	let mut msg = heapless::String::<500>::new();

	let mut keys = report.iter();

	// remove report id
	keys.next();

	let modifier_msg = if keys.next().is_some_and(|&v| v == 0) {
		"none"
	} else {
		"??"
	};

	let mut pressed_keys = heapless::String::<500>::new();

	for (i, key) in keys.enumerate() {
		if *key == 0 {
			continue;
		}

		if i != 0 {
			write!(pressed_keys, ", ").unwrap();
		}

		write!(pressed_keys, "0x{key:02}").unwrap();
	}

	writeln!(msg, "NKRO report sent:").ok();
	writeln!(msg, "Modifiers: {modifier_msg}").ok();
	writeln!(msg, "Keys: [{pressed_keys}]").ok();
	write!(msg, "---").ok();

	defmt::info!("{}", msg);
}
