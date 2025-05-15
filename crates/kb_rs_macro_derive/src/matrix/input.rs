use syn::{ExprArray, Ident, Token, parse::Parse};

pub struct MatrixInput {
	pub rows: ExprArray,
	pub cols: ExprArray,
	pub layout: ExprArray,
}

impl Parse for MatrixInput {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let rows = input.parse()?;
		input.parse::<Token![,]>()?;
		let cols = input.parse()?;
		input.parse::<Token![,]>()?;
		let layout = input.parse()?;

		Ok(Self { rows, cols, layout })
	}
}

pub struct MatrixInputWithEnv {
	pub rows: ExprArray,
	pub cols: ExprArray,
	pub layout_env: Ident,
}

impl Parse for MatrixInputWithEnv {
	fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let rows = input.parse()?;
		input.parse::<Token![,]>()?;
		let cols = input.parse()?;
		input.parse::<Token![,]>()?;
		let layout_env = input.parse()?;

		Ok(Self { rows, cols, layout_env })
	}
}
