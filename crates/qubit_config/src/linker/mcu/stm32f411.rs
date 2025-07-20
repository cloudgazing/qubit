use crate::general::Device;
use crate::linker::family::cortex_m;

use cortex_m::{MemoryRegion, MemorySpace, Permissions, Section};

const FLASH_ORIGIN: u32 = 0x0800_0000;

const RAM_ORIGIN: u32 = 0x2000_0000;
const RAM_LENGTH: u32 = 0x20000;

pub fn linker_layout(flash_size: u32, device: Device, config_size: u32, dev_config_size: u32) -> String {
	let remaining_flash_size = {
		let size = flash_size;
		let size = size.strict_sub(config_size);

		size.strict_sub(dev_config_size)
	};

	let flash = MemoryRegion::new_flash(Permissions::new(true, false, true), FLASH_ORIGIN, remaining_flash_size);

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

	mem_space
		.after_flash_regions
		.extend_from_slice(&[config, device_config]);

	let mut mem_x = String::new();

	mem_x += &cortex_m::mem_x(&mem_space, &[], &[Section::configuration(), Section::keyboard()]);

	mem_x += "
/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
";

	mem_x
}
