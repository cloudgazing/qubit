use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
	// Put `memory.x` in our output directory and ensure it's on the linker search path.
	let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

	let memory_x = include_bytes!("memory.x");

	let mut f = File::create(out.join("memory.x")).unwrap();
	f.write_all(memory_x).unwrap();

	println!("cargo:rustc-link-search={}", out.display());

	println!("cargo:rerun-if-changed=memory.x");
	println!("cargo:rerun-if-changed=build.rs");

	// println!("cargo:rustc-link-arg-bins=--nmagic");
	println!("cargo:rustc-link-arg-bins=--nmagic");
	println!("cargo:rustc-link-arg-bins=-Tlink.x");
	// println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
	println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
