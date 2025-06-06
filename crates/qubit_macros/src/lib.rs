use proc_macro::TokenStream;

mod config;
mod hid;
mod matrix;

#[proc_macro]
pub fn define_configuration(input: TokenStream) -> TokenStream {
	config::define_keyboard_configuration_macro(input)
}

#[proc_macro_attribute]
pub fn kb_pin_matrix(args: TokenStream, item: TokenStream) -> TokenStream {
	matrix::kb_pin_matrix_macro(args, item)
}

#[proc_macro]
pub fn date_now_bitmap(_input: TokenStream) -> TokenStream {
	hid::date_now_bitmap_macro()
}
