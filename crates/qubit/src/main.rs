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

use embedded_hal::digital::OutputPin;
use hal::pac::interrupt;
use keyboard::KeyboardMatrix;
use usb_device::bus::UsbBusAllocator;

use qubit_macros::define_configuration;

mod connection;
mod keyboard;
mod keymap;
mod parse;
mod report;
mod time;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz.
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

const KM_SIZE: usize = parse::str_to_usize(env!("CONFIG_KEYMAP_SIZE"));

type Keymap = keyboards::config::Keymap<KM_SIZE>;
type Configuration = keyboards::config::Configuration<KM_SIZE>;

#[used]
#[unsafe(link_section = ".boot2")]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rustfmt::skip]
#[used]
#[unsafe(link_section = ".keyboard_configuration")]
pub static CONFIGURATION: Configuration = define_configuration!(
	CONFIG_NAME,
	CONFIG_AUTHOR,
	CONFIG_ID,
	CONFIG_VERSION,
	CONFIG_KEYMAP
);

// Check that the configuration we use fits in the link section. (config section size = 0x19000)
const _: () = assert!(core::mem::size_of::<Configuration>() <= 0x19000);

#[hal::entry]
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

	let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

	let sio = hal::Sio::new(pac.SIO);

	let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

	// Take ownership of this reserved pin to prevent it's use.
	#[allow(clippy::no_effect_underscore_binding)]
	let _reserved_pin = pins.gpio15;

	// We use the LED to verify that the initialization was successful by turning it off after.
	let mut led_pin = pins.gpio25.into_push_pull_output();
	led_pin.set_high().unwrap();

	// SAFETY: Keymap gets initalized only once.
	unsafe { keymap::initialize_active_keymap() };

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

	let usb_alloc = UsbBusAllocator::new(hal::usb::UsbBus::new(
		pac.USBCTRL_REGS,
		pac.USBCTRL_DPRAM,
		clocks.usb_clock,
		true,
		&mut pac.RESETS,
	));

	// Safety: We initialize the USB device before enabling the interrupts
	let usb_dev = unsafe { connection::UsbDeviceInstance::new(usb_alloc) };

	unsafe {
		hal::pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
	}

	// Counter used for the interval at which to check for pressed keys.
	let mut count_down = time::CountDown::new(timer);
	count_down.start(hal::fugit::MicrosDurationU64::millis(10));

	led_pin.set_low().unwrap();

	let mut prev_report: report::RawKeyboardReport = Default::default();

	loop {
		if count_down.wait().is_ok() {
			let pressed_keys = kb_matrix.get_pressed_keys();

			// SAFETY: The active keymap was initialized before this call.
			let report = unsafe { report::construct_6kro_report(pressed_keys) };

			if report != prev_report {
				usb_dev.send_keyboard_report(&report);

				prev_report = report;
			}
		}
	}
}

/// Poll the USB constantly.
#[interrupt]
fn USBCTRL_IRQ() {
	// Safety: The function is called inside an interrupt context and after initialization.
	unsafe {
		connection::poll_usb_device();
	}
}
