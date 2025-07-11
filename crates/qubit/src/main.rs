//! # Qubit

// #![warn(missing_docs)]
#![feature(cfg_select)]
#![no_std]
#![no_main]

// Imports used just for linking
use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

use embedded_hal::digital::OutputPin;
use qubit_config::general::Configuration;
use usb_device::bus::UsbBusAllocator;

mod setup;
mod time;
mod usb;

mod codegen {
	include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

use setup::hal::pac::interrupt;

#[used]
#[unsafe(link_section = ".configuration")]
static DEVICE_CONFIG: Configuration = Configuration {
	name: codegen::NAME,
	author: codegen::AUTHOR,
	version: codegen::VERSION.as_bitmap(),
	device: codegen::DEVICE,
};

/// The main entry point for the program.
#[setup::hal::entry]
fn main() -> ! {
	let mut peripherals = setup::take_peripherals().unwrap();

	let (clocks, timer, pins) = setup_hw!(peripherals);

	// Use this LED to check for errors during setup.
	let mut led_pin = pins.gpio25.into_push_pull_output();
	led_pin.set_high().unwrap();
	// -- --

	// #[cfg(feature = "keyboard")]
	let kb_matrix = setup_keyboard!(pins);

	let usb_bus = ::rp2040_hal::usb::UsbBus::new(
		peripherals.USBCTRL_REGS,
		peripherals.USBCTRL_DPRAM,
		clocks.usb_clock,
		true,
		&mut peripherals.RESETS,
	);

	let usb_alloc = UsbBusAllocator::new(usb_bus);

	// SAFETY: We initialize the USB device before enabling the interrupt.
	let mut qubit_usb_device = unsafe { usb::QubitDevice::new(usb_alloc, kb_matrix) };

	// SAFETY: The values this function has access to have been initialized above. It is safe
	// to enable the interrupt.
	unsafe {
		setup::hal::pac::NVIC::unmask(setup::hal::pac::Interrupt::USBCTRL_IRQ);
	}

	let mut count_down = time::CountDown::new(timer);
	count_down.start(setup::hal::fugit::MicrosDurationU64::millis(10));

	// -- --
	led_pin.set_low().unwrap();
	// -- --

	loop {
		if count_down.wait().is_ok() {
			// #[cfg(feature = "keyboard")]
			qubit_usb_device.keyboard.send_pressed_keys();
		}
	}
}

/// Poll the USB for new events.
#[interrupt]
fn USBCTRL_IRQ() {
	// SAFETY: The function is called inside an interrupt context and after initialization.
	unsafe {
		usb::poll_device();
	}
}
