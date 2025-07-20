//! Procedural macros for the Qubit firmware.

#![warn(missing_docs)]

use proc_macro::TokenStream;

#[cfg(feature = "all")]
mod date;
#[cfg(feature = "import")]
mod import;
#[cfg(feature = "all")]
mod keyboard;

/// This attribute macro generates a struct representing the keyboard's GPIO pin matrix,
/// along with a `new` method and a `get_pressed_keys` method for scanning key states.
///
/// # Attributes
///
/// The macro takes the following arguments:
///
/// - `processor` *(required)*: The target microcontroller (e.g., `"RP2040"`).
/// - `keymap` *(required)*: A 2D array of HID keycodes that defines the layout.
/// - `rows` *(required)*: An array of GPIO pin numbers used as rows.
/// - `cols` *(required)*: An array of GPIO pin numbers used as columns.
/// - `direction` *(optional)*: Scanning direction, either `"RowCol"` or `"ColRow"`. Defaults to `"ColRow"`.
/// - `delay` *(optional)*: Delay in microseconds between column/row scans. Defaults to `40`.
///
/// # Example
///
/// For a struct like:
///
/// ```
/// #[qubit_macros::keyboard_pin_matrix(
///     processor = "RP2040",
///     keymap = [[41, 30, 31], [20, 4, 5]],
///     rows = [2, 3],
///     cols = [0, 1, 4]
/// )]
/// pub struct KeyboardMatrix;
/// ```
///
/// the macro expands to:
///
/// - A struct `KeyboardMatrix` containing named GPIO pins: `row_0`, `row_1`, ..., `col_0`, `col_1`, etc.
/// - A `fn new(rows: (...), cols: (...)) -> Self` that initializes the pins into correct modes.
/// - A `fn get_pressed_keys(&mut self) -> [usize; N]` method that returns a compressed bitmap
///   of pressed key positions based on scanning the matrix.
///
/// # Notes
///
/// - All pin numbers passed to this macro must be unique.
/// - The number of bitmaps in the array depends on the ammount of keys and the target's word size.
#[cfg(feature = "all")]
#[proc_macro_attribute]
pub fn keyboard_matrix(args: TokenStream, item: TokenStream) -> TokenStream {
	keyboard::keyboard_matrix_macro(args, item)
}

/// Expands to a 16-bit integer representing the current UTC date.
///
/// The bits are packed as follows:
/// - Bits 0–4: Day (1–31)
/// - Bits 5–8: Month (1–12)
/// - Bits 9–15: Years since 2024
///
/// # Example
///
/// ```
/// const BUILD_DATE: u16 = date_now_bitmap!();
/// ```
#[cfg(feature = "all")]
#[proc_macro]
pub fn build_date_bitmap(_input: TokenStream) -> TokenStream {
	date::build_date_bitmap_macro()
}

/// Includes a device module based on environment variables.
///
/// This macro takes two identifiers corresponding to environment variables
/// that define the device author and model. It generates a `pub use` statement
/// that imports the appropriate module from `qubit_device`.
///
/// # Example
///
/// ```rust
/// include_device_mod!(QUBIT_AUTHOR, QUBIT_MODEL);
/// ```
///
/// Given the following in `config.toml`:
///
/// ```toml
/// [env]
/// QUBIT_AUTHOR = "cloudgazing"
/// QUBIT_MODEL = "quartz"
/// ```
///
/// This expands to:
///
/// ```rust
/// pub use ::qubit_device::models::cloudgazing::quartz as device;
/// ```
#[cfg(feature = "import")]
#[proc_macro]
pub fn import_device(input: TokenStream) -> TokenStream {
	import::import_device_macro(input)
}
