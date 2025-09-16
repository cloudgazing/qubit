use core::num::NonZeroU8;

use crate::codegen::{KEYMAPS, Keymaps};

#[used]
#[unsafe(link_section = ".qubit.DEFAULT_KEYMAPS")]
static DEFAULT_KEYMAPS: Keymaps = KEYMAPS;

#[derive(Debug, Clone, Copy)]
pub enum Layer {
	L0,
	L1,
	L2,
	L3,
	L4,
}

#[derive(Debug)]
pub struct KeymapsState {
	active_keymaps: Keymaps,
	active_layers: u8,
}

impl KeymapsState {
	pub fn new() -> Self {
		let active_keymaps = Keymaps {
			keymap_0: DEFAULT_KEYMAPS.keymap_0,
			keymap_1: DEFAULT_KEYMAPS.keymap_1,
			keymap_2: DEFAULT_KEYMAPS.keymap_2,
			keymap_3: DEFAULT_KEYMAPS.keymap_3,
			keymap_4: DEFAULT_KEYMAPS.keymap_4,
		};

		Self {
			active_keymaps,
			active_layers: 0b0000_0001,
		}
	}

	pub fn get_keycode(&self, idx: usize) -> NonZeroU8 {
		const NO_KEY: NonZeroU8 = NonZeroU8::new(1).unwrap();

		let mut active_layers = self.active_layers;

		while active_layers != 0 {
			let set_bit = 7 - active_layers.leading_zeros() as usize;

			let code = match set_bit {
				0 => self.active_keymaps.keymap_0[idx],
				1 => self.active_keymaps.keymap_1[idx],
				2 => self.active_keymaps.keymap_2[idx],
				3 => self.active_keymaps.keymap_3[idx],
				4 => self.active_keymaps.keymap_4[idx],
				_ => panic!(),
			};

			if let Some(code) = NonZeroU8::new(code) {
				return code;
			}

			active_layers &= !(1 << set_bit);
		}

		NO_KEY
	}

	pub fn enable_layer(&mut self, layer: Layer) {
		let layer_num: u8 = match layer {
			Layer::L0 => 0,
			Layer::L1 => 1,
			Layer::L2 => 2,
			Layer::L3 => 3,
			Layer::L4 => 4,
		};

		self.active_layers |= 1 << layer_num;
	}

	pub fn disable_layer(&mut self, layer: Layer) {
		let layer_num: u8 = match layer {
			Layer::L0 => 0,
			Layer::L1 => 1,
			Layer::L2 => 2,
			Layer::L3 => 3,
			Layer::L4 => 4,
		};

		self.active_layers &= !(1 << layer_num);
	}

	// pub fn toggle_layer(&mut self, layer: Layer) {
	// 	let layer_num: u8 = match layer {
	// 		Layer::L0 => 0,
	// 		Layer::L1 => 1,
	// 		Layer::L2 => 2,
	// 		Layer::L3 => 3,
	// 		Layer::L4 => 4,
	// 	};

	// 	self.active_layers ^= 1 << layer_num;
	// }
}
