use crate::general::{Configuration, Device, Processor};

mod family;
mod mcu;

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
		Processor::RP2040 => mcu::rp2040::linker_layout(flash, device, config_size, device_config_size),
		Processor::STM32F411 => mcu::stm32f411::linker_layout(flash, device, config_size, device_config_size),
	}
}
