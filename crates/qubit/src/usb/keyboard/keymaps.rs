use core::mem::MaybeUninit;

use qubit_config::keyboard::Keymaps;

use super::{CONFIG, PACKED_SIZE};

static mut ACTIVE_KEYMAPS: MaybeUninit<Keymaps<PACKED_SIZE>> = MaybeUninit::uninit();

/// Get the keymap from the storage.
///
/// This does nothing for now and it's just a reminder to implement EEPROM support.
///
/// TODO: Keymaps should be stored in an EEPROM or something similar.
fn fetch_stored_keymap() -> Option<Keymaps<PACKED_SIZE>> {
	// Here I need to read from the EEPROM chip.
	let stored_keymap: Option<Keymaps<PACKED_SIZE>> = None;

	stored_keymap
}

/// # Safety
///
/// This function must be called **only once** for the lifetime of the program.
pub unsafe fn init_active_keymaps() {
	let keymap = if let Some(keymap) = fetch_stored_keymap() {
		keymap
	} else {
		Keymaps {
			keymap_0: CONFIG.keymaps.keymap_0,
			keymap_1: CONFIG.keymaps.keymap_1,
			keymap_2: CONFIG.keymaps.keymap_2,
			keymap_3: CONFIG.keymaps.keymap_3,
			keymap_4: CONFIG.keymaps.keymap_4,
		}
	};

	let ptr = &raw mut ACTIVE_KEYMAPS;

	// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and properly
	// aligned. This sets the value of the MaybeUninit.
	unsafe {
		(*ptr).write(keymap);
	}
}

/// # Safety
///
/// Calling this function before initializing the active keymap is **undefined behavior**.
pub const unsafe fn get_keymap_keycode(index: usize) -> u8 {
	let active_keymap = {
		let ptr = &raw mut ACTIVE_KEYMAPS;

		// SAFETY: The caller gurantees the keymap was initialized.
		unsafe { (*ptr).assume_init_ref() }
	};

	active_keymap.keymap_0[index]
}
