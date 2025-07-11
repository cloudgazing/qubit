use core::fmt::Write;

#[derive(Debug, Clone, Copy)]
pub struct Permissions {
	pub read: bool,
	pub write: bool,
	pub execute: bool,
}

impl Permissions {
	#[must_use]
	pub const fn new(read: bool, write: bool, execute: bool) -> Self {
		Self { read, write, execute }
	}

	#[must_use]
	pub const fn read_only() -> Self {
		Self {
			read: true,
			write: false,
			execute: false,
		}
	}

	#[must_use]
	pub const fn as_str(self) -> &'static str {
		match (self.read, self.write, self.execute) {
			(false, false, false) => "",
			(false, false, true) => "x",
			(false, true, false) => "w",
			(false, true, true) => "wx",
			(true, false, false) => "r",
			(true, false, true) => "rx",
			(true, true, false) => "rw",
			(true, true, true) => "rwx",
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
	pub name: &'static str,
	pub permissions: Permissions,
	pub origin: u32,
	pub length: u32,
}

impl MemoryRegion {
	#[must_use]
	pub const fn new(name: &'static str, permissions: Permissions, origin: u32, length: u32) -> Self {
		Self {
			name,
			permissions,
			origin,
			length,
		}
	}

	#[must_use]
	pub const fn region_end(&self) -> u32 {
		self.origin + self.length
	}

	#[must_use]
	pub fn string_representation(&self) -> String {
		let name = &self.name;
		let perms = &self.permissions.as_str();
		let origin = self.origin;
		let length = self.length;

		format!("{name} ({perms}) : ORIGIN = {origin}, LENGTH = {length}")
	}
}

#[derive(Debug)]
pub struct MemoryBlock {
	pub boot: MemoryRegion,
	pub flash: MemoryRegion,

	pub config: MemoryRegion,
	pub device_config: MemoryRegion,

	pub ram: MemoryRegion,
}

impl MemoryBlock {
	#[must_use]
	pub fn to_string_representation(&self) -> String {
		let mut contents = String::new();
		contents += "MEMORY {\n";

		contents += &self.boot.string_representation();
		contents += "\n";

		contents += &self.flash.string_representation();
		contents += "\n";

		contents += &self.config.string_representation();
		contents += "\n";

		contents += &self.device_config.string_representation();
		contents += "\n";

		contents += &self.ram.string_representation();
		contents += "\n";

		contents += "}\n";

		contents
	}

	pub fn sections_block(&self) -> String {
		let mut contents = String::new();

		let boot2_name = &self.boot.name;
		let config_name = &self.config.name;

		let device_config_section = &self.device_config.name.to_lowercase();
		let device_config_name = &self.device_config.name;

		write!(
			contents,
			"
			SECTIONS {{
				.boot2 ORIGIN({boot2_name}) :
				{{
					KEEP(*(.boot2));
				}} > {boot2_name}

				.configuration ORIGIN({config_name}) :
				{{
					KEEP(*(.configuration));
				}} > {config_name}

				.{device_config_section} ORIGIN({device_config_name}) :
				{{
					KEEP(*(.{device_config_section}));
				}} > {device_config_name}
			}} INSERT BEFORE .text;
			"
		)
		.unwrap();

		contents
	}
}
