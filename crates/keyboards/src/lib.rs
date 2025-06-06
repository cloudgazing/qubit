#![cfg_attr(not(feature = "std"), no_std)]

pub mod config;
pub mod keycodes;
#[cfg(feature = "std")]
pub mod linker;
