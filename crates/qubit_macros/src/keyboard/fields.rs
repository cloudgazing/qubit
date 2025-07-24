use proc_macro2::{Span, TokenStream};
use qubit_config::general::Processor;
use quote::{ToTokens, format_ident, quote};
use syn::{Expr, ExprArray, Ident, LitChar, LitInt};

use super::attributes::Direction;

fn split_stm32_def(pin: &TokenStream) -> (LitChar, LitInt) {
	let ident_str = pin.to_string();
	let ident_str = ident_str.trim();

	let (port, number) = ident_str.split_at(1);

	let port_char = LitChar::new(port.chars().next().unwrap(), Span::call_site());
	let pin_number = LitInt::new(number, Span::call_site());

	(port_char, pin_number)
}

pub fn row_field_name(index: usize) -> Ident {
	format_ident!("row_{index}")
}

pub fn col_field_name(index: usize) -> Ident {
	format_ident!("col_{index}")
}

fn input_pin_type(processor: Processor, pin: &Expr) -> TokenStream {
	let pin = pin.into_token_stream();

	match processor {
		Processor::RP2040 => {
			let pin_expr = format_ident!("Gpio{}", pin.to_string());

			quote! {
				::rp2040_hal::gpio::Pin<
					::rp2040_hal::gpio::bank0::#pin_expr,
					::rp2040_hal::gpio::FunctionSioInput,
					::rp2040_hal::gpio::PullUp
				>
			}
		}
		Processor::STM32F411 => {
			let (port_char, pin_number) = split_stm32_def(&pin);

			quote! {
				::stm32f4xx_hal::gpio::Pin<
					#port_char,
					#pin_number,
					::stm32f4xx_hal::gpio::Input
				>
			}
		}
	}
}

fn output_pin_type(processor: Processor, pin: &Expr) -> TokenStream {
	let pin = pin.into_token_stream();

	match processor {
		Processor::RP2040 => {
			let pin_expr = format_ident!("Gpio{}", pin.to_string());

			quote! {
				::rp2040_hal::gpio::Pin<
					::rp2040_hal::gpio::bank0::#pin_expr,
					::rp2040_hal::gpio::FunctionSioOutput,
					::rp2040_hal::gpio::PullDown
				>
			}
		}
		Processor::STM32F411 => {
			let (port_char, pin_number) = split_stm32_def(&pin);

			quote! {
				::stm32f4xx_hal::gpio::Pin<
					#port_char,
					#pin_number,
					::stm32f4xx_hal::gpio::Output
				>
			}
		}
	}
}

fn into_input_method(processor: Processor) -> TokenStream {
	match processor {
		Processor::RP2040 | Processor::STM32F411 => {
			quote! { into_pull_up_input() }
		}
	}
}

fn into_output_method(processor: Processor) -> TokenStream {
	match processor {
		Processor::RP2040 => {
			quote! { into_push_pull_output_in_state(::embedded_hal::digital::PinState::High) }
		}
		Processor::STM32F411 => {
			quote! { into_push_pull_output_in_state(::stm32f4xx_hal::gpio::PinState::High) }
		}
	}
}

pub fn map_new_args(processor: Processor, arr: &ExprArray) -> TokenStream {
	let pins = arr.elems.iter().map(|pin| {
		let pin = pin.to_token_stream();

		match processor {
			Processor::RP2040 => {
				let pin = format_ident!("Gpio{}", pin.to_string());

				quote! {
					::rp2040_hal::gpio::Pin<
						::rp2040_hal::gpio::bank0::#pin,
						::rp2040_hal::gpio::FunctionNull,
						::rp2040_hal::gpio::PullDown,
					>
				}
			}
			Processor::STM32F411 => {
				let (port_char, pin_number) = split_stm32_def(&pin);

				quote! { ::stm32f4xx_hal::gpio::Pin<#port_char, #pin_number> }
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

		let method = match direction {
			Direction::RowCol => into_input_method(processor),
			Direction::ColRow => into_output_method(processor),
		};

		quote! { #name: rows.#index.#method }
	});

	quote! { #(#map,)* }
}

pub fn map_cols_new(processor: Processor, direction: &Direction, cols: &ExprArray) -> TokenStream {
	let map = (0..cols.elems.len()).map(|i| {
		let name = col_field_name(i);
		let index = syn::Index::from(i);

		let method = match direction {
			Direction::RowCol => into_output_method(processor),
			Direction::ColRow => into_input_method(processor),
		};

		quote! { #name: cols.#index.#method }
	});

	quote! { #(#map,)* }
}
