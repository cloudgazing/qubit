use core::mem::MaybeUninit;

use crate::Keymap;

static mut ACTIVE_KEYMAP: MaybeUninit<Keymap> = MaybeUninit::uninit();

/// Get the keymap stored in the EEPROM.
///
/// This function is not yet implemented and just returns [`None`].
const fn fetch_stored_keymap() -> Option<Keymap> {
	// Here I need to read from the EEPROM chip.
	let stored_keymap: Option<Keymap> = None;

	stored_keymap
}

/// # Safety
///
/// This function must be called **only once** for the lifetime of the program.
/// Calling it more than once is **undefined behavior** because it writes to static
/// mutable memory without synchronization.
pub const unsafe fn initialize_active_keymap() {
	let keymap = if let Some(keymap) = fetch_stored_keymap() {
		keymap
	} else {
		// Get the default keymap stored in the device configuration.

		// This is highly unsafe and just used for now during testing.
		// TODO: Remove this and add a proper way to get the default keymap.
		let config_location = 0x1000_0000 + 0x100 + (0x0080_0000 - 0x100 - 0x19000);

		let ptr = config_location as *const crate::Configuration;
		let config = unsafe { &*ptr };

		config.keymap
	};

	let ptr = &raw mut ACTIVE_KEYMAP;

	unsafe {
		(*ptr).write(keymap);
	}
}

/// # Safety
///
/// Calling this function before initializing the active keymap is **undefined behavior**.
pub const unsafe fn get_keymap_keycode(index: usize) -> u8 {
	let active_keymap = {
		let ptr = &raw mut ACTIVE_KEYMAP;

		// SAFETY: The caller gurantees the keymap was initiated.
		unsafe { (*ptr).assume_init_ref() }
	};

	active_keymap[index]
}
