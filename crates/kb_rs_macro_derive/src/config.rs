use proc_macro::TokenStream;

use quote::quote;
use syn::{ExprArray, Ident, Token, parse::Parse, parse_macro_input};

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

pub fn define_keyboard_configuration_macro(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as ConfigInput);

	let Ok(name) = std::env::var(ast.name.to_string()) else {
		panic!("Missing env value {}.", ast.name);
	};
	let Ok(author) = std::env::var(ast.author.to_string()) else {
		panic!("Missing env value {}.", ast.author);
	};
	let Ok(id) = std::env::var(ast.id.to_string()) else {
		panic!("Missing env value {}.", ast.id);
	};
	let Ok(version) = std::env::var(ast.version.to_string()) else {
		panic!("Missing env value {}.", ast.version);
	};
	let Ok(keymap) = std::env::var(ast.keymap.to_string()) else {
		panic!("Missing env value {}.", ast.keymap);
	};

	let keymap: ExprArray = syn::parse_str(&keymap).expect("Failed to parse str into an array expr");

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
