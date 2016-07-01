#![warn(missing_docs)]

//! A fast and lightweight math library
//!
//! `reikna` contains implementations of various useful
//! functions, structs, and algorithms from various branches
//! of mathematics, including number theory, graph theory,
//! and calculus. The library is designed with speed and
//! ease of use in mind.

#[macro_use] pub mod func;

pub mod derivative;
pub mod factor;
pub mod figurate;
pub mod partition;
pub mod prime;
