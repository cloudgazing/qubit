#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

pub mod config;
pub mod keycodes;
#[cfg(feature = "std")]
pub mod linker;
