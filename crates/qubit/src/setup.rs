cfg_select! {
	processor = "RP2040" => {
		pub use rp2040_hal as hal;

		#[used]
		#[unsafe(link_section = ".boot2")]
		pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

		pub fn take_peripherals() -> Option<hal::pac::Peripherals> {
			hal::pac::Peripherals::take()
		}
	}
}

/// Set up and initialize the hardware needed.
#[macro_export]
macro_rules! setup_hw {
	($pac:ident) => {{
		cfg_select! {
			processor = "RP2040" => {
				/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz.
				const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

				let mut watchdog = ::rp2040_hal::Watchdog::new($pac.WATCHDOG);

				let clocks = ::rp2040_hal::clocks::init_clocks_and_plls(
					XOSC_CRYSTAL_FREQ,
					$pac.XOSC,
					$pac.CLOCKS,
					$pac.PLL_SYS,
					$pac.PLL_USB,
					&mut $pac.RESETS,
					&mut watchdog,
				)
				.unwrap();

				let timer = ::rp2040_hal::timer::Timer::new($pac.TIMER, &mut $pac.RESETS, &clocks);

				let sio = ::rp2040_hal::Sio::new($pac.SIO);

				let pins = ::rp2040_hal::gpio::Pins::new($pac.IO_BANK0, $pac.PADS_BANK0, sio.gpio_bank0, &mut $pac.RESETS);

				(clocks, timer, pins)
			}
		}
	}};
}
