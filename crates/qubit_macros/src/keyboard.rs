use proc_macro::TokenStream;

use qubit_config::general::Processor;
use quote::quote;
use syn::{ItemStruct, Visibility, parse_macro_input};

mod attributes;
mod fields;

use attributes::{Attributes, Direction, KeymapExpr};

type FieldNameFn = fn(usize) -> proc_macro2::Ident;

pub fn keyboard_matrix_macro(args: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);

	let attrs = parse_macro_input!(args as Attributes);

	let visibility = input.vis;
	let struct_name = input.ident;

	let delay = attrs.delay;
	let processor = attrs.processor;
	let rows = attrs.rows;
	let cols = attrs.cols;
	let keymap = attrs.keymap;
	let direction = attrs.direction;

	let struct_definition = {
		let row_fields = fields::map_row_fields(processor, &direction, &rows);
		let col_fields = fields::map_col_fields(processor, &direction, &cols);

		quote! {
			#visibility struct #struct_name {
				#row_fields
				#col_fields
			}
		}
	};

	let struct_impl = {
		let new_method = {
			let row_args = fields::map_new_args(processor, &rows);
			let col_args = fields::map_new_args(processor, &cols);

			let row_fields = fields::map_rows_new(processor, &direction, &rows);
			let col_fields = fields::map_cols_new(processor, &direction, &cols);

			quote! {
				#[must_use]
				#visibility fn new(rows: #row_args, cols: #col_args) -> Self {
					Self {
						#row_fields
						#col_fields
					}
				}
			}
		};

		let pressed_keys_method = def_pressed_keys_method(delay, processor, &keymap, &direction, &visibility);

		quote! {
			impl #struct_name {
				#new_method
				#pressed_keys_method
			}
		}
	};

	quote! {
		#struct_definition
		#struct_impl
	}
	.into()
}

/// Generates the implementation of the `get_pressed_keys` method.
///
/// This method scans a key matrix connected to GPIO pins and returns a compact
/// bitmap of pressed key positions. The layout is defined in the provided keymap,
/// where positions with `0x00` are skipped as they represent unused keys.
///
/// For each drive pin (row or column depending on scanning direction), it is set
/// low, a short delay is applied, and then the sense pins are checked. If a sense
/// pin reads low while the drive pin is active, it indicates a key is pressed
/// at that matrix position.
fn def_pressed_keys_method(
	delay: u32,
	processor: Processor,
	keymap: &KeymapExpr,
	direction: &Direction,
	visibility: &Visibility,
) -> proc_macro2::TokenStream {
	// Bring in scope the traits needed.
	let imports = match processor {
		Processor::RP2040 => {
			quote! { use ::embedded_hal::digital::{InputPin as _, OutputPin as _}; }
		}
	};

	// Get the total amount of keys, excluding the empty spaces.
	let keys_count: usize = keymap.keymap.iter().flatten().filter(|&&x| x != 0).count();

	let delay_call = match processor {
		Processor::RP2040 => quote! { ::cortex_m::asm::delay(#delay); },
	};

	// Walk the keymap and assign a sequential bit index to each key that's not empty (0x00),
	// building a 2D map of bit positions for generating the key bitmap.
	let mut bit_position: usize = 0;
	let bit_pos_map: Vec<Vec<Option<usize>>> = keymap
		.keymap
		.iter()
		.map(|row| {
			row.iter()
				.map(|key| {
					if *key != 0 {
						let pos = Some(bit_position);

						bit_position += 1;

						pos
					} else {
						None
					}
				})
				.collect()
		})
		.collect();

	// Abstract away from row and col to `drive` and `sense`.
	let (drive_len, sense_len, get_drive_name, get_sense_name) = match direction {
		Direction::RowCol => (
			keymap.keymap[0].len(),
			keymap.keymap.len(),
			fields::col_field_name as FieldNameFn,
			fields::row_field_name as FieldNameFn,
		),
		Direction::ColRow => (
			keymap.keymap.len(),
			keymap.keymap[0].len(),
			fields::row_field_name as FieldNameFn,
			fields::col_field_name as FieldNameFn,
		),
	};

	let check_tokens = (0..drive_len).map(|drive_idx| {
		let check_sense_lines = (0..sense_len).map(|sense_idx| {
			let (row_idx, col_idx) = match direction {
				Direction::RowCol => (sense_idx, drive_idx),
				Direction::ColRow => (drive_idx, sense_idx),
			};

			if let Some(pos) = bit_pos_map[row_idx][col_idx] {
				let sense_name = get_sense_name(sense_idx);

				quote! {
					if self.#sense_name.is_low().unwrap() {
						bitmaps[const { #pos / USIZE_BITS }] |= 1 << const { #pos % USIZE_BITS };
					}
				}
			} else {
				quote! {}
			}
		});

		let check_sense_lines: Vec<_> = check_sense_lines.collect();

		if check_sense_lines.is_empty() {
			// Ideally I'd like to emit a warning if all sense lines are empty
			// but for now I don't know if there is a way to emit warnings during
			// compilation.

			quote! {}
		} else {
			let drive_name = get_drive_name(drive_idx);

			quote! {
				self.#drive_name.set_low().unwrap();

				#delay_call

				#(#check_sense_lines)*

				self.#drive_name.set_high().unwrap();
			}
		}
	});

	let bitmaps_count = quote! { #keys_count.div_ceil(usize::BITS as usize) };

	quote! {
		#[doc = r"Scans the key matrix and returns a bitmap of pressed keys."]
		#visibility fn get_pressed_keys(&mut self) -> [usize; #bitmaps_count] {
			#imports

			const USIZE_BITS: usize = usize::BITS as usize;

			let mut bitmaps = [0_usize; #bitmaps_count];

			#(#check_tokens)*

			bitmaps
		}
	}
}
