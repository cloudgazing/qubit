// https://docs.rs/cortex-m-rt/latest/cortex_m_rt

use crate::general::{Configuration, Device, Processor};

mod cortex_m;

use cortex_m::{MemoryBlock, MemoryRegion, Permissions};

/// # Panics
///
/// Will panic if the [`Configuration`](qubit_config::general::Configuration) size is outside
/// the range of u32. This should never happen though.
#[must_use]
pub fn output_linker_script<T>(processor: Processor, flash: u32, device: Device) -> String {
	let config_size = std::mem::size_of::<Configuration>();
	let config_size = u32::try_from(config_size).unwrap();

	let device_config_size = std::mem::size_of::<T>();
	let device_config_size = u32::try_from(device_config_size).unwrap();

	match processor {
		Processor::RP2040 => {
			let boot2_size: u32 = 0x100;

			let remaining_flash_size = {
				let mut size = flash;
				size = size.strict_sub(boot2_size);
				size = size.strict_sub(config_size);
				size = size.strict_sub(device_config_size);

				size
			};

			let boot = MemoryRegion::new("BOOT2", Permissions::new(true, false, true), 0x1000_0000, boot2_size);

			let flash = MemoryRegion::new(
				"FLASH",
				Permissions::new(true, false, true),
				boot.region_end(),
				remaining_flash_size,
			);

			let config = MemoryRegion::new(
				"CONFIGURATION",
				Permissions::read_only(),
				flash.region_end(),
				config_size,
			);

			let device_config = {
				let name = match device {
					Device::Keyboard => "KEYBOARD",
				};

				MemoryRegion::new(name, Permissions::read_only(), config.region_end(), device_config_size)
			};

			let ram = MemoryRegion::new("RAM", Permissions::new(true, true, false), 0x2000_0000, 0x42000);

			let mem_block = MemoryBlock {
				boot,
				flash,
				config,
				device_config,
				ram,
			};

			let mut mem_x_contents = String::new();

			mem_x_contents += &mem_block.to_string_representation();
			mem_x_contents += "\nEXTERN(BOOT2_FIRMWARE)\n";
			mem_x_contents += &mem_block.sections_block();

			mem_x_contents
		}
	}
}
