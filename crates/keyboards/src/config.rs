#[cfg(feature = "std")]
pub mod parse;

pub type Keymap<const S: usize> = [u8; S];

pub struct Configuration<const S: usize> {
	/// Tha name of the board.
	pub name: &'static str,
	/// The board author or manufacturer.
	pub author: &'static str,
	/// The id diferentates the board from the others.
	pub id: &'static str,
	pub version: &'static str,
	pub keymap: Keymap<S>,
}
