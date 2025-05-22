use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn export_model_configuration(model_name: &str) {
	let full_config = keyboards::config::parse::parse_model_config(model_name).unwrap();

	let config_name = full_config.name;
	let config_author = full_config.author;
	let config_id = full_config.id;
	let config_version = full_config.version;

	let config_layout_keymap = format!("{:?}", full_config.layout_keymap);
	let config_layout_keymap_row_len = full_config.row_len;
	let config_layout_keymap_col_len = full_config.col_len;

	let config_keymap = format!("{:?}", full_config.keymap);
	let config_keymap_size = full_config.keymap_size;

	println!("cargo:rustc-env=CONFIG_NAME={config_name}");
	println!("cargo:rustc-env=CONFIG_AUTHOR={config_author}");
	println!("cargo:rustc-env=CONFIG_ID={config_id}");
	println!("cargo:rustc-env=CONFIG_VERSION={config_version}");

	println!("cargo:rustc-env=CONFIG_LAYOUT_KEYMAP={config_layout_keymap}");
	println!("cargo:rustc-env=CONFIG_LAYOUT_KEYMAP_ROW_LEN={config_layout_keymap_row_len}");
	println!("cargo:rustc-env=CONFIG_LAYOUT_KEYMAP_COL_LEN={config_layout_keymap_col_len}");

	println!("cargo:rustc-env=CONFIG_KEYMAP={config_keymap}");
	println!("cargo:rustc-env=CONFIG_KEYMAP_SIZE={config_keymap_size}");
}

fn main() {
	// TODO: Find a way to maybe pass the model name when compiling.
	// For now this uses the model I'm currently working on.
	let model_name = "moonquartz";

	export_model_configuration(model_name);

	let memory_x_path = keyboards::linker::memory_x_path(model_name);

	// Put `memory.x` in our output directory and ensure it's on the linker search path.
	let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

	let memory_x = keyboards::linker::memory_x_contents("moonquartz").unwrap();

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
