use quote::ToTokens;
use syn::{Expr, ExprArray};

pub fn get_keymap_size(layout_keymap: &ExprArray) -> Result<usize, syn::Error> {
	let mut size = 0_usize;

	for row_expr in &layout_keymap.elems {
		let Expr::Array(arr_expr) = row_expr else {
			return Err(syn::Error::new_spanned(row_expr, "Duplicate cols field."));
		};

		for key_expr in &arr_expr.elems {
			let Ok(literal_number) = key_expr.to_token_stream().to_string().parse::<u8>() else {
				return Err(syn::Error::new_spanned(
					key_expr,
					"Expected literal u8 value in layout row.",
				));
			};

			if literal_number != 0 {
				size += 1;
			}
		}
	}

	Ok(size)
}
