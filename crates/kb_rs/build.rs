use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn export_model_configuration(model_name: &str) {
	let config = keyboards::config::parse::parse_model_config(model_name).unwrap();

	let config_name = config.name;
	let config_author = config.firmware.author;
	let config_id = config.firmware.id;
	let config_version = config.firmware.version;

	let config_keymap = format!("{:?}", config.rows);
	let config_keymap_row_count = config.keymap.rows;
	let config_keymap_col_count = config.keymap.cols;

	println!("cargo:rustc-env=CONFIG_NAME={config_name}");
	println!("cargo:rustc-env=CONFIG_AUTHOR={config_author}");
	println!("cargo:rustc-env=CONFIG_ID={config_id}");
	println!("cargo:rustc-env=CONFIG_VERSION={config_version}");
	println!("cargo:rustc-env=CONFIG_KEYMAP={config_keymap}");
	println!("cargo:rustc-env=CONFIG_KEYMAP_ROW_COUNT={config_keymap_row_count}");
	println!("cargo:rustc-env=CONFIG_KEYMAP_COL_COUNT={config_keymap_col_count}");
}

fn main() {
	// TODO: Find a way to maybe pass the model name when compiling.
	// For now this uses the model I'm currently working on.
	let model_name = "moonquartz";

	export_model_configuration(model_name);

	let memory_x_path = keyboards::memory_x_path(model_name);

	// Put `memory.x` in our output directory and ensure it's on the linker search path.
	let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

	let memory_x = keyboards::memory_x_contents("moonquartz").unwrap();

	let mut f = File::create(out.join("memory.x")).unwrap();
	f.write_all(&memory_x).unwrap();

	println!("cargo:rustc-link-search={}", out.display());

	println!("cargo:rerun-if-changed={memory_x_path}");
	println!("cargo:rerun-if-changed=build.rs");

	println!("cargo:rustc-link-arg-bins=--nmagic");
	println!("cargo:rustc-link-arg-bins=-Tlink.x");
	// println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
	println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
