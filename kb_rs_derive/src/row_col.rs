use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ExprArray;

pub fn map_row_fields(rows: &ExprArray) -> TokenStream {
	let map = rows.elems.iter().enumerate().map(|(i, e)| {
		let field_name = format_ident!("row_{i}");
		quote! { #field_name: Pin<#e, FunctionSio<SioInput>, PullUp> }
	});

	quote! { #(#map,)* }
}

pub fn map_rows_new(rows: &ExprArray) -> (TokenStream, TokenStream) {
	let args = {
		let iter = rows.elems.iter();

		quote! { (#(Pin<#iter, FunctionNull, PullDown>),*) }
	};

	let row_fields_init = {
		let map = (0..rows.elems.len()).map(|i| {
			let name = format_ident!("row_{i}");
			let index = syn::Index::from(i);
			quote! { #name: rows.#index.into_pull_up_input() }
		});

		quote! { #(#map,)* }
	};

	(args, row_fields_init)
}

pub fn map_col_fields(cols: &ExprArray) -> TokenStream {
	let map = cols.elems.iter().enumerate().map(|(i, e)| {
		let field_name = format_ident!("col_{i}");
		quote! { #field_name: Pin<#e, FunctionSio<SioOutput>, PullDown> }
	});

	quote! { #(#map,)* }
}

pub fn map_cols_new(cols: &ExprArray) -> (TokenStream, TokenStream) {
	let args = {
		let iter = cols.elems.iter();

		quote! { (#(Pin<#iter, FunctionNull, PullDown>),*) }
	};

	let col_fields_init = {
		let map = (0..cols.elems.len()).map(|i| {
			let name = format_ident!("col_{i}");
			let index = syn::Index::from(i);
			quote! { #name: cols.#index.into_push_pull_output_in_state(PinState::High) }
		});

		quote! { #(#map,)* }
	};

	(args, col_fields_init)
}
