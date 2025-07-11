#![feature(strict_overflow_ops)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod general;
pub mod keyboard;
#[cfg(feature = "std")]
pub mod linker;
#[cfg(feature = "std")]
pub mod parse;
pub mod usb;
