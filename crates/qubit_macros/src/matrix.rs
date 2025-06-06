//! Generate `KeyboardMatrix` struct
//!
//! # Example:
//!
//! ```
//! // Given 'n' rows and 'm' columns the following struct will be generated:
//!
//! pub struct KeyboardMatrix {
//!   row_0: Pin<PIN_ID, FunctionSio<SioInput>, PullUp>,
//!   row_1: Pin<PIN_ID, FunctionSio<SioInput>, PullUp>,
//!   row_2: Pin<PIN_ID, FunctionSio<SioInput>, PullUp>,
//!   ...
//!   row_n: Pin<PIN_ID, FunctionSio<SioInput>, PullUp>,
//!   col_0: Pin<PIN_ID, FunctionSio<SioOutput>, PullDown>,
//!   col_1: Pin<PIN_ID, FunctionSio<SioOutput>, PullDown>,
//!   col_2: Pin<PIN_ID, FunctionSio<SioOutput>, PullDown>,
//!   ...
//!   col_m: Pin<PIN_ID, FunctionSio<SioOutput>, PullDown>,
//! }
//!
//! impl KeyboardMatrix {
//!   #[must_use]
//!   pub fn new(rows: (r0, r1, r2 ... rn), cols: (c0, c1, c2 ... cm)) -> Self {
//!     Self {
//!       row_0: rows.0.into_push_pull_output_in_state(PinState::High),
//!       row_1: rows.1.into_push_pull_output_in_state(PinState::High),
//!       row_2: rows.2.into_push_pull_output_in_state(PinState::High),
//!       ...
//!       row_m: rows.m.into_push_pull_output_in_state(PinState::High),
//!       col_0: cols.0.into_pull_up_input(),
//!       col_1: cols.1.into_pull_up_input(),
//!       col_2: cols.2.into_pull_up_input(),
//!       ...
//!       col_n: cols.n.into_pull_up_input(),
//!     }
//!   }
//! }
//!
//! ```

use parse::{Hal, ParsedFields};
use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::{Expr, ExprArray, Ident, ItemStruct, Visibility};
use syn::{ExprTuple, parse_macro_input};

mod keymap;
mod parse;
mod row_col;

pub fn kb_pin_matrix_macro(args: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as ItemStruct);

	let parsed_attrs = parse_macro_input!(args as parse::Attributes);
	let parsed_fields = match input.fields {
		syn::Fields::Named(named_fields) => {
			let tokens: TokenStream = named_fields.named.into_token_stream().into();

			parse_macro_input!(tokens as ParsedFields)
		}
		_ => parse::ParsedFields::default(),
	};

	// TODO: Experiment with the delay and find the right value.
	// For now a min of 35 cycles works.
	// let delay_value = parsed_attrs.delay.unwrap_or(35);
	let delay_value = parsed_attrs.delay;

	let rows = {
		let mut rows: Option<ExprArray> = None;

		if let Some(rows_type) = parsed_fields.rows {
			let tuple_tokens = rows_type.into_token_stream();
			let tuple_expr: ExprTuple = syn::parse2(tuple_tokens).unwrap();

			let arr_expr = ExprArray {
				attrs: tuple_expr.attrs,
				bracket_token: syn::token::Bracket::default(),
				elems: tuple_expr.elems,
			};

			rows = Some(arr_expr);
		}

		if let Some(env_key) = parsed_attrs.env.rows {
			let rows_arr = parse::parse_env_val_to_expr_arr(&env_key).unwrap();

			let prev_value = rows.replace(rows_arr);

			assert!(prev_value.is_none(), "'rows' field already provided.");
		}

		rows.expect("Missing cols definition.")
		// rows.ok_or(syn::Error::new(proc_macro2::Span::call_site(), "Missing rows definition.")).unwrap()
	};

	let cols = {
		let mut cols: Option<ExprArray> = None;

		if let Some(cols_type) = parsed_fields.cols {
			let tuple_tokens = cols_type.into_token_stream();
			let tuple_expr: ExprTuple = syn::parse2(tuple_tokens).unwrap();

			let arr_expr = ExprArray {
				attrs: tuple_expr.attrs,
				bracket_token: syn::token::Bracket::default(),
				elems: tuple_expr.elems,
			};

			cols = Some(arr_expr);
		}

		if let Some(env_key) = parsed_attrs.env.cols {
			let cols_arr = parse::parse_env_val_to_expr_arr(&env_key).unwrap();

			let prev_value = cols.replace(cols_arr);

			assert!(prev_value.is_none(), "'cols' field already provided.");
		}

		cols.expect("Missing cols definition.")
	};

	let layout = {
		let mut layout: Option<ExprArray> = None;

		if let Some(_layout_type) = parsed_fields.layout {
			todo!();
		}

		if let Some(env_key) = parsed_attrs.env.layout {
			let layout_arr = parse::parse_env_val_to_expr_arr(&env_key).unwrap();

			let prev_value = layout.replace(layout_arr);

			assert!(prev_value.is_none(), "'layout' field already provided.");
		}

		layout.expect("Missing layout definition.")
	};

	row_col::validate_layout(rows.elems.len(), cols.elems.len(), &layout).unwrap();

	let visibility = input.vis;
	let struct_name = input.ident;

	// Include needed imports.
	let imports_tokens = define_imports(&parsed_attrs.hal);

	let struct_definition = define_struct_definition(&visibility, &struct_name, &rows, &cols);

	// Generate struct implemenation.
	let struct_impl = {
		let new_method = define_new_method(&visibility, &rows, &cols);
		let generate_key_report_method = define_get_pressed_keys_method(&visibility, &layout, delay_value).unwrap();

		quote! {
			impl #struct_name {
				#new_method
				#generate_key_report_method
			}
		}
	};

	quote! {
		#imports_tokens
		#struct_definition
		#struct_impl
	}
	.into()
}

fn define_imports(hal: &parse::Hal) -> proc_macro2::TokenStream {
	match hal {
		Hal::Rp2040hal => {
			quote! {
				use ::rp2040_hal::gpio::{FunctionNull, FunctionSio, FunctionSioInput, FunctionSioOutput, Pin, PinState, PullDown, PullUp};
				use ::embedded_hal::digital::{InputPin, OutputPin};
			}
		}
	}
}

/// Generates the basic struct definition.
fn define_struct_definition(
	visibility: &Visibility,
	struct_name: &Ident,
	rows: &ExprArray,
	cols: &ExprArray,
) -> proc_macro2::TokenStream {
	let row_fields = row_col::map_row_fields(rows);
	let col_fields = row_col::map_col_fields(cols);

	quote! {
		#visibility struct #struct_name {
			#row_fields
			#col_fields
		}
	}
}

/// Generates the new method used to construct the matrix.
fn define_new_method(visibility: &Visibility, rows: &ExprArray, cols: &ExprArray) -> proc_macro2::TokenStream {
	let (row_args, row_fields) = row_col::map_rows_new(rows);
	let (col_args, col_fields) = row_col::map_cols_new(cols);

	quote! {
		#[must_use]
		#visibility fn new(rows: #row_args, cols: #col_args) -> Self {
			Self {
				#row_fields
				#col_fields
			}
		}
	}
}

/// Generates the key report method.
///
/// To check which keys are pressed, we drive the row pin to low, check each column pin
/// if it's low, then drive the row pin back up. This is repeated for every row and for
/// efficiency the key positions marked as empty in the layout (0x00) are skipped.
fn define_get_pressed_keys_method(
	visibility: &Visibility,
	layout: &ExprArray,
	delay: u32,
) -> Result<proc_macro2::TokenStream, syn::Error> {
	// Get the total amount of keys and from that find out how many keymaps we need to use.
	let keys_count = keymap::get_keymap_size(layout)?;

	let mut bit_pos = 0_usize;

	let check_keys = layout.elems.iter().enumerate().map(|(row_index, row)| {
		let Expr::Array(cols_arr_expr) = row else {
			panic!("Expected array expression.");
		};

		// We iterate over every column position in the row and skip a check if the literal value is 0.
		let check_col_keys = cols_arr_expr.elems.iter().enumerate().map(|(col_index, col)| {
			let Ok(literal_number) = col.to_token_stream().to_string().parse::<u8>() else {
				panic!("Expected literal u8 value in layout row.");
			};

			if literal_number != 0 {
				let col_name = row_col::col_field_name(col_index);

				let pos = bit_pos;

				bit_pos += 1;

				quote! {
					// check the col here and store the position if the pin is low
					if self.#col_name.is_low().unwrap() {
						bitmaps[const { #pos / SIZE }] |= 1 << const { #pos % SIZE };
					}
				}
			} else {
				quote! {}
			}
		});

		// Ideally I'd like to emit a warning if all columns in that row are empty
		// but for now I don't know if there is a way to emit warnings during
		// compilation.

		let row_name = row_col::row_field_name(row_index);

		quote! {
			self.#row_name.set_low().unwrap();

			::cortex_m::asm::delay(#delay);

			#(#check_col_keys)*

			self.#row_name.set_high().unwrap();
		}
	});

	let tokens = quote! {
		#[doc = r"Checks every pin looking for pressed keys and returns a `KeyboardReport`."]
		#visibility fn get_pressed_keys(&mut self) -> [usize; #keys_count.div_ceil(usize::BITS as usize)] {
			const SIZE: usize = usize::BITS as usize;

			let mut bitmaps = [0_usize; #keys_count.div_ceil(usize::BITS as usize)];

			#(#check_keys)*

			bitmaps
		}
	};

	Ok(tokens)
}
