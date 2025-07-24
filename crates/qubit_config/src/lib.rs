#![feature(strict_overflow_ops)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "build")]
pub mod cargo;
pub mod general;
pub mod keyboard;
#[cfg(feature = "build")]
pub mod linker;
pub mod mcu;
#[cfg(feature = "std")]
pub mod parse;
pub mod usb;
pub mod version;
