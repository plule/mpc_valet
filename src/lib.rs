#![warn(clippy::all, rust_2018_idioms)]
#![doc = include_str!("../README.md")]

pub mod components;
pub mod export;
pub mod model;
mod range;
mod static_iterable;

pub use range::*;
pub use static_iterable::*;
