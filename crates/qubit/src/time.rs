//! A module which defines time related implementations.

use rp2040_hal as hal;

use hal::Timer;
use hal::fugit::MicrosDurationU64;

/// A `CountDown` implementation that was part of `embedded_hall` 2.0.
pub struct CountDown {
	timer: Timer,
	period: MicrosDurationU64,
	next_end: Option<u64>,
}

impl CountDown {
	#[must_use]
	pub fn new(timer: Timer) -> Self {
		Self {
			timer,
			period: MicrosDurationU64::nanos(0),
			next_end: None,
		}
	}
}

impl CountDown {
	/// Start the countdown.
	pub fn start<T: Into<MicrosDurationU64>>(&mut self, count: T) {
		self.period = count.into();

		let next_end = self.timer.get_counter().ticks();
		self.next_end = Some(next_end.wrapping_add(self.period.to_micros()));
	}

	/// "Blockingly" wait until the countdown is finished.
	///
	/// # Errors
	///
	/// Returns an error if the countdown is not finished. The recommended way to use this
	/// is to call `.is_ok()` on every iteration to check when the countdown is finished.
	///
	/// # Panics
	///
	/// Panics if the countdown was not started.
	pub fn wait(&mut self) -> Result<(), &'static str> {
		if let Some(end) = self.next_end {
			let current_ticks = self.timer.get_counter().ticks();

			if current_ticks >= end {
				self.next_end = Some(end.wrapping_add(self.period.to_micros()));

				Ok(())
			} else {
				Err("not finished")
			}
		} else {
			panic!("CountDown not started!");
		}
	}

	/// Cancel the countdown.
	///
	/// # Errors
	///
	/// Returns an error if the countdown was not started.
	#[allow(dead_code, reason = "Allow for now as I might use this method at some point.")]
	pub fn cancel(&mut self) -> Result<(), &'static str> {
		if self.next_end.is_none() {
			Err("CountDown is not running.")
		} else {
			self.next_end = None;
			Ok(())
		}
	}
}
