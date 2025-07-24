#[cfg(mcu = "rp2040")]
mod rp2040;
#[cfg(mcu = "rp2040")]
use rp2040 as mcu;

#[cfg(mcu = "stm32f411")]
mod stm32f411;
#[cfg(mcu = "stm32f411")]
use stm32f411 as mcu;

pub use mcu::*;
