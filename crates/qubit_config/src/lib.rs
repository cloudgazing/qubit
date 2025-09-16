#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "build")]
pub mod cargo;
pub mod general;
pub mod keyboard;
pub mod mcu;
#[cfg(feature = "std")]
pub mod parse;
pub mod usb;
pub mod version;
