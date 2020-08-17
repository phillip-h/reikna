//! Module for working with `Function`s
//!
//! This module contains a type alias for `Rc<Fn(f64) -> f64>`,
//! which is used in many other modules, and functions for
//! working with the alias.

pub use std::rc::Rc;

/// Type alias used to represent functions.
///
/// A `Function` is a `Fn` that takes a single `f64`,
/// does something with it, and returns another `f64`.
///
/// Functions are stored in an `Rc` so they can be `cloned()`
/// and subsequently consumed in other functions.
pub type Function = Rc<dyn Fn(f64) -> f64>;

/// Macro for creating a `Function`.
///
/// More idiomatic than calling `Rc::new()`.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate reikna;
/// # fn main() {
/// use reikna::func::*;
/// let f: Function = func!(|x| x * x);
/// assert_eq!(f(5.0), 25.0);
/// # }
/// ```
#[macro_export]
macro_rules! func {
    ($e:expr) => (Rc::new($e) as Function);
}
