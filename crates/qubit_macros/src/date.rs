use proc_macro::TokenStream;

use chrono::Datelike;
use quote::quote;

pub fn build_date_bitmap_macro() -> TokenStream {
	// Bits 4-0:  Day (1-31) - 5 bits
	// Bits 8-5:  Month (1-12) - 4 bits
	// Bits 15-9: Years since 2024 - 7 bits

	let now = chrono::Utc::now();

	let year_offset = u16::try_from(now.year()).unwrap() - 2024;
	let month = u16::try_from(now.month()).unwrap();
	let day = u16::try_from(now.day()).unwrap();

	let mut bitmap: u16 = 0;
	bitmap |= day & 0x1F; // 0-4
	bitmap |= (month & 0x0F) << 5; // 5-8
	bitmap |= (year_offset & 0x7F) << 9; // 9-15

	quote! { #bitmap }.into()
}
