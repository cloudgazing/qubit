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

use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::{Expr, ExprArray, parse_macro_input};

mod input;
mod row_col;

use input::{MatrixInput, MatrixInputWithEnv};

pub fn define_pin_matrix_macro(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as MatrixInput);

	define_pin_matrix(&ast.rows, &ast.cols, &ast.layout)
}

pub fn define_pin_matrix_env_keymap_macro(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as MatrixInputWithEnv);

	let Ok(layout_str) = std::env::var(ast.layout_env.to_string()) else {
		panic!("Missing env value {}", ast.layout_env);
	};

	let layout: ExprArray = syn::parse_str(&layout_str).expect("Failed to parse str into an array expr");

	define_pin_matrix(&ast.rows, &ast.cols, &layout)
}

fn define_pin_matrix(rows: &ExprArray, cols: &ExprArray, layout: &ExprArray) -> TokenStream {
	let row_len = rows.elems.len();
	let col_len = cols.elems.len();

	row_col::validate_layout(row_len, col_len, layout);

	// Include needed imports.
	let imports_tokens = quote! {
		use hal::gpio::{Pin, FunctionSio, SioInput, SioOutput, PullUp, PullDown, PinState, FunctionNull};
		use embedded_hal::digital::{InputPin, OutputPin};
	};

	// Generate the struct definition.
	let definition_tokens = get_fn_definition(rows, cols);

	// Generate struct implemenation.
	let impl_tokens = {
		let new_method = get_new_method(rows, cols);
		let generate_key_report_method = get_generate_key_report_method(rows, layout);

		quote! {
			impl KeyboardMatrix {
				#new_method
				#generate_key_report_method
			}
		}
	};

	quote! {
		#imports_tokens
		#definition_tokens
		#impl_tokens
	}
	.into()
}

/// Generates the basic struct definition.
fn get_fn_definition(rows: &ExprArray, cols: &ExprArray) -> proc_macro2::TokenStream {
	let row_fields = row_col::map_row_fields(rows);
	let col_fields = row_col::map_col_fields(cols);

	quote! {
		pub struct KeyboardMatrix {
			#row_fields
			#col_fields
		}
	}
}

/// Generates the new method used to construct the matrix.
fn get_new_method(rows: &ExprArray, cols: &ExprArray) -> proc_macro2::TokenStream {
	let (row_args, row_fields) = row_col::map_rows_new(rows);
	let (col_args, col_fields) = row_col::map_cols_new(cols);

	quote! {
		#[must_use]
		pub fn new(rows: #row_args, cols: #col_args) -> Self {
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
fn get_generate_key_report_method(rows: &ExprArray, layout: &ExprArray) -> proc_macro2::TokenStream {
	let row_len = rows.elems.len();

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

				quote! {
					// check the col here and store the position if the pin is low
					if self.#col_name.is_low().unwrap() {
						is_pressed = true;
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

		// We add some delay after each row check, skipping the last one.
		let delay = if row_index == row_len - 1 {
			// add a delay of 1ms or something here
			quote! {}
		} else {
			quote! {}
		};

		quote! {
			self.#row_name.set_low().unwrap();

			#(#check_col_keys)*

			self.#row_name.set_high().unwrap();

			#delay
		}
	});

	quote! {
		#[doc = r"Checks every pin looking for pressed keys and returns a `KeyboardReport`."]
		pub fn generate_key_report(&mut self) -> Option<KeyboardReport> {
			// for now I just set is_pressed to true, but I need to store the coordinates
			// for the pressed key somehow and at the end return them
			let mut is_pressed = false;

			#(#check_keys)*

			if is_pressed { Some(get_key_report()) } else { None }
		}
	}
}
