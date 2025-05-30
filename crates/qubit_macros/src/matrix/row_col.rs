use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ExprArray, Ident};

pub fn row_field_name(index: usize) -> Ident {
	format_ident!("row_{index}")
}

pub fn col_field_name(index: usize) -> Ident {
	format_ident!("col_{index}")
}

pub fn validate_layout(row_len: usize, col_len: usize, layout: &ExprArray) {
	assert_eq!(
		layout.elems.len(),
		row_len,
		"Layout row length doesn't match pin row length."
	);

	for elem in &layout.elems {
		let Expr::Array(array_elem) = elem else {
			panic!("Expected arr expr.");
		};

		assert_eq!(
			array_elem.elems.len(),
			col_len,
			"Layout column length doesn't match pin column length."
		);
	}
}

pub fn map_row_fields(rows: &ExprArray) -> TokenStream {
	let map = rows.elems.iter().enumerate().map(|(i, e)| {
		let field_name = row_field_name(i);

		let doc_string = format!("Pin [`{}`] for row {i}.", quote! { #e });

		quote! {
			#[doc = #doc_string]
			#[doc = ""]
			#[doc = "This acts as a pulled up *output*."]
			#field_name: Pin<#e, FunctionSioOutput, PullDown>
		}
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
			let name = row_field_name(i);
			let index = syn::Index::from(i);
			quote! { #name: rows.#index.into_push_pull_output_in_state(PinState::High) }
		});

		quote! { #(#map,)* }
	};

	(args, row_fields_init)
}

pub fn map_col_fields(cols: &ExprArray) -> TokenStream {
	let map = cols.elems.iter().enumerate().map(|(i, e)| {
		let field_name = col_field_name(i);

		let doc_string = format!("Pin [`{}`] for column {i}.", quote! { #e });

		quote! {
			#[doc = #doc_string]
			#[doc = ""]
			#[doc = "This acts as a pulled up *input*."]
		  #field_name: Pin<#e, FunctionSioInput, PullUp>
		}
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
			let name = col_field_name(i);
			let index = syn::Index::from(i);
			quote! { #name: cols.#index.into_pull_up_input() }
		});

		quote! { #(#map,)* }
	};

	(args, col_fields_init)
}
