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

		let Ok(author_value) = std::env::var(author_ident.to_string()) else {
			let err_msg = format!("Failed to read env value {author_ident}");
			return Err(syn::Error::new(author_ident.span(), err_msg));
		};

		let author = Ident::new(&author_value, author_ident.span());

		let Ok(model_value) = std::env::var(model_ident.to_string()) else {
			let err_msg = format!("Failed to read env value {model_ident}");
			return Err(syn::Error::new(model_ident.span(), err_msg));
		};

		let model = Ident::new(&model_value, model_ident.span());

		Ok(Self { author, model })
	}
}

pub fn import_device_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Input);

	let author = input.author;
	let model = input.model;

	quote! { pub use ::qubit_device::models::#author::#model as device; }.into()
}
