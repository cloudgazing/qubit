use core::num::NonZeroU8;

use qubit_config::keyboard::keycodes::{
	KC_A, KC_LAYER_0, KC_LAYER_1, KC_LAYER_2, KC_LAYER_3, KC_LAYER_4, KC_LEFTCTRL, KC_RIGHTMETA, RESERVED,
};

use super::descriptor::KB_REP_ID_IN;
use super::keymaps::{KeymapsState, Layer, PACKED_SIZE};
use crate::codegen::KeyboardMatrix;

type ScannedKeys = [usize; KeyboardMatrix::BITMAP_COUNT];

// id + modifier + reserved + 6 keys
pub type Report6kro = [u8; 9];
// id + modifier + 32 bytes bitmap
pub type ReportNkro = [u8; 34];

pub const EMPTY_6KRO_REPORT: Report6kro = [KB_REP_ID_IN, 0, RESERVED, 0, 0, 0, 0, 0, 0];
pub const EMPTY_NKRO_REPORT: ReportNkro = {
	let mut report = [0u8; 34];

	report[0] = KB_REP_ID_IN;

	report
};

/// Checks the keycode is within the range of "normal" codes.
fn is_normal(key_code: NonZeroU8) -> bool {
	// 0xdd  Keypad Hexadecimal
	const KEYPAD_HEXDEC: NonZeroU8 = NonZeroU8::new(0xDD).unwrap();

	key_code >= KC_A && key_code <= KEYPAD_HEXDEC
}

/// Checks if the keycode matches a modifier scan code and turns it into it's modifier mask
/// counterpart.
fn is_modifier(key_code: NonZeroU8) -> Option<NonZeroU8> {
	if key_code >= KC_LEFTCTRL && key_code <= KC_RIGHTMETA {
		let modifier_mask: u8 = 1 << (key_code.get() & 0x07);

		// SAFETY: This is guaranteed to be non-zero since a bitand with the 0x07 mask will always result in
		// a value between 0 and 7.
		let mask = unsafe { NonZeroU8::new_unchecked(modifier_mask) };

		return Some(mask);
	}

	None
}

fn is_layer(keycode: NonZeroU8) -> Option<Layer> {
	match keycode {
		KC_LAYER_0 => Some(Layer::L0),
		KC_LAYER_1 => Some(Layer::L1),
		KC_LAYER_2 => Some(Layer::L2),
		KC_LAYER_3 => Some(Layer::L3),
		KC_LAYER_4 => Some(Layer::L4),
		_ => None,
	}
}

#[derive(Debug)]
pub struct ReportState {
	prev_6kro_report: Report6kro,
	prev_nkro_report: ReportNkro,

	prev_scanned_keys: ScannedKeys,
	pressed_keys: [Option<NonZeroU8>; PACKED_SIZE],
}

impl ReportState {
	pub fn new() -> Self {
		Self {
			prev_6kro_report: EMPTY_6KRO_REPORT,
			prev_nkro_report: EMPTY_NKRO_REPORT,

			prev_scanned_keys: [0; KeyboardMatrix::BITMAP_COUNT],
			pressed_keys: [None; PACKED_SIZE],
		}
	}

	#[allow(
		clippy::trivially_copy_pass_by_ref,
		reason = "Lint is triggered only when `scanned_keys` has one element. Makes sense to allow since that has the
		same size as the target pointer width."
	)]
	pub fn build_6kro_report(&mut self, keymaps: &mut KeymapsState, scanned_keys: &ScannedKeys) -> Option<&[u8]> {
		const USIZE_BITS: usize = usize::BITS as usize;

		let mut report = self.prev_6kro_report;

		for (current_idx, current_bitmap) in scanned_keys.iter().enumerate() {
			let offset = current_idx * USIZE_BITS;

			let prev_bitmap = self.prev_scanned_keys[current_idx];

			let mut set_bitmap = current_bitmap & !prev_bitmap;
			let mut cleared_bitmap = prev_bitmap & !current_bitmap;

			while cleared_bitmap != 0 {
				let bit_pos = cleared_bitmap.trailing_zeros() as usize;

				let flat_idx = offset + bit_pos;

				let keycode = self.pressed_keys[flat_idx].take().unwrap();

				if let Some(layer) = is_layer(keycode) {
					keymaps.disable_layer(layer);
				} else if is_normal(keycode) {
					if let Some(slot) = report[3..9].iter_mut().find(|slot| **slot == keycode.get()) {
						*slot = 0;
					}
				} else if let Some(mod_code) = is_modifier(keycode) {
					report[1] &= !mod_code.get();
				}

				cleared_bitmap &= !(1 << bit_pos);
			}

			while set_bitmap != 0 {
				let bit_pos = set_bitmap.trailing_zeros() as usize;

				let flat_idx = offset + bit_pos;

				let keycode = keymaps.get_keycode(flat_idx);

				self.pressed_keys[flat_idx] = Some(keycode);

				if let Some(layer) = is_layer(keycode) {
					keymaps.enable_layer(layer);
				} else if is_normal(keycode) {
					if let Some(slot) = report[3..9].iter_mut().find(|slot| **slot == 0) {
						*slot = keycode.get();
					}
				} else if let Some(mod_code) = is_modifier(keycode) {
					report[1] |= mod_code.get();
				}

				set_bitmap &= !(1 << bit_pos);
			}

			self.prev_scanned_keys[current_idx] = *current_bitmap;
		}

		if report != self.prev_6kro_report {
			self.prev_6kro_report = report;

			#[cfg(feature = "defmt")]
			self.log_6kro_report();

			return Some(&self.prev_6kro_report);
		}

		None
	}

	#[allow(
		clippy::trivially_copy_pass_by_ref,
		reason = "Lint is triggered only when `scanned_keys` has one element. Makes sense to allow since that has the
		same size as the target pointer width."
	)]
	pub fn build_nkro_report(&mut self, keymaps: &mut KeymapsState, scanned_keys: &ScannedKeys) -> Option<&[u8]> {
		const USIZE_BITS: usize = usize::BITS as usize;

		let mut report = self.prev_nkro_report;

		for (current_idx, current_bitmap) in scanned_keys.iter().enumerate() {
			let offset = current_idx * USIZE_BITS;

			let prev_bitmap = self.prev_scanned_keys[current_idx];

			let mut set_bitmap = current_bitmap & !prev_bitmap;
			let mut cleared_bitmap = prev_bitmap & !current_bitmap;

			while cleared_bitmap != 0 {
				let bit_pos = cleared_bitmap.trailing_zeros() as usize;

				let flat_idx = offset + bit_pos;

				let keycode = self.pressed_keys[flat_idx].take().unwrap();

				if let Some(layer) = is_layer(keycode) {
					keymaps.disable_layer(layer);
				} else if is_normal(keycode) {
					let byte_idx = ((keycode.get() / 8) + 2) as usize;
					let bit_idx = (keycode.get() % 8) as usize;

					// This will always be within bounds because (u8::MAX / 8) + 2 = 33
					report[byte_idx] &= !(1 << bit_idx);
				} else if let Some(mod_code) = is_modifier(keycode) {
					report[1] &= !mod_code.get();
				}

				cleared_bitmap &= !(1 << bit_pos);
			}

			while set_bitmap != 0 {
				let bit_pos = set_bitmap.trailing_zeros() as usize;

				let flat_idx = offset + bit_pos;

				let keycode = keymaps.get_keycode(flat_idx);

				self.pressed_keys[flat_idx] = Some(keycode);

				if let Some(layer) = is_layer(keycode) {
					keymaps.enable_layer(layer);
				} else if is_normal(keycode) {
					let byte_idx = ((keycode.get() / 8) + 2) as usize;
					let bit_idx = (keycode.get() % 8) as usize;

					// This will always be within bounds because (u8::MAX / 8) + 2 = 33
					report[byte_idx] |= 1 << bit_idx;
				} else if let Some(mod_code) = is_modifier(keycode) {
					report[1] |= mod_code.get();
				}

				set_bitmap &= !(1 << bit_pos);
			}

			self.prev_scanned_keys[current_idx] = *current_bitmap;
		}

		if report != self.prev_nkro_report {
			self.prev_nkro_report = report;

			#[cfg(feature = "defmt")]
			self.log_nkro_report();

			return Some(&self.prev_nkro_report);
		}

		None
	}

	#[cfg(feature = "defmt")]
	pub fn log_6kro_report(&self) {
		use core::fmt::Write;

		let mut msg = heapless::String::<700>::new();

		writeln!(msg, "6KRO report sent:").ok();

		let mut report = self.prev_6kro_report.iter();

		// Remove report id.
		report.next();

		write!(msg, "Modifiers: [").ok();

		if let Some(mod_byte) = report.next() {
			write!(msg, "{mod_byte:08b}").ok();
		}

		writeln!(msg, "]").ok();

		// Remove reserved byte.
		report.next();

		write!(msg, "Keys: [").ok();

		for (i, key) in report.enumerate() {
			if *key == 0 {
				break;
			}

			if i != 0 {
				write!(msg, ", ").ok();
			}

			write!(msg, "{key:#04X}").ok();
		}

		writeln!(msg, "]").ok();

		write!(msg, "---").ok();

		defmt::debug!("{}", msg);
	}

	#[cfg(feature = "defmt")]
	pub fn log_nkro_report(&self) {
		use core::fmt::Write;

		let mut msg = heapless::String::<1000>::new();

		writeln!(msg, "NKRO report sent:").ok();

		let mut report = self.prev_nkro_report.iter();

		// Remove report id.
		report.next();

		write!(msg, "Modifiers: [").ok();

		if let Some(mod_byte) = report.next() {
			write!(msg, "{mod_byte:08b}").ok();
		}

		writeln!(msg, "]").ok();

		write!(msg, "Keys: [").ok();

		for (i, key) in report.enumerate() {
			let offset = i * u8::BITS as usize;

			let mut key = *key;

			while key != 0 {
				let bit_pos = key.trailing_zeros() as usize;

				let keycode = offset + bit_pos;

				write!(msg, "{keycode:#04X}").ok();
				write!(msg, ", ").ok();

				key &= !(1 << bit_pos);
			}
		}

		writeln!(msg, "]").ok();

		write!(msg, "---").ok();

		defmt::debug!("{}", msg);
	}
}
