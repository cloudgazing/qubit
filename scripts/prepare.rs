#!/usr/bin/env cargo
---cargo
package.edition = "2024"
---

use std::process::Command;

fn main() {
	let mut args = std::env::args().skip(1);

	let mut author = None;
	let mut model = None;
	let mut custom = None;

	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--author" => {
				author = args.next();
			}
			"--model" => {
				model = args.next();
			}
			"--custom" => {
				custom = args.next();
			}
			_ => {
				eprintln!("Unexpected argument: {}", arg);
				std::process::exit(1);
			}
		}
	}

	match (author, model, custom) {
		(Some(a), Some(m), None) => {
			let cwd = std::env::current_dir().unwrap();
			let prepare_path = cwd.join("scripts/_prepare.rs");

			let status = Command::new("cargo")
				.env("AUTHOR", a)
				.env("MODEL", m)
				.args(["+nightly", "-Zscript", prepare_path.to_str().unwrap()])
				.status()
				.expect("failed to run cargo script");

			if !status.success() {
				eprintln!("cargo script failed with code: {status}");

				std::process::exit(1);
			}
		}
		(None, None, Some(_c)) => {
			todo!()
		}
		_ => {
			eprintln!("Usage: --author <name> --model <name>  OR  --custom <path>");

			std::process::exit(1);
		}
	};
}
