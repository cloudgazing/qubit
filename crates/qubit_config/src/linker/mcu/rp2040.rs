use crate::general::Device;
use crate::linker;
use linker::family::cortex_m::{MemoryRegion, MemorySpace, Permissions, Section};

const BOOT2_ORIGIN: u32 = 0x1000_0000;
const BOOT2_LENGTH: u32 = 0x100;

const RAM_ORIGIN: u32 = 0x2000_0000;
const RAM_LENGTH: u32 = 0x42000;

const BOO2_EXTERN_DEF: &str = "EXTERN(BOOT2_FIRMWARE)";

pub fn linker_layout(flash_size: u32, device: Device, config_size: u32, dev_config_size: u32) -> String {
	let remaining_flash_size = {
		let size = flash_size;
		let size = size.strict_sub(BOOT2_LENGTH);
		let size = size.strict_sub(config_size);

		size.strict_sub(dev_config_size)
	};

	let boot2 = MemoryRegion::new("BOOT2", Permissions::new(true, false, true), BOOT2_ORIGIN, BOOT2_LENGTH);

	let flash = MemoryRegion::new_flash(
		Permissions::new(true, false, true),
		boot2.region_end(),
		remaining_flash_size,
	);

	let config = MemoryRegion::new(
		"CONFIGURATION",
		Permissions::read_only(),
		flash.region_end(),
		config_size,
	);

	let device_config = MemoryRegion::new(
		device.region_name(),
		Permissions::read_only(),
		config.region_end(),
		dev_config_size,
	);

	let ram = MemoryRegion::new_ram(Permissions::read_write(), RAM_ORIGIN, RAM_LENGTH);

	let mut mem_space = MemorySpace::new(flash, ram);

	mem_space.before_flash_regions.extend_from_slice(&[boot2]);
	mem_space
		.after_flash_regions
		.extend_from_slice(&[config, device_config]);

	crate::linker::family::cortex_m::mem_x(
		&mem_space,
		&[BOO2_EXTERN_DEF],
		&[Section::boot2(), Section::configuration(), Section::keyboard()],
	)
}
