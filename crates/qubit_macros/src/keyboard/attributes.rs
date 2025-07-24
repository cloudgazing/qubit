use std::str::FromStr;

use qubit_config::mcu::Mcu;
use syn::spanned::Spanned;
use syn::{Expr, ExprArray, Ident, Lit, LitInt, LitStr, Token};

#[derive(Debug)]
pub enum Direction {
	RowCol,
	ColRow,
}

impl FromStr for Direction {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"RowCol" => Ok(Self::RowCol),
			"ColRow" => Ok(Self::ColRow),
			_ => Err("Unknown direction. Supported values are `RowCol` and `ColRow`"),
		}
	}
}

#[derive(Debug)]
pub struct KeymapExpr {
	pub keymap: Vec<Vec<u8>>,
}

impl KeymapExpr {
	fn from_arr_expr(arr_expr: ExprArray) -> Result<Self, syn::Error> {
		let mut expected_col_len: Option<usize> = None;

		let keymap: Vec<Vec<u8>> = arr_expr
			.elems
			.into_iter()
			.map(|row| {
				let Expr::Array(row_expr) = row else {
					return Err(syn::Error::new(row.span(), "Expected array expression."));
				};

				let row_expr_span = row_expr.span();

				let row: Vec<u8> = row_expr
					.elems
					.into_iter()
					.map(|key| {
						let Expr::Lit(lit_expr) = key else {
							return Err(syn::Error::new(key.span(), "Expected literal expression."));
						};

						let value: u8 = match lit_expr.lit {
							Lit::Int(value) => value.base10_parse()?,
							_ => {
								return Err(syn::Error::new(lit_expr.span(), "Expected literal int expression."));
							}
						};

						Ok(value)
					})
					.collect::<Result<_, syn::Error>>()?;

				if let Some(expected_len) = expected_col_len {
					if row.len() != expected_len {
						let err_msg = format!("Array lengths don't match. Expected length of {expected_len}.");

						return Err(syn::Error::new(row_expr_span, err_msg));
					}
				} else {
					expected_col_len = Some(row.len());
				}

				Ok(row)
			})
			.collect::<Result<_, syn::Error>>()?;

		Ok(Self { keymap })
	}
}

#[derive(Debug)]
pub struct Attributes {
	pub delay: u32,
	pub mcu: Mcu,
	pub rows: ExprArray,
	pub cols: ExprArray,
	pub keymap: KeymapExpr,
	pub direction: Direction,
}

impl syn::parse::Parse for Attributes {
	fn parse(stream: syn::parse::ParseStream) -> Result<Self, syn::Error> {
		let mut delay: Option<u32> = None;
		let mut mcu: Option<Mcu> = None;
		let mut rows: Option<ExprArray> = None;
		let mut cols: Option<ExprArray> = None;
		let mut keymap: Option<KeymapExpr> = None;
		let mut direction: Option<Direction> = None;

		while !stream.is_empty() {
			let key: Ident = stream.parse()?;

			stream.parse::<Token![=]>()?;

			match key.to_string().as_str() {
				"delay" => {
					if delay.is_some() {
						return Err(syn::Error::new(key.span(), "Keyword argument repeated: `delay`."));
					}

					let value: LitInt = stream.parse()?;

					delay = Some(value.base10_parse()?);
				}
				"mcu" => {
					if mcu.is_some() {
						return Err(syn::Error::new(key.span(), "Keyword argument repeated: `mcu`."));
					}

					let lit: LitStr = stream.parse()?;
					let value =
						Mcu::from_str(&lit.value()).map_err(|_| syn::Error::new(lit.span(), "Unsupported mcu."))?;

					mcu = Some(value);
				}
				"rows" => {
					if rows.is_some() {
						return Err(syn::Error::new(key.span(), "Keyword argument repeated: `rows`."));
					}

					rows = Some(stream.parse()?);
				}
				"cols" => {
					if cols.is_some() {
						return Err(syn::Error::new(key.span(), "Keyword argument repeated: `cols`."));
					}

					cols = Some(stream.parse()?);
				}
				"keymap" => {
					if keymap.is_some() {
						return Err(syn::Error::new(key.span(), "Keyword argument repeated: `keymap`."));
					}

					let value = KeymapExpr::from_arr_expr(stream.parse()?)?;

					keymap = Some(value);
				}
				"direction" => {
					if direction.is_some() {
						return Err(syn::Error::new(key.span(), "Keyword argument repeated: `direction`."));
					}

					let ident: Ident = stream.parse()?;
					let value =
						Direction::from_str(&ident.to_string()).map_err(|s| syn::Error::new(ident.span(), s))?;

					direction = Some(value);
				}
				_ => {
					return Err(syn::Error::new(key.span(), "Unexpected keyword argument."));
				}
			}

			if stream.peek(Token![,]) {
				stream.parse::<Token![,]>()?;
			} else {
				break;
			}
		}

		let delay = delay.unwrap_or(40);
		let mcu = mcu.ok_or(syn::Error::new(stream.span(), "Missing `mcu` argument."))?;
		let rows = rows.ok_or(syn::Error::new(stream.span(), "Missing `rows` argument."))?;
		let cols = cols.ok_or(syn::Error::new(stream.span(), "Missing `cols` argument."))?;
		let keymap = keymap.ok_or(syn::Error::new(stream.span(), "Missing `keymap` argument."))?;
		let direction = direction.unwrap_or(Direction::ColRow);

		Ok(Self {
			delay,
			mcu,
			rows,
			cols,
			keymap,
			direction,
		})
	}
}
