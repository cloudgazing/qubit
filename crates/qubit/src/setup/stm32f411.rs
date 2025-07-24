pub use cortex_m_rt::entry;

use hal::pac::interrupt;
pub use stm32f4xx_hal as hal;

use crate::usb::QubitDevice;
use hal::{gpio::GpioExt, rcc::RccExt, timer::TimerExt};

pub type Countdown = hal::timer::CounterUs<hal::pac::TIM2>;
pub type UsbBus = hal::otg_fs::UsbBus<hal::otg_fs::USB>;
pub type UsbBusAllocator = usb_device::bus::UsbBusAllocator<UsbBus>;

pub type CountDuration = fugit::TimerDurationU32<1_000_000>;

struct Pins {
	pub gpio_a: hal::gpio::gpioa::Parts,
	#[cfg(stm32f411_bank_b)]
	pub gpio_b: hal::gpio::gpiob::Parts,
	#[cfg(stm32f411_bank_c)]
	pub gpio_c: hal::gpio::gpioc::Parts,
	#[cfg(stm32f411_bank_d)]
	pub gpio_d: hal::gpio::gpiod::Parts,
	#[cfg(stm32f411_bank_e)]
	pub gpio_e: hal::gpio::gpioe::Parts,
	#[cfg(stm32f411_bank_h)]
	pub gpio_h: hal::gpio::gpioh::Parts,
}

/// Initialize all the peripherals and components the device needs.
///
/// # Safety
///
/// The function needs to be called only once, before enabling interrupts.
pub unsafe fn initialize_mcu() -> (QubitDevice, Countdown) {
	let dp = hal::pac::Peripherals::take().unwrap();

	let clocks = dp
		.RCC
		.constrain()
		.cfgr
		.use_hse(fugit::HertzU32::MHz(25))
		.sysclk(fugit::HertzU32::MHz(96))
		.pclk1(fugit::HertzU32::MHz(48))
		.pclk2(fugit::HertzU32::MHz(96))
		.freeze();

	let pins = Pins {
		gpio_a: dp.GPIOA.split(),
		#[cfg(stm32f411_bank_b)]
		gpio_b: dp.GPIOB.split(),
		#[cfg(stm32f411_bank_c)]
		gpio_c: dp.GPIOC.split(),
		#[cfg(stm32f411_bank_d)]
		gpio_d: dp.GPIOD.split(),
		#[cfg(stm32f411_bank_e)]
		gpio_e: dp.GPIOE.split(),
		#[cfg(stm32f411_bank_h)]
		gpio_h: dp.GPIOH.split(),
	};

	let countdown = dp.TIM2.counter_us(&clocks);

	let usb_alloc = {
		static mut EP_MEMORY: [u32; 1024] = [0; 1024];

		let usb = hal::otg_fs::USB::new(
			(dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK),
			(pins.gpio_a.pa11, pins.gpio_a.pa12),
			&clocks,
		);

		let ep_memory = {
			let ptr = &raw mut EP_MEMORY;

			// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and
			// properly aligned.
			unsafe { (*ptr).as_mut_slice() }
		};

		hal::otg_fs::UsbBus::new(usb, ep_memory)
	};

	#[cfg(keyboard)]
	let kb_matrix = crate::codegen::setup_keyboard!(pins);

	// SAFETY: The caller guarantees this is called once, before interrupts are enabled..
	let qubit_usb_device = unsafe { crate::usb::QubitDevice::new(usb_alloc, kb_matrix) };

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
		hal::pac::NVIC::unmask(hal::pac::Interrupt::OTG_FS);
	}
}

pub fn start_countdown(countdown: &mut Countdown) {
	countdown.start(CountDuration::millis(10)).unwrap();
}

#[interrupt]
/// Poll the USB for new events.
fn OTG_FS() {
	// SAFETY: The function is called inside an interrupt context and after initialization.
	unsafe {
		crate::usb::poll_device();
	}
}
