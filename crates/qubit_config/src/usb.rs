pub struct Usb {
	pub vid: u16,
	pub pid: u16,
}

impl Usb {
	#[must_use]
	pub const fn new(vid: u16, pid: u16) -> Self {
		Self { vid, pid }
	}
}
