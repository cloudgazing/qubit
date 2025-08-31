use core::mem::MaybeUninit;

use usb_device::bus::UsbBusAllocator;
use usbd_hid::hid_class::{HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig};

use crate::codegen::KeyboardMatrix;
use crate::setup::UsbBus;

mod descriptor;
mod keymaps;
mod report;
#[cfg(feature = "silverplate")]
mod silverplate;

/// HID class for a keyboard device.
#[unsafe(link_section = ".uninit.HID_CLASS")]
static mut HID_CLASS: MaybeUninit<HIDClass<'static, UsbBus>> = MaybeUninit::uninit();

#[derive(Debug)]
pub struct KeyboardInstance {
	matrix: KeyboardMatrix,
	is_nkro: bool,
	report_state: report::ReportState,
	keymaps_state: keymaps::KeymapsState,
}

impl KeyboardInstance {
	/// Creates a new [`KeyboardInstance`].
	///
	/// # Safety
	///
	/// This method sets the value of a `static mut` and should only be called once to prevent the previous values
	/// from being leaked and other USB device issues that could occur.
	pub unsafe fn new(usb_bus_alloc: &'static UsbBusAllocator<UsbBus>, matrix: KeyboardMatrix) -> Self {
		// Set the value of the HID static.
		let hid_settings = HidClassSettings {
			subclass: HidSubClass::NoSubClass,
			protocol: HidProtocol::Keyboard,
			config: ProtocolModeConfig::ForceReport,
			locale: HidCountryCode::US,
		};

		// TODO: Find a way to switch between boot and report mode.

		let is_nkro = true;

		let report_descriptor = if is_nkro {
			descriptor::DESCRIPTOR_NKRO
		} else {
			descriptor::DESCRIPTOR_6KRO
		};

		let hid_class = HIDClass::new_with_settings(usb_bus_alloc, report_descriptor, 1, hid_settings);

		let ptr = &raw mut HID_CLASS;

		// SAFETY: `ptr` was obtained from a static value and so is guaranteed to be non-null and properly
		// aligned. This sets the value of the MaybeUninit.
		unsafe {
			(*ptr).write(hid_class);
		}

		Self {
			matrix,
			is_nkro,
			report_state: report::ReportState::new(),
			keymaps_state: keymaps::KeymapsState::new(),
		}
	}

	/// Scans the keyboard matrix, constructs a HID report, and sends it over USB (if changed).
	/// A critical section is used to ensure safe, exclusive access to global mutable state.
	pub fn send_pressed_keys(&mut self) {
		let scanned_keys = self.matrix.get_pressed_keys();

		let report_opt = if self.is_nkro {
			self.report_state
				.build_nkro_report(&mut self.keymaps_state, &scanned_keys)
		} else {
			self.report_state
				.build_6kro_report(&mut self.keymaps_state, &scanned_keys)
		};

		if let Some(report) = report_opt {
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

				_ = hid_class.push_raw_input(report);
			});
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

pub fn process_incoming_report(keyboard_hid: &mut HIDClass<UsbBus>) {
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
					process_led_report(buf[1]);
				}
				silverplate::VEND_REP_ID_OUT => {
					let vendor_byte = buf[1];

					silverplate::process_vendor_report(keyboard_hid, vendor_byte);
				}
				_ => {}
			}
		}
		_ => {
			// The report should contain the reportID and LED byte.
			if rep_size != 2 {
				return;
			}

			process_led_report(buf[1]);
		}
	}
}

fn process_led_report(led_byte: u8) {
	use qubit_config::keyboard::keycodes::{
		KM_LALT, KM_LCTRL, KM_LMETA, KM_LSHIFT, KM_RALT, KM_RCTRL, KM_RMETA, KM_RSHIFT,
	};

	let mut left_ctrl = false;
	let mut left_shift = false;
	let mut left_alt = false;
	let mut left_meta = false;
	let mut right_ctrl = false;
	let mut right_shift = false;
	let mut right_alt = false;
	let mut right_meta = false;

	let is_left_ctrl = (led_byte & KM_LCTRL.get()) != 0;
	let is_left_alt = (led_byte & KM_LALT.get()) != 0;
	let is_left_shift = (led_byte & KM_LSHIFT.get()) != 0;
	let is_left_meta = (led_byte & KM_LMETA.get()) != 0;
	let is_right_ctrl = (led_byte & KM_RCTRL.get()) != 0;
	let is_right_shift = (led_byte & KM_RSHIFT.get()) != 0;
	let is_right_alt = (led_byte & KM_RALT.get()) != 0;
	let is_right_meta = (led_byte & KM_RMETA.get()) != 0;

	#[cfg(feature = "defmt")]
	{
		if is_left_ctrl == left_ctrl {
			defmt::info!("Received left CTRL LED report!");
		}
		if is_left_shift == left_shift {
			defmt::info!("Received left SHIFT LED report!");
		}
		if is_left_alt == left_alt {
			defmt::info!("Received left ALT LED report!");
		}
		if is_left_meta == left_meta {
			defmt::info!("Received left META LED report!");
		}
		if is_right_ctrl == right_ctrl {
			defmt::info!("Received right CTRL LED report!");
		}
		if is_right_shift == right_shift {
			defmt::info!("Received right SHIFT LED report!");
		}
		if is_right_alt == right_alt {
			defmt::info!("Received right ALT LED report!");
		}
		if is_right_meta == right_meta {
			defmt::info!("Received right META LED report!");
		}
	}

	#[allow(unused_assignments)]
	{
		left_ctrl = is_left_ctrl;
		left_shift = is_left_shift;
		left_alt = is_left_alt;
		left_meta = is_left_meta;
		right_ctrl = is_right_ctrl;
		right_shift = is_right_shift;
		right_alt = is_right_alt;
		right_meta = is_right_meta;
	}
}
