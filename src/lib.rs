#![warn(missing_docs)]
//! doomguy_rs
//!
//! A collection of tools and utilities for reading, parsing, modifying and writing various WAD types.

mod error;
pub use error::DoomguyError;

pub mod wad;
