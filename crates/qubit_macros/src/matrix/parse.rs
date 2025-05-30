use std::str::FromStr;

use syn::parenthesized;
use syn::{ExprArray, Ident, LitInt, LitStr, Token, TypeTuple};

#[derive(Debug)]
pub enum Hal {
	Rp2040hal,
}

impl FromStr for Hal {
	type Err = syn::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"rp2040hal" => Ok(Self::Rp2040hal),
			_ => Err(syn::Error::new(
				proc_macro2::Span::call_site(),
				format!("Unknown hal: {s}."),
			)),
		}
	}
}

#[derive(Debug, Default)]
pub struct EnvValues {
	pub rows: Option<String>,
	pub cols: Option<String>,
	pub layout: Option<String>,
}

impl syn::parse::Parse for EnvValues {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let content;
		parenthesized!(content in input);

		let mut rows: Option<String> = None;
		let mut cols: Option<String> = None;
		let mut layout: Option<String> = None;

		while !content.is_empty() {
			let ident: Ident = content.parse()?;

			content.parse::<Token![=]>()?;

			let val: LitStr = content.parse()?;

			match ident.to_string().as_str() {
				"rows" => {
					if rows.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate rows value."));
					}

					rows = Some(val.value());
				}
				"cols" => {
					if cols.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate cols value."));
					}

					cols = Some(val.value());
				}
				"layout" => {
					if layout.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate layout value."));
					}

					layout = Some(val.value());
				}
				_ => {
					return Err(syn::Error::new_spanned(ident, "Unexpected env field."));
				}
			}

			if content.peek(Token![,]) {
				content.parse::<Token![,]>()?;
			} else {
				break;
			}
		}

		Ok(Self { rows, cols, layout })
	}
}

#[derive(Debug)]
pub struct Attributes {
	pub hal: Hal,
	pub delay: u32,
	pub env: EnvValues,
}

impl syn::parse::Parse for Attributes {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let input_span = input.span();

		let mut hal: Option<Hal> = None;
		let mut delay: Option<u32> = None;
		let mut env = EnvValues::default();

		let mut env_already_provided = false;

		while !input.is_empty() {
			let ident: Ident = input.parse()?;

			match ident.to_string().as_str() {
				"hal" => {
					if hal.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate hal value."));
					}

					input.parse::<Token![=]>()?;

					let val: LitStr = input.parse()?;

					hal = Some(Hal::from_str(&val.value())?);
				}
				"delay" => {
					if delay.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate delay value."));
					}

					input.parse::<Token![=]>()?;

					let val: LitInt = input.parse()?;

					delay = Some(val.base10_parse()?);
				}
				"env" => {
					if env_already_provided {
						return Err(syn::Error::new_spanned(ident, "Duplicate env value."));
					}

					env = input.parse()?;

					env_already_provided = true;
				}
				_ => {
					return Err(syn::Error::new_spanned(ident, "Unexpected argument."));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			} else {
				break;
			}
		}

		let hal = hal.ok_or(syn::Error::new(input_span, "Missing hal value."))?;
		let delay = delay.unwrap_or(35);

		Ok(Self { hal, delay, env })
	}
}

#[derive(Debug, Default)]
pub struct ParsedFields {
	pub rows: Option<TypeTuple>,
	pub cols: Option<TypeTuple>,
	pub layout: Option<TypeTuple>,
}

impl syn::parse::Parse for ParsedFields {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let mut rows: Option<TypeTuple> = None;
		let mut cols: Option<TypeTuple> = None;
		let mut layout: Option<TypeTuple> = None;

		while !input.is_empty() {
			let ident: Ident = input.parse()?;

			input.parse::<Token![:]>()?;

			let val: TypeTuple = input.parse()?;

			match ident.to_string().as_str() {
				"rows" => {
					if rows.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate rows field."));
					}

					rows = Some(val);
				}
				"cols" => {
					if cols.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate cols field."));
					}

					cols = Some(val);
				}
				"layout" => {
					if layout.is_some() {
						return Err(syn::Error::new_spanned(ident, "Duplicate layout field."));
					}

					layout = Some(val);
				}
				_ => {
					return Err(syn::Error::new_spanned(ident, "Unexpected field."));
				}
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			} else {
				break;
			}
		}

		Ok(Self { rows, cols, layout })
	}
}

pub fn parse_env_val_to_expr_arr(key: &str) -> Result<ExprArray, syn::Error> {
	let layout_str = std::env::var(key)
		.map_err(|_| syn::Error::new(proc_macro2::Span::call_site(), format!("Missing env value {key}")))?;

	syn::parse_str(&layout_str)
}
