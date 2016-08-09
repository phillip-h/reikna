//! Module for working with integrals.
//!
//! This module has functions for estimating the values of
//! integrals through numeric integration techniques.

pub use super::func::*;

/// The default precision constant used in `integrate`.
///
/// This value can be thought of as the number of subintervals to use
/// for each integral interval in the region `[a, b`].
pub const DEFAULT_PRECISION: u64 = 4;

/// Estimate the value of the integral of `f` over `[a, b]` using
/// `p` subintervals.
///
/// This function works by applying Simpson's rule to the function
/// over the specified interval, using `p` subintervals.
///
/// Note that a higher `p` will increase the accuracy of the result,
/// but also increase the time the computation takes. `p` should be chosen
/// to ensure that a good estimate can be made without drastically 
/// increasing the computational complexity.
///
/// If `a` is equal to `b` or `p` equals zero, `zero` will be
/// returned.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::integral::*;
/// 
/// let f = func!(|x| x + 4.0);
/// assert_eq!(integrate_wp(&f, 0.0, 0.0, 10), 0.0);
/// assert_eq!(integrate_wp(&f, 0.0, 1.0, 10), 4.5);
///# }
/// ```
pub fn integrate_wp(f: &Function, a: f64, b: f64, p: u64) -> f64 {
    if a == b || p == 0 {
        return 0.0;
    }

    let delta = (b - a) / p as f64;

    let mut integral = f(a) + f(b);
    let mut pos = a;
    for i in 1..p {
        pos += delta;
        if i & 0x01 == 0 {
            integral += 2.0 * f(pos);
        } else {
            integral += 4.0 * f(pos);
        }
    }

    integral * delta / 3.0
}

/// Estimate the value of the integral of `f` over `[a, b]`.
///
/// This is a helper function that calls `integrate_wp()` using
/// a `p` value calculated depending on the size of `[a, b]`. See
/// the documentation for `integrate_wp()` for more information.
///
/// The value of `p` is calculated by the following formula:
///
/// ``` text
/// p = round(|b - a|) * precision
/// ```
///
/// Where `precision` is the constant `DEFAULT_PRECISION`.
///
/// Note -- because of the way the precision is calculated, the
/// computational complexity of this function grows linearly with
/// the size of the interval. Very large intervals will have very
/// large precision values, which can slow down computation while not
/// providing a large improvement to accuracy. For very large intervals,
/// is is better to use `integrate_wp()` directly, so the precision value
/// can be set to a more reasonable target.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::integral::*;
/// 
/// let f = func!(|x| x + 4.0);
/// assert_eq!(integrate(&f, 0.0, 0.0), 0.0);
/// assert_eq!(integrate(&f, 0.0, 1.0), 4.5);
///# }
/// ```
pub fn integrate(f: &Function, a: f64, b: f64) -> f64 {
    let p = (b - a).abs().round() as u64 * DEFAULT_PRECISION;
    integrate_wp(f, a, b, p)
}

/// Return a `Function` that estimates the `n`th integral of `f`, using a
/// constant of `c` and a positive precision constant of `p`.
///
/// The integration itself is done by `integrate_wp()`, see the
/// documentation for `integrate_wp()` for more information.
///
/// The precision value passed to `integrate_wp()` is calculated with the
/// following formula:
///
/// ``` text
/// precision = round(|x|) * p
/// ```
///
/// Where `p` is the precision constant supplied to this function.
///
/// Note -- the computational complexity of the resulting function grows
/// exponentially based on the value of `n`, IE:
///
/// ``` text
/// nth_integral(1, f, c, p)(x) -> O(1^x)
/// nth_integral(2, f, c, p)(x) -> O(2^x)
/// nth_integral(3, f, c, p)(x) -> O(3^x)
/// ```
/// 
/// where `x` is the value of `x` supplied to the resulting function.
/// For this reason it is not recommended to use values of `n` greater than
/// three or four, as larger values will cause computational complexity to
/// rapidly inflate.
///
/// # Panics
///
/// Panics if `p` equals zero.
/// 
/// # Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::integral::*;
/// 
/// let f = func!(|x| x * x);
/// let integral = nth_integral(1, &f, 1.0, DEFAULT_PRECISION);
/// 
/// println!("integral({}) = {}",  0.0, integral( 0.0));
/// println!("integral({}) = {}",  1.0, integral( 1.0));
/// println!("integral({}) = {}", -2.0, integral(-2.0));
///# }
/// ```
///
/// Outputs:
///
/// ```text
/// integral(0.0) = 1.0
/// integral(1.0) = 1.3333333333333333
/// integral(-2.0) = -1.6666666666666665
/// ```
pub fn nth_integral(n: u64, f: &Function, c: f64, p: u64) -> Function {
    assert!(p != 0, "Precision constant must be positive!");

    let f_copy = f.clone();
    let integral: Function = func!(
        move |x: f64| {
            let prec = x.abs().round() as u64 * p;
            integrate_wp(&f_copy, 0.0, x, prec) + c
    });

    match n {
        0 => f.clone(),
        1 => integral,
        _ => nth_integral(n - 1, &integral, c, p),
    }
}

/// Return a `Function` that estimates the integral of `f`, using a
/// constant of `c` and a positive precision constant of `p`.
///
/// This is a helper function that calls `nth_integral()` with an
/// `n` value of `1`. See the documentation for `nth_integral()` for
/// more information.
///
/// # Panics
///
/// Panics if `nth_integral()` panics. See the documentation of
/// `nth_integral()` for more information.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate reikna;
/// # fn main() {
/// use reikna::integral::*;
/// 
/// let f = func!(|x| x * x);
/// let integral = integral(&f, 1.0, DEFAULT_PRECISION);
/// 
/// println!("integral({}) = {}",  0.0, integral( 0.0));
/// println!("integral({}) = {}",  1.0, integral( 1.0));
/// println!("integral({}) = {}", -2.0, integral(-2.0));
///# }
/// ```
///
/// Outputs:
///
/// ```text
/// integral(0.0) = 1.0
/// integral(1.0) = 1.3333333333333333
/// integral(-2.0) = -1.6666666666666665
/// ```
pub fn integral(f: &Function, c: f64, p: u64) -> Function {
    nth_integral(1, f, c, p)
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_integrate() {
        let f = func!(|x: f64| x * x);
        assert_fp!(integrate(&f,  0.0, 0.0),  0.0);
        assert_fp!(integrate(&f, -1.0, 1.0),  2.0 / 3.0);
        assert_fp!(integrate(&f,  0.0, 1.0),  1.0 / 3.0);
        assert_fp!(integrate(&f, -1.0, 0.0),  1.0 / 3.0);
        assert_fp!(integrate(&f,  1.0, 0.0), -1.0 / 3.0);

        assert_fp!(integrate(&f,  0.0, 1000.0), integrate(&f, -1000.0, 0.0));
        assert_fp!(integrate(&f,  13.0, 0.0),  -integrate(&f, 0.0, 13.0));

        let f_int = nth_integral(1, &f, 0.0, 2);
        assert_fp!(f_int( 0.0),  0.0);
        assert_fp!(f_int( 1.0),  1.0 / 3.0);
        assert_fp!(f_int(-1.0), -1.0 / 3.0);

        let f_int = nth_integral(1, &f, 1.0, 2);
        assert_fp!(f_int( 0.0),  1.0);
        assert_fp!(f_int( 1.0),  4.0 / 3.0);
        assert_fp!(f_int(-1.0),  2.0 / 3.0);

        let f_int = nth_integral(2, &f, 0.0, 2);
        assert_fp!(f_int( 0.0),  0.0);
        assert_fp!(f_int( 1.0), 1.0 / 12.0);
        assert_fp!(f_int(-1.0), 1.0 / 12.0);
    }

#[test]
#[should_panic]
    fn t_integrate_panic() {
        let f = func!(|x: f64| x * x);
        nth_integral(1, &f, 1.0, 0);
    }
}
