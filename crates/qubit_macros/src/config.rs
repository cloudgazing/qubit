use proc_macro::TokenStream;

use quote::quote;
use syn::{ExprArray, Ident, Token, parse::Parse, parse_macro_input};

#[derive(Debug)]
struct ConfigInput {
	name: Ident,
	author: Ident,
	id: Ident,
	version: Ident,
	keymap: Ident,
}

impl Parse for ConfigInput {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let name = input.parse()?;
		input.parse::<Token![,]>()?;
		let author = input.parse()?;
		input.parse::<Token![,]>()?;
		let id = input.parse()?;
		input.parse::<Token![,]>()?;
		let version = input.parse()?;
		input.parse::<Token![,]>()?;
		let keymap = input.parse()?;

		Ok(Self {
			name,
			author,
			id,
			version,
			keymap,
		})
	}
}

fn read_env_value(ident: &Ident) -> Result<String, syn::Error> {
	std::env::var(ident.to_string())
		.map_err(|_| syn::Error::new(proc_macro2::Span::call_site(), format!("Missing env value {ident}")))
}

pub fn define_keyboard_configuration_macro(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as ConfigInput);

	let name = read_env_value(&ast.name).unwrap();
	let author = read_env_value(&ast.author).unwrap();
	let id = read_env_value(&ast.id).unwrap();
	let version = read_env_value(&ast.version).unwrap();
	let keymap = read_env_value(&ast.keymap).unwrap();

	let keymap: ExprArray = syn::parse_str(&keymap).unwrap();

	quote! {
		Configuration {
			name: #name,
			author: #author,
			id: #id,
			version: #version,
			keymap: #keymap,
		}
	}
	.into()
}
