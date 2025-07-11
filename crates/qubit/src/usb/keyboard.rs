use core::mem::MaybeUninit;

use usb_device::bus::UsbBusAllocator;
use usbd_hid::hid_class::{HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig};

use crate::codegen::KeyboardMatrix;
use crate::setup::hal::usb::UsbBus;

mod descriptor;
mod keymaps;
mod report;
#[cfg(feature = "silverplate")]
mod silverplate;

//
use qubit_config::keyboard::Keymaps;

use crate::codegen;

pub const PACKED_SIZE: usize = codegen::LAYER0.get_packed_size();
pub const PRESSED_KEYS_BITMAPS_LEN: usize = PACKED_SIZE.div_ceil(usize::BITS as usize);

pub type KeyboardConfiguration = qubit_config::keyboard::KeyboardConfiguration<PACKED_SIZE>;
//

#[used]
#[unsafe(link_section = ".keyboard")]
static CONFIG: KeyboardConfiguration = KeyboardConfiguration {
	keymaps: Keymaps {
		keymap_0: codegen::LAYER0.get_packed(),
		keymap_1: codegen::LAYER1.get_packed(),
		keymap_2: codegen::LAYER2.get_packed(),
		keymap_3: codegen::LAYER3.get_packed(),
		keymap_4: codegen::LAYER4.get_packed(),
	},
};

/// HID class for a keyboard device.
static mut HID_CLASS: MaybeUninit<HIDClass<'static, UsbBus>> = MaybeUninit::uninit();

#[derive(Debug)]
pub struct KeyboardInstance {
	prev_report: report::Keyboard6kroReport,
	matrix: KeyboardMatrix,
}

impl KeyboardInstance {
	/// Creates a new [`KeyboardInstance`] and initializes required static state.
	///
	/// # Safety
	///
	/// This function must only be called **once** for the entire lifetime of the program.
	///
	/// If the `serial` feature is enabled, the caller must ensure the static for the port was
	/// already initialized using [`init_class`](super::serial::init_class) before calling this method.
	pub unsafe fn new(usb_bus_alloc: &'static UsbBusAllocator<UsbBus>, matrix: KeyboardMatrix) -> Self {
		// Set the value of the HID static.
		let hid_settings = HidClassSettings {
			subclass: HidSubClass::NoSubClass,
			protocol: HidProtocol::Keyboard,
			config: ProtocolModeConfig::ForceReport,
			locale: HidCountryCode::US,
		};

		let hid_class = HIDClass::new_with_settings(usb_bus_alloc, descriptor::REPORT_DESCRIPTOR, 1, hid_settings);

		let ptr = &raw mut HID_CLASS;

		// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and properly
		// aligned. This sets the value of the MaybeUninit.
		unsafe {
			(*ptr).write(hid_class);
		}

		// SAFETY: The caller guarantees this will be called only once.
		unsafe {
			keymaps::init_active_keymaps();
		}

		Self {
			prev_report: [0; 9],
			matrix,
		}
	}

	/// Scans the keyboard matrix, constructs a HID report, and sends it over USB (if changed).
	/// A critical section is used to ensure safe, exclusive access to global mutable state.
	pub fn send_pressed_keys(&mut self) {
		let pressed_keys = self.matrix.get_pressed_keys();

		// SAFETY: The active keymap was initialized before this call.
		let report = unsafe { report::construct_6kro_report(pressed_keys) };

		if report != self.prev_report {
			cortex_m::interrupt::free(|_| {
				let hid_class = {
					let ptr = &raw const HID_CLASS;

					// SAFETY: This is safe because:
					//
					// * The content was fully initialized when this struct was created.
					// * We access this inside the critical section which prevents two mutable references
					// to the value from being created.
					unsafe { (*ptr).assume_init_ref() }
				};

				_ = hid_class.push_raw_input(report.as_ref());

				#[cfg(feature = "serial")]
				{
					// SAFETY: This is safe because:
					//
					// * The caller guarantees the seral static was initialized before calling the `new` method.
					// * We access this inside the critical section which prevents two mutable references
					// to the value from being created.
					let serial_port = unsafe { super::serial::get_mut() };

					super::serial::write_message(serial_port, b"Sent report");
				}
			});

			self.prev_report = report;
		}
	}
}

/// Returns a mutable reference to the HID class instance for the keyboard.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * [`KeyboardInstance::new`] must have been called before this function.
/// * No other reference to the static value exists.
/// * The function needs to be called inside an **interrupt** or **interrupt-free** context
pub unsafe fn get_mut<'a>() -> &'a mut HIDClass<'static, UsbBus> {
	let ptr = &raw mut HID_CLASS;

	// SAFETY: The caller guarantees the content was initialized.
	unsafe { (*ptr).assume_init_mut() }
}

pub fn process_incoming_report(
	keyboard_hid: &mut HIDClass<UsbBus>,
	#[cfg(feature = "serial")] serial_port: &mut usbd_serial::SerialPort<UsbBus>,
) {
	let mut buf = [0_u8; 64];

	let Ok(rep_size) = keyboard_hid.pull_raw_output(&mut buf) else {
		return;
	};

	cfg_select! {
		feature = "silverplate" => {
			if rep_size < 2 {
				return;
			}

			let report_id = buf[0];

			match report_id {
				descriptor::KB_REP_ID_OUT => {
					process_led_report(
						buf[1],
						#[cfg(feature = "serial")]
						serial_port,
					);
				}
				silverplate::VEND_REP_ID_OUT => {
					let vendor_byte = buf[1];

					silverplate::process_vendor_report(
						keyboard_hid,
						vendor_byte,
						#[cfg(feature = "serial")]
						serial_port,
					);
				}
				_ => {}
			}
		}
		_ => {
			// The report should contain the reportID and LED byte.
			if rep_size != 2 {
				return;
			}

			process_led_report(
				buf[1],
				#[cfg(feature = "serial")]
				serial_port,
			);
		}
	}
}

fn process_led_report(led_byte: u8, #[cfg(feature = "serial")] serial_port: &mut usbd_serial::SerialPort<UsbBus>) {
	// Handle LED here.
	_ = led_byte;

	#[cfg(feature = "serial")]
	super::serial::write_message(serial_port, b"Received LED report!!\n".as_ref());
}
