#![feature(sync_unsafe_cell)]
#![no_std]
#![no_main]

// Imports used just for linking
use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

// A shorter import rename for the HAL. In the future this might change to support the use
// of other BSPs through feature flags.
use rp2040_hal as hal;

#[cfg(feature = "serial")]
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use hal::pac::interrupt;
use keyboard::KeyboardMatrix;
use usb_device::bus::UsbBusAllocator;

use kb_rs_macro_derive::define_configuration;
use keyboards::config::Configuration;

pub mod keyboard;
mod misc;
pub mod time;
pub mod usb;

const R: usize = misc::parse_env_usize(env!("CONFIG_KEYMAP_ROW_COUNT"));
const C: usize = misc::parse_env_usize(env!("CONFIG_KEYMAP_COL_COUNT"));

#[used]
#[unsafe(link_section = ".boot2")]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rustfmt::skip]
#[used]
#[unsafe(link_section = ".keyboard_configuration")]
pub static CONFIGURATION: Configuration<R, C> = define_configuration!(
	CONFIG_NAME,
	CONFIG_AUTHOR,
	CONFIG_ID,
	CONFIG_VERSION,
	CONFIG_KEYMAP
);

const _: () = {
	// Check that the configuration we use fits in the link section.
	const CONFIG_SECTION_SIZE: usize = 0x19000;

	assert!(core::mem::size_of::<Configuration<R, C>>() <= CONFIG_SECTION_SIZE);
};

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz.
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[hal::entry]
fn main_entry() -> ! {
	main()
}

fn main() -> ! {
	// Get the main components which are used for the initialization, like peripherals,
	// clocks, registers, pins.
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

	#[cfg(feature = "serial")]
	let mut timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
	#[cfg(not(feature = "serial"))]
	let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

	let sio = hal::Sio::new(pac.SIO);

	let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

	// Get the stored or default keymap configuration.

	// TODO: Remove these pins declarations once storing the configuration is implmented.
	// Column pins
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
		(r0, r1, r2, r3, r4),
		(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13),
	);

	// Take ownership of this reserved pin to prevent it's use.
	#[allow(clippy::no_effect_underscore_binding)]
	let _reserved_pin = pins.gpio15;

	// We use the LED to verify that the initialization was successful by turning it off after.
	let mut led_pin = pins.gpio25.into_push_pull_output();
	led_pin.set_high().unwrap();

	let usb_alloc = UsbBusAllocator::new(hal::usb::UsbBus::new(
		pac.USBCTRL_REGS,
		pac.USBCTRL_DPRAM,
		clocks.usb_clock,
		true,
		&mut pac.RESETS,
	));

	// Safety: We initialize the USB device before enabling the interrupts
	let usb_dev = unsafe { usb::UsbDeviceInstance::new(usb_alloc) };

	unsafe {
		// let mut core_peripherals = hal::pac::CorePeripherals::take().unwrap();
		// core_peripherals.NVIC.set_priority(hal::pac::interrupt::USBCTRL_IRQ, 1);
		hal::pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
	}

	#[cfg(feature = "serial")]
	{
		// Set a delay to wait for serial to get connected.
		timer.delay_ms(1500);
		usb_dev.send_serial_message("Listening:\n".as_bytes());
	}

	// TODO: just for testing
	#[cfg(feature = "serial")]
	{
		unsafe {
			let p = 0x1000_0000 + 0x100 + (0x0080_0000 - 0x100 - 0x19000);
			let ptr = p as *const Configuration<R, C>;

			let config = &*ptr;

			usb_dev.send_serial_message("Board name: ".as_bytes());
			usb_dev.send_serial_message(config.name.as_bytes());
			usb_dev.send_serial_message("\n".as_bytes());
			usb_dev.send_serial_message("Author name: ".as_bytes());
			usb_dev.send_serial_message(config.author.as_bytes());
			usb_dev.send_serial_message("\n".as_bytes());
			usb_dev.send_serial_message("Version: ".as_bytes());
			usb_dev.send_serial_message(config.version.as_bytes());
			usb_dev.send_serial_message("\n".as_bytes());
		}
	}
	// remove the code above when done

	// Counter used for the interval at which to check for pressed keys.
	let mut count_down = time::CountDown::new(timer);
	count_down.start(hal::fugit::MicrosDurationU64::millis(10));

	led_pin.set_low().unwrap();

	loop {
		if count_down.wait().is_ok() {
			if let Some(report) = kb_matrix.generate_key_report() {
				usb_dev.send_keyboard_report(&report);
				#[cfg(feature = "serial")]
				usb_dev.send_serial_message("Key sent!!\n".as_bytes());
			} else {
				#[cfg(feature = "serial")]
				usb_dev.send_keyboard_report(&keyboard::get_end_keyboard_report());
			}
		}
	}
}

/// Poll the USB constantly.
#[interrupt]
fn USBCTRL_IRQ() {
	// Safety: The function is called inside an interrupt context and after initialization.
	unsafe {
		usb::poll_usb_device();
	}
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
	main();
}
