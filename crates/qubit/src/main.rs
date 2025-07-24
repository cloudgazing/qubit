//! # Qubit

// #![warn(missing_docs)]
#![feature(cfg_select)]
#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;

use panic_probe as _;

use qubit_config::general::Configuration;

mod setup;
#[cfg(mcu = "rp2040")]
mod time;
mod usb;

mod codegen {
	include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

	#[allow(
		clippy::single_component_path_imports,
		reason = "The pub export is required to access this macro from other modules."
	)]
	pub(crate) use setup_keyboard;
}

#[used]
#[unsafe(link_section = ".configuration")]
static DEVICE_CONFIG: Configuration = Configuration {
	name: codegen::NAME,
	author: codegen::AUTHOR,
	version: codegen::VERSION.as_bitmap(),
	device: codegen::DEVICE,
};

/// The main entry point for the program.
#[setup::entry]
fn main() -> ! {
	// SAFETY: This function is called once, initializing the USB device and other statics
	// before enabling the interrupt.
	let (mut qubit_usb_device, mut countdown) = unsafe { setup::initialize_mcu() };

	// SAFETY: The values this function has access to have been initialized above. It is safe
	// to enable the interrupt.
	unsafe {
		setup::enable_interrupt();
	}

	setup::start_countdown(&mut countdown);

	loop {
		if countdown.wait().is_ok() {
			#[cfg(keyboard)]
			qubit_usb_device.keyboard.send_pressed_keys();
		}
	}
}
