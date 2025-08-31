use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;
use syn::{Ident, Token};

struct Input {
	author: Ident,
	model: Ident,
}

impl syn::parse::Parse for Input {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let author_ident: Ident = input.parse()?;

		input.parse::<Token![,]>()?;

		let model_ident: Ident = input.parse()?;

		let author = match std::env::var(author_ident.to_string()) {
			Ok(value) => Ident::new(&value, author_ident.span()),
			Err(e) => {
				let msg = format!("Failed to read env value {author_ident}. {e}");

				return Err(syn::Error::new(author_ident.span(), msg));
			}
		};

		let model = match std::env::var(model_ident.to_string()) {
			Ok(value) => Ident::new(&value, model_ident.span()),
			Err(e) => {
				let msg = format!("Failed to read env value {model_ident}. {e}");

				return Err(syn::Error::new(model_ident.span(), msg));
			}
		};

		Ok(Self { author, model })
	}
}

pub fn import_device_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Input);

	let author = input.author;
	let model = input.model;

	quote! { pub use ::qubit_device::models::#author::#model as device; }.into()
}
