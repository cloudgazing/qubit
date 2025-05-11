use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{ExprArray, Token, parse::Parse, parse_macro_input};

struct MatrixInput {
	rows: ExprArray,
	_comma: Token![,],
	cols: ExprArray,
}

impl Parse for MatrixInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(MatrixInput {
			rows: input.parse()?,
			_comma: input.parse()?,
			cols: input.parse()?,
		})
	}
}

pub(crate) fn define_matrix_macro(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as MatrixInput);

	let imports_tokens = quote! {
		use hal::gpio::{Pin, FunctionSio, SioInput, SioOutput, PullUp, PullDown, PinState, FunctionNull};
		use embedded_hal::digital::{InputPin, OutputPin};
	};

	let definition_tokens = {
		let row_fields = crate::row_col::map_row_fields(&ast.rows);
		let col_fields = crate::row_col::map_col_fields(&ast.cols);

		quote! {
			pub struct KeyboardMatrix {
				#row_fields
				#col_fields
			}
		}
	};

	let impl_tokens = {
		let (row_args, row_fields) = crate::row_col::map_rows_new(&ast.rows);
		let (col_args, col_fields) = crate::row_col::map_cols_new(&ast.cols);

		// Check on each row if the input is low
		let _row_iter_tokens = (0..ast.rows.elems.len()).map(|i| {
			let row_field = format_ident!("self.row_{i}");

			quote! {
				let is_low = #row_field.is_low();

			}
		});

		// Drive each column to low, check each row then drive the column back to high
		let _col_iter_tokens = (0..ast.cols.elems.len()).map(|i| {
			let col_field = format_ident!("self.col_{i}");

			quote! {
				#col_field.set_low().unwrap()



				#col_field.set_high().unwrap()
			}
		});

		quote! {
			impl KeyboardMatrix {
				#[must_use]
				pub fn new(rows: #row_args, cols: #col_args) -> Self {
					Self {
						#row_fields
						#col_fields
					}
				}

				pub fn generate_key_report(&mut self) -> Option<KeyboardReport> {
					// #(#row_iter_tokens)*
					
					// This is currently used for debugging.
					let is_pressed = self.row_2.is_low().unwrap();
					if is_pressed { Some(get_key_report()) } else { None }
				}
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
