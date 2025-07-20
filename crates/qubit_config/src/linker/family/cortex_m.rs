// https://docs.rs/cortex-m-rt/latest/cortex_m_rt

/// Permissions set for a specific memory region.
#[derive(Debug, Clone, Copy)]
pub struct Permissions {
	/// read-only section
	pub read: bool,
	/// read-write section
	pub write: bool,
	/// executable section
	pub execute: bool,
}

impl Permissions {
	#[must_use]
	pub const fn new(read: bool, write: bool, execute: bool) -> Self {
		Self { read, write, execute }
	}

	#[must_use]
	pub const fn read_only() -> Self {
		Self::new(true, false, false)
	}

	#[must_use]
	pub const fn read_write() -> Self {
		Self::new(true, true, false)
	}

	#[must_use]
	pub const fn to_str(self) -> &'static str {
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
	pub const fn new_flash(permissions: Permissions, origin: u32, length: u32) -> Self {
		Self {
			name: "FLASH",
			permissions,
			origin,
			length,
		}
	}

	#[must_use]
	pub const fn new_ram(permissions: Permissions, origin: u32, length: u32) -> Self {
		Self {
			name: "RAM",
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
	pub fn to_string_representation(self) -> String {
		let name = &self.name;
		let perms = &self.permissions.to_str();
		let origin = self.origin;
		let length = self.length;

		format!("	{name} ({perms}) : ORIGIN = {origin}, LENGTH = {length}\n")
	}
}

pub struct MemorySpace {
	pub flash: MemoryRegion,
	pub ram: MemoryRegion,

	pub before_flash_regions: Vec<MemoryRegion>,
	pub after_flash_regions: Vec<MemoryRegion>,
}

impl MemorySpace {
	pub fn new(flash: MemoryRegion, ram: MemoryRegion) -> Self {
		let before_flash_regions = Vec::new();
		let after_flash_regions = Vec::new();

		Self {
			flash,
			ram,
			before_flash_regions,
			after_flash_regions,
		}
	}

	pub const fn space_start() -> &'static str {
		"MEMORY {\n"
	}

	pub const fn space_end() -> &'static str {
		"}\n"
	}

	#[must_use]
	pub fn to_string_representation(&self) -> String {
		let mut contents = String::new();
		contents += Self::space_start();

		for region in &self.before_flash_regions {
			contents += &region.to_string_representation();
		}

		contents += &self.flash.to_string_representation();

		for region in &self.after_flash_regions {
			contents += &region.to_string_representation();
		}

		contents += &self.ram.to_string_representation();

		contents += Self::space_end();

		contents
	}
}

pub struct Section {
	pub section_name: &'static str,
	pub mem_region_name: &'static str,
}

impl Section {
	pub const fn new(section_name: &'static str, mem_region_name: &'static str) -> Self {
		Self {
			section_name,
			mem_region_name,
		}
	}

	pub const fn boot2() -> Self {
		Self::new("boot2", "BOOT2")
	}

	pub const fn configuration() -> Self {
		Self::new("configuration", "CONFIGURATION")
	}

	pub const fn keyboard() -> Self {
		Self::new("keyboard", "KEYBOARD")
	}

	pub const fn sections_start() -> &'static str {
		"SECTIONS {\n"
	}

	pub const fn sections_end() -> &'static str {
		"} INSERT BEFORE .text;\n"
	}

	pub fn to_string_representation(&self) -> String {
		let section_name = self.section_name;
		let region_name = self.mem_region_name;

		format!(
			"	.{section_name} ORIGIN({region_name}) :
	{{
		KEEP(*(.{section_name}));
	}} > {region_name}\n\n"
		)
	}
}

pub fn mem_x(mem_space: &MemorySpace, extern_defs: &[&str], sections: &[Section]) -> String {
	let mut mem_x_contents = String::new();

	mem_x_contents += &mem_space.to_string_representation();

	for extern_def in extern_defs {
		mem_x_contents += "\n";
		mem_x_contents += extern_def;
		mem_x_contents += "\n";
	}

	mem_x_contents += "\n";
	mem_x_contents += Section::sections_start();

	for section in sections {
		mem_x_contents += &section.to_string_representation();
	}

	mem_x_contents += Section::sections_end();

	mem_x_contents
}
