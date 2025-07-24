pub use hal::entry;
pub use hal::pac::interrupt;
pub use rp2040_hal as hal;

use crate::usb::QubitDevice;

pub type Countdown = crate::time::CountDown;
pub type UsbBus = hal::usb::UsbBus;
pub type UsbBusAllocator = usb_device::bus::UsbBusAllocator<UsbBus>;

#[used]
#[unsafe(link_section = ".boot2")]
pub static BOOT2: [u8; 256] = ::rp2040_boot2::BOOT_LOADER_GENERIC_03H;

pub type CountDuration = hal::fugit::MicrosDurationU64;

/// Initialize all the peripherals and components the device needs.
///
/// # Safety
///
/// The function needs to be called only once, before enabling interrupts.
pub unsafe fn initialize_mcu() -> (QubitDevice, Countdown) {
	let mut dp = hal::pac::Peripherals::take().unwrap();

	let clocks = {
		const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

		let mut watchdog = hal::Watchdog::new(dp.WATCHDOG);

		hal::clocks::init_clocks_and_plls(
			XOSC_CRYSTAL_FREQ,
			dp.XOSC,
			dp.CLOCKS,
			dp.PLL_SYS,
			dp.PLL_USB,
			&mut dp.RESETS,
			&mut watchdog,
		)
		.unwrap()
	};

	let pins = {
		let sio = hal::Sio::new(dp.SIO);

		hal::gpio::Pins::new(dp.IO_BANK0, dp.PADS_BANK0, sio.gpio_bank0, &mut dp.RESETS)
	};

	// // Use this LED to check for errors during setup.
	// #[cfg(has_led)]
	// let mut led_pin = pins.gpio25.into_push_pull_output_in_state(PinState::High);

	let countdown = {
		let timer = hal::timer::Timer::new(dp.TIMER, &mut dp.RESETS, &clocks);

		crate::time::CountDown::new(timer)
	};

	let usb_alloc = {
		let usb_bus = hal::usb::UsbBus::new(
			dp.USBCTRL_REGS,
			dp.USBCTRL_DPRAM,
			clocks.usb_clock,
			true,
			&mut dp.RESETS,
		);

		UsbBusAllocator::new(usb_bus)
	};

	#[cfg(keyboard)]
	let kb_matrix = crate::codegen::setup_keyboard!(pins);

	// SAFETY: The caller guarantees this is called once, before interrupts are enabled.
	let qubit_usb_device = unsafe { crate::usb::QubitDevice::new(usb_alloc, kb_matrix) };

	// #[cfg(has_led)]
	// led_pin.set_low().unwrap();

	(qubit_usb_device, countdown)
}

/// Enable the USB interrupt.
///
/// # Safety
///
/// This function enables an interrupt that accesses and mutates static data.
/// The caller must ensure that all those statics have been properly initialized before calling this,
/// which means calling [`initialize_mcu`] first.
pub unsafe fn enable_interrupt() {
	// SAFETY: The caller has ensured that all required statics are initialized.
	unsafe {
		hal::pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
	}
}

pub fn start_countdown(countdown: &mut Countdown) {
	countdown.start(CountDuration::millis(10));
}

/// Poll the USB for new events.
#[interrupt]
fn USBCTRL_IRQ() {
	// SAFETY: The function is called inside an interrupt context and after initialization.
	unsafe {
		crate::usb::poll_device();
	}
}
