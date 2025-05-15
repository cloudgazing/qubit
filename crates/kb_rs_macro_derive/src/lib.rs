use proc_macro::TokenStream;

mod config;
mod matrix;

#[proc_macro]
pub fn define_pin_matrix(input: TokenStream) -> TokenStream {
	matrix::define_pin_matrix_macro(input)
}

#[proc_macro]
pub fn define_pin_matrix_env_keymap(input: TokenStream) -> TokenStream {
	matrix::define_pin_matrix_env_keymap_macro(input)
}

#[proc_macro]
pub fn define_configuration(input: TokenStream) -> TokenStream {
	config::define_keyboard_configuration_macro(input)
}
