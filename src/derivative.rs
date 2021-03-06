//! Module for working with derivatives.
//!
//! This module has functions for estimating and evaluating
//! derivatives of functions and for computing the slope and
//! concavity of functions at single points.

pub use super::func::*;

/// The value used for `h` in derivative estimates.
///
/// This value is chosen so as to offer the best accuracy, 
/// it is a compromise between the increase in accuracy caused 
/// by having an `h` closer to zero, and the decrease decrease 
/// in accuracy caused by floating point imprecision with very 
/// small values.
pub const EPSILON: f64 = 5.0e-7;

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
            (f_copy(x + EPSILON) - f_copy(x - EPSILON)) / (EPSILON * 2.0)
    });

    match n {
        0 => f.clone(),
        1 => deriv,
        _ => nth_derivative(n - 1, &deriv),
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
    nth_derivative(1, f)
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
    nth_derivative(2, f)
}

/// Estimate the value of the derivative of `f` at `x`
///
/// This function works by applying the limit definition of
/// the derivative at `x` in the same way that `nth_derivative()`
/// does. See the documentation for `nth_derivative()` for more
/// information.
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
    (f(x + EPSILON) - f(x - EPSILON)) / (EPSILON * 2.0)
}

/// Estimate the value of the second derivative of `f` at `x`
///
/// This function works by applying the limit definition of
/// the derivative twice, once to estimate the first derivative of `f()`
/// at two points, then once again to estimate the concavity.
///
/// The calculation is equivalent to:
///
/// ```text
///                     f(x + h) - 2f(x) * 2 + f(x - h)
/// f''(x) = lim        -------------------------------
///          h -> 0                    h^2 
/// ```
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
      (f(x + EPSILON * 2.0) - f(x) * 2.0 + f(x - EPSILON * 2.0)) 
    / (EPSILON * 4.0 * EPSILON)
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_nth_derivative() {
        let f = func!(|x: f64| x * x * x + 5.0);
        let f_deriv = derivative(&f);
        let f_s_deriv = second_derivative(&f);

        assert_fp!(f( 0.0),  5.0,  0.0001);
        assert_fp!(f( 4.0),  69.0, 0.0001);
        assert_fp!(f(-2.0), -3.0,  0.0001);

        assert_fp!(f_deriv( 0.0), 0.0,  0.001);
        assert_fp!(f_deriv( 4.0), 48.0, 0.001);
        assert_fp!(f_deriv(-2.0), 12.0, 0.001);

        assert_fp!(f_s_deriv( 0.0),  0.0,  0.1);
        assert_fp!(f_s_deriv( 4.0),  24.0, 0.1);
        assert_fp!(f_s_deriv(-2.0), -12.0, 0.1);

    }

#[test]
    fn t_helpers() {
        let f = func!(|x: f64| x * x);
        let f_deriv = derivative(&f);
        let f_s_deriv = second_derivative(&f);
        let f_deriv_2 = nth_derivative(1, &f);
        let f_s_deriv_2 = nth_derivative(2, &f);

        assert_eq!(f_deriv(0.0),  f_deriv_2(0.0));
        assert_eq!(f_deriv(10.4), f_deriv_2(10.4));
        assert_eq!(f_deriv(56.8), f_deriv_2(56.8));

        assert_eq!(f_s_deriv(0.0),  f_s_deriv_2(0.0));
        assert_eq!(f_s_deriv(40.4), f_s_deriv_2(40.4));
        assert_eq!(f_s_deriv(12.3), f_s_deriv_2(12.3));

        assert_eq!(f_deriv(0.0),  slope_at(&f, 0.0));
        assert_eq!(f_deriv(10.4), slope_at(&f, 10.4));
        assert_eq!(f_deriv(56.8), slope_at(&f, 56.8));

        assert_eq!(f_s_deriv(0.0),  concavity_at(&f, 0.0));
        assert_eq!(f_s_deriv(40.4), concavity_at(&f, 40.4));
        assert_eq!(f_s_deriv(12.3), concavity_at(&f, 12.3));
    }
}
