//! Module for working with derivatives.
//!
//! This module has functions for estimating and evaluating
//! derivatives of functions with one or more variables.

pub use super::func::*;

/// The value used for `h` in derivative estimates.
///
/// This value is chosen so as to offer the best accuracy, 
/// it is a compromise between the increase in accuracy caused 
/// by having an `h` closer to zero, and the decrease decrease 
/// in accuracy caused by floating point imprecision with very 
/// small values.
pub const EPSILON: f64 = 10.0e-7;

/// Return a `Function` estimating the `n`th derivative of `f`.
///
/// This function will return a `Function` that estimates the
/// `n`th derivative of `f` using the limit definition of the
/// derivative:
///
/// ```text
///                     f(x + h) - f(x)
/// f'(x) = lim         ---------------
///         h -> 0             h
/// ```
///
/// Where `h` is equal to `EPSILON`. See the documentation for
/// `EPSILON` for more information.
///
/// This function will use recursion to provide derivatives for `n > 1`.
///
/// It is important to note that the inaccuracy of the derivative
/// estimates compound each other, the higher `n` is, the less precise
/// the resulting function will be!
///
/// If `n = 0`, then a `clone()` of `f` is returned.
///
/// Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::derivative::*;
///
/// let f = func![|x| x * x];
///
/// let first_deriv = nth_derivative(1, &f);
/// let second_deriv = nth_derivative(2, &f);
///
/// println!("f(5)   = {}", f(5.0));
/// println!("f'(5)  = {}", first_deriv(5.0));
/// println!("f''(5) = {}", second_deriv(5.0));
/// # }
///
/// ```
///
/// Outputs:
///
/// ``` text
/// f(5)   = 25
/// f'(5)  = 10.00000100148668
/// f''(5) = 2.000177801164682
/// ```
pub fn nth_derivative(n: u64, f: &Function) -> Function {

    let f_copy = f.clone();
    let deriv: Function = func!(
        move |x: f64| {
            (f_copy(x + EPSILON) - f_copy(x)) / EPSILON
    });

    match n {
        0 => f.clone(),
        1 => deriv,
        _ => nth_derivative(n - 1, & deriv),
    }
}

/// Return a function estimating the first derivative of `f`.
///
/// This is a helper function and is equivalent to calling
/// `nth_derivative(1, f)`.
///
/// Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::derivative::*;
///
/// let f = func![|x| x * x];
///
/// let first_deriv = derivative(&f);
///
/// println!("f(5)  = {}", f(5.0));
/// println!("f'(5) = {}", first_deriv(5.0));
/// # }
///
/// ```
///
/// Outputs:
///
/// ``` text
/// f(5)   = 25
/// f'(5)  = 10.00000100148668
/// ```
pub fn derivative(f: &Function) -> Function {
    nth_derivative(1, &f)
}

/// Return a function estimating the second derivative of `f`.
///
/// This is a helper function and is equivalent to calling
/// `nth_derivative(2, f)`.
///
/// Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::derivative::*;
///
/// let f = func![|x| x * x];
///
/// let second_deriv = nth_derivative(2, &f);
///
/// println!("f(5)   = {}", f(5.0));
/// println!("f''(5) = {}", second_deriv(5.0));
/// # }
///
/// ```
///
/// Outputs:
///
/// ``` text
/// f(5)   = 25
/// f''(5) = 2.000177801164682
/// ```
pub fn second_derivative(f: &Function) -> Function {
    nth_derivative(2, &f)
}

/// Estimate the value of the derivative of `f` at `x`
///
/// This is a helper function that calls `derivative` and
/// then calls the resulting function with `x`.
///
/// Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::derivative::*;
///
/// let f = func![|x| (x + 4.0) * (x + 4.0)];
/// println!("f'(-4.0) = {}", slope_at(&f, -4.0));
/// # }
///
/// ```
/// Outputs:
///
/// ```text
/// f'(-4.0) = 0.000001000000000279556
/// ```
pub fn slope_at(f: &Function, x: f64) -> f64 {
    derivative(f)(x)
}

/// Estimate the value of the second derivative of `f` at `x`
///
/// This is a helper function that calls `second_derivative` and
/// then calls the resulting function with `x`.
///
/// Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::derivative::*;
///
/// let f = func![|x| (x + 4.0) * (x + 4.0)];
/// println!("f''(-4.0) = {}", concavity_at(&f, -4.0));
/// # }
///
/// ```
/// Outputs:
///
/// ```text
/// f''(-4.0) = 2.0000000005591114
/// ```
pub fn concavity_at(f: &Function, x: f64) -> f64 {
    second_derivative(f)(x)
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_nth_derivative() {
        let f = func!(|x: f64| x * x);
        assert_eq!(f(2.0), 4.0);
    }
}
