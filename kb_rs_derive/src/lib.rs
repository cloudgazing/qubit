#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

use rp2040_hal as _;

use proc_macro::TokenStream;

mod matrix;
mod row_col;

#[proc_macro]
pub fn define_keyboard_matrix(input: TokenStream) -> TokenStream {
	matrix::define_matrix_macro(input)
}
