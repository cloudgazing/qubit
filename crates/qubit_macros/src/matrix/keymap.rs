use quote::ToTokens;
use syn::{Expr, ExprArray};

pub fn get_keymap_size(layout_keymap: &ExprArray) -> usize {
	let mut size = 0_usize;

	for row_expr in &layout_keymap.elems {
		let Expr::Array(arr_expr) = row_expr else {
			panic!("Expected array expression.");
		};

		for key_expr in &arr_expr.elems {
			let Ok(literal_number) = key_expr.to_token_stream().to_string().parse::<u8>() else {
				panic!("Expected literal u8 value in layout row.");
			};

			if literal_number != 0 {
				size += 1;
			}
		}
	}

	size
}
