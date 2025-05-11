#![feature(sync_unsafe_cell)]
#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]
#![no_std]
#![no_main]

// Imports used just for linking
use defmt_rtt as _;
use panic_probe as _;

// A shorter import rename for the HAL. In the future this might change to support the use
// of other BSPs through feature flags.
use rp2040_hal as hal;

use defmt::warn;
use embedded_hal::digital::OutputPin;
use hal::pac::interrupt;
use keyboard::KeyboardMatrix;
use usb_device::bus::UsbBusAllocator;

pub use kb_rs_derive;

pub mod boilerplate;
pub mod buff;
pub mod keyboard;
pub mod time;

#[used]
#[unsafe(link_section = ".boot2")]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz.
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[hal::entry]
fn main_entry() -> ! {
	main()
}

fn main() -> ! {
	let mut pac = hal::pac::Peripherals::take().unwrap();
	let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
	let clocks = hal::clocks::init_clocks_and_plls(
		XOSC_CRYSTAL_FREQ,
		pac.XOSC,
		pac.CLOCKS,
		pac.PLL_SYS,
		pac.PLL_USB,
		&mut pac.RESETS,
		&mut watchdog,
	)
	.unwrap();
	let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
	let sio = hal::Sio::new(pac.SIO);

	let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

	// Col pins
	let c0 = pins.gpio0;
	let c1 = pins.gpio1;
	let c2 = pins.gpio2;
	let c3 = pins.gpio3;
	let c4 = pins.gpio4;
	let c5 = pins.gpio5;
	let c6 = pins.gpio6;
	let c7 = pins.gpio7;
	let c8 = pins.gpio8;
	let c9 = pins.gpio9;
	let c10 = pins.gpio10;
	let c11 = pins.gpio11;
	let c12 = pins.gpio12;
	let c13 = pins.gpio13;

	// Row pins
	let r0 = pins.gpio16;
	let r1 = pins.gpio17;
	let r2 = pins.gpio18;
	let r3 = pins.gpio19;
	let r4 = pins.gpio20;

	let mut kb_matrix = KeyboardMatrix::new(
		(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13),
		(r0, r1, r2, r3, r4),
	);

	#[cfg(feature = "marble")]
	#[allow(clippy::no_effect_underscore_binding)]
	let _reserved_pin = pins.gpio15;

	let mut led_pin = pins.gpio25.into_push_pull_output();
	led_pin.set_high().unwrap();

	let usb_alloc = UsbBusAllocator::new(hal::usb::UsbBus::new(
		pac.USBCTRL_REGS,
		pac.USBCTRL_DPRAM,
		clocks.usb_clock,
		true,
		&mut pac.RESETS,
	));

	// Call this ONCE to set the static variables
	boilerplate::init_usb_alloc(usb_alloc);
	boilerplate::init_device_class_list();
	boilerplate::init_usb_device();

	unsafe {
		// let mut core_peripherals = hal::pac::CorePeripherals::take().unwrap();
		// core_peripherals.NVIC.set_priority(hal::pac::interrupt::USBCTRL_IRQ, 1);

		hal::pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
	}

	// let mut kb_inteface = keyboard::KeyboardInterface::new(pins.gpio10);

	// Start counter
	let mut count_down = time::CountDown::new(timer);
	count_down.start(hal::fugit::MicrosDurationU64::millis(10));

	led_pin.set_low().unwrap();

	loop {
		if count_down.wait().is_ok() {
			if let Some(report) = kb_matrix.generate_key_report() {
				boilerplate::push_keyboard_events(report);
				boilerplate::send_serial_message("Key sent!!\n");
			} else {
				boilerplate::push_keyboard_events(keyboard::get_end_keyboard_report());
			}

			// 	if let Some(report) = kb_inteface.get_pressed_letter().unwrap() {
			// 	} else {
			// 	}
		}
	}
}

#[interrupt]
fn USBCTRL_IRQ() {
	boilerplate::poll_usb_device();
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
	main();
}
