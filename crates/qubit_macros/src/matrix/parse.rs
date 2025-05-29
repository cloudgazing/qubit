use syn::{ExprArray, LitInt, LitStr, Type, TypeTuple};

#[derive(Debug, Default)]
pub struct EnvAttributes {
	pub rows: Option<String>,
	pub cols: Option<String>,
	pub layout: Option<String>,
}

#[derive(Debug, Default)]
pub struct ParsedAttributes {
	pub delay: Option<u32>,
	pub env: EnvAttributes,
}

#[derive(Debug, Default)]
pub struct ParsedFields {
	pub rows: Option<TypeTuple>,
	pub cols: Option<TypeTuple>,
	pub layout: Option<TypeTuple>,
}

pub fn parse_attributes(attrs: Vec<syn::Attribute>) -> Result<ParsedAttributes, syn::Error> {
	let mut parsed_attrs = ParsedAttributes::default();

	for attr in attrs {
		if attr.path().is_ident("kb_pin_matrix") {
			attr.parse_nested_meta(|meta| {
				let ident = meta.path.get_ident().map(std::string::ToString::to_string);

				match ident.as_deref() {
					Some("delay") => {
						let lit: LitInt = meta.value()?.parse()?;

						let delay_value = lit.base10_parse::<u32>()?;

						if let Some(prev_delay) = parsed_attrs.delay.replace(delay_value) {
							panic!("'delay' value {prev_delay} already provided.");
						}
					}
					Some("env") => {
						meta.parse_nested_meta(|meta| {
							let ident = meta.path.get_ident().map(std::string::ToString::to_string);

							let lit: LitStr = meta.value()?.parse()?;

							let (env_name, replace_res) = match ident.as_deref() {
								Some("rows") => {
									let replace_res = parsed_attrs.env.rows.replace(lit.value());

									("rows", replace_res)
								}
								Some("cols") => {
									let replace_res = parsed_attrs.env.cols.replace(lit.value());

									("cols", replace_res)
								}
								Some("layout") => {
									let replace_res = parsed_attrs.env.layout.replace(lit.value());

									("layout", replace_res)
								}
								Some(other) => {
									panic!("Unsupported value {other} provided.");
								}
								None => {
									todo!();
								}
							};

							if let Some(prev_env) = replace_res {
								panic!("env '{env_name}' value {prev_env} already provided.");
							}

							Ok(())
						})?;
					}
					Some(other) => {
						panic!("Unsupported key '{other}' in kb_pin_matrix.");
					}
					None => {
						panic!("Expected identifier in kb_pin_matrix.");
					}
				}

				Ok(())
			})?;
		}
	}

	Ok(parsed_attrs)
}

pub fn parse_fields(fields: syn::Fields) -> Result<ParsedFields, &'static str> {
	let mut parsed_fields = ParsedFields::default();

	for field in fields {
		let ident = field.ident.map(|i| i.to_string());

		let Type::Tuple(lit_type) = field.ty else {
			panic!("Expected tuple value.");
		};

		let replaced_field_opt = match ident.as_deref() {
			Some("rows") => {
				let replace_res = parsed_fields.rows.replace(lit_type);

				replace_res.map(|_| "rows")
			}
			Some("cols") => {
				let replace_res = parsed_fields.cols.replace(lit_type);

				replace_res.map(|_| "cols")
			}
			Some("layout") => {
				let replace_res = parsed_fields.layout.replace(lit_type);

				replace_res.map(|_| "layout")
			}
			Some(other) => {
				panic!("Unsupported field '{other}' in kb_pin_matrix.");
			}
			None => {
				todo!();
			}
		};

		if let Some(field_name) = replaced_field_opt {
			panic!("Duplicate {field_name} field.");
		}
	}

	Ok(parsed_fields)
}

pub fn parse_env_val_to_expr_arr(key: &str) -> Result<ExprArray, &str> {
	let Ok(layout_str) = std::env::var(key) else {
		panic!("Missing env value {key}");
	};

	let layout: ExprArray = syn::parse_str(&layout_str).expect("Failed to parse str into an array expr");

	Ok(layout)
}
