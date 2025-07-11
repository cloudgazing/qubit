use proc_macro2::TokenStream;
use qubit_config::general::Processor;
use quote::{ToTokens, format_ident, quote};
use syn::{Expr, ExprArray, Ident};

use super::attributes::Direction;

pub fn row_field_name(index: usize) -> Ident {
	format_ident!("row_{index}")
}

pub fn col_field_name(index: usize) -> Ident {
	format_ident!("col_{index}")
}

fn input_pin_type(processor: Processor, pin: &Expr) -> TokenStream {
	match processor {
		Processor::RP2040 => {
			let pin = pin.into_token_stream();
			let pin_expr = format_ident!("Gpio{}", pin.to_string());

			quote! {
				::rp2040_hal::gpio::Pin<
					::rp2040_hal::gpio::bank0::#pin_expr,
					::rp2040_hal::gpio::FunctionSioInput,
					::rp2040_hal::gpio::PullUp
				>
			}
		}
	}
}

fn output_pin_type(processor: Processor, pin: &Expr) -> TokenStream {
	match processor {
		Processor::RP2040 => {
			let pin = pin.into_token_stream();
			let pin_expr = format_ident!("Gpio{}", pin.to_string());

			quote! {
				::rp2040_hal::gpio::Pin<
					::rp2040_hal::gpio::bank0::#pin_expr,
					::rp2040_hal::gpio::FunctionSioOutput,
					::rp2040_hal::gpio::PullDown
				>
			}
		}
	}
}

fn into_input_method(processor: Processor) -> TokenStream {
	match processor {
		Processor::RP2040 => {
			quote! { into_pull_up_input() }
		}
	}
}

fn into_output_method(processor: Processor) -> TokenStream {
	match processor {
		Processor::RP2040 => {
			quote! { into_push_pull_output_in_state(::rp2040_hal::gpio::PinState::High) }
		}
	}
}

pub fn map_new_args(processor: Processor, arr: &ExprArray) -> TokenStream {
	let pins = arr.elems.iter().map(|pin| match processor {
		Processor::RP2040 => {
			let pin = pin.to_token_stream();

			let pin = format_ident!("Gpio{}", pin.to_string());

			quote! {
				::rp2040_hal::gpio::Pin<
					::rp2040_hal::gpio::bank0::#pin,
					::rp2040_hal::gpio::FunctionNull,
					::rp2040_hal::gpio::PullDown,
				>
			}
		}
	});

	quote! { (#( #pins ),*) }
}

pub fn map_row_fields(processor: Processor, direction: &Direction, rows: &ExprArray) -> TokenStream {
	let map = rows.elems.iter().enumerate().map(|(i, pin)| {
		let field_name = row_field_name(i);

		let doc_string = format!("Pin {} for row {i}.", quote! { #pin });

		let field_type = match direction {
			Direction::RowCol => input_pin_type(processor, pin),
			Direction::ColRow => output_pin_type(processor, pin),
		};

		quote! {
			#[doc = #doc_string]
			#field_name: #field_type
		}
	});

	quote! { #(#map,)* }
}

pub fn map_col_fields(processor: Processor, direction: &Direction, cols: &ExprArray) -> TokenStream {
	let map = cols.elems.iter().enumerate().map(|(i, pin)| {
		let field_name = col_field_name(i);

		let doc_string = format!("Pin {} for column {i}.", quote! { #pin });

		let field_type = match direction {
			Direction::RowCol => output_pin_type(processor, pin),
			Direction::ColRow => input_pin_type(processor, pin),
		};

		quote! {
			#[doc = #doc_string]
			#field_name: #field_type
		}
	});

	quote! { #(#map,)* }
}

pub fn map_rows_new(processor: Processor, direction: &Direction, rows: &ExprArray) -> TokenStream {
	let map = (0..rows.elems.len()).map(|i| {
		let name = row_field_name(i);
		let index = syn::Index::from(i);

		match processor {
			Processor::RP2040 => {
				let method = match direction {
					Direction::RowCol => into_input_method(processor),
					Direction::ColRow => into_output_method(processor),
				};

				quote! { #name: rows.#index.#method }
			}
		}
	});

	quote! { #(#map,)* }
}

pub fn map_cols_new(processor: Processor, direction: &Direction, cols: &ExprArray) -> TokenStream {
	let map = (0..cols.elems.len()).map(|i| {
		let name = col_field_name(i);
		let index = syn::Index::from(i);

		match processor {
			Processor::RP2040 => {
				let method = match direction {
					Direction::RowCol => into_output_method(processor),
					Direction::ColRow => into_input_method(processor),
				};

				quote! { #name: cols.#index.#method }
			}
		}
	});

	quote! { #(#map,)* }
}
