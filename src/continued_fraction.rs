//! Module for working with continued fractions.
//!
//! This module has functions for generating continued fraction
//! representations of square roots and `e`, and functions for
//! expanding continued fractions into simple fractions and floating
//! point formats.

use std::mem;

/// Type alias for continued fractions.
///
/// These are `Vec<u64`s of the form:
///
/// `[a; b, c, d, ...]`
///
/// where `a` is the initial term of the fraction,
/// and `b, c, d, ...` are repeating terms in the
/// case of an infinite fraction, or simply the 
/// other terms in the case of a finite fraction.
pub type ContinuedFraction = Vec<u64>;

/// Return a `ContinuedFraction` representing the square root of `x`.
///
/// This is an infinite continued fraction that should be expanded
/// multiple times to obtain an accurate value.
///
/// If `x` is a perfect square, a `ContinuedFraction` will be returned
/// containing the square root of `x`.
///
/// # Examples
///
/// ```
/// use reikna::continued_fraction::square_root;
/// assert_eq!(square_root(19), vec![4, 2, 1, 3, 1, 2, 8]);
/// assert_eq!(square_root(25), vec![5]);
/// ```
pub fn square_root(x: u64) -> ContinuedFraction {
    let a0: f64 = (x as f64).sqrt();

    let mut m: u64 = 0;
    let mut d: u64 = 1;
    let mut a: u64 = a0.floor() as u64;

    let mut expansion: ContinuedFraction = vec![a];

    if (a0 - a0.floor()).abs() < ::std::f64::EPSILON {
        return expansion;
    }

    let end: u64 = a * 2;
    while a != end {
        m = d * a - m;
        d = (x - m * m) / d;
        a = ((a0 + m as f64) / d as f64).floor() as u64;
        expansion.push(a);
    }

    expansion
}

/// Return a `ContinuedFraction` of the continued fraction representing
/// `e` to `n` terms.
///
/// The general form of this expansion is:
///
/// ```text
/// e = [2; 1, x, 1, 1, x, ...]
/// ```
/// where `x` starts at `2` and increases by `2` each time.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
/// 
/// ```
/// use reikna::continued_fraction::e;
/// assert_eq!(e(4), vec![2, 1, 2, 1]);
/// assert_eq!(e(8), vec![2, 1, 2, 1, 1, 4, 1, 1]);
/// ```
pub fn e(n: u64) -> ContinuedFraction {
    assert!(n != 0, "cannot produce continued fraction of zero length!"); 

    let mut frac: ContinuedFraction = Vec::with_capacity(n as usize);
    frac.push(2);

    let mut val = 2;
    for i in 0..(n - 1) {
        match i % 3 {
            1 => {
                     frac.push(val);
                     val += 2
                 }
            _ => frac.push(1),
        }
    }

    frac
}

/// Expand the continued fraction `fraction` `n` times, storing
/// the result as a fraction in a double tuple of `u64`.
///
/// The result tuple is formatted as:
///
/// ```text
/// (numerator, denominator)
/// ```
///
/// Finite fractions should be expanded with `n = 1`,
/// infinite fractions should be expanded with an `n`
/// large enough to gain the desired precision.
///
/// Note that is `n` is large or the continued fraction
/// is very long, the `u64`s representing the numerator and denominator
/// may overflow.
///
/// # Panics
///
/// Panics if `n` is zero or if `fraction` is empty.
///
/// # Examples
///
/// ```
/// use reikna::continued_fraction::expand_fraction_ntimes;
/// assert_eq!(expand_fraction_ntimes(&vec![1, 2], 3), (41, 29));
/// assert_eq!(expand_fraction_ntimes(&vec![14], 2), (14, 1));
/// ```
pub fn expand_fraction_ntimes(fraction: &ContinuedFraction, 
                              n: u64) -> (u64, u64) {
    assert!(fraction.len() != 0, "cannot expand empty continued fraction!");
    assert!(n != 0, "cannot expand continued fraction zero times!");

    if fraction.len() == 1 {
        return (fraction[0], 1);
    }

    let mut frac = Vec::with_capacity(fraction.len() * n as usize);
    frac.extend_from_slice(fraction);
    for _ in 0..(n - 1) {
        frac.extend_from_slice(&fraction[1..]);
    }

    let mut num = 1; 
    let mut den = *frac.last().unwrap();
    for i in (1..frac.len()).rev() {
        num += frac[i] * den;
        mem::swap(&mut den, &mut num);
    }
    
    num += frac[0] * den;

    (num, den)
}

/// Expand the continued fraction `fraction` one time, storing
/// the result as a fraction in a double tuple of `u64`.
///
/// This is a helper function that calls `expand_fraction_ntimes()`
/// with `n = 1`. See the documentation for `expand_fraction_ntimes()`
/// for more information.
///
/// # Panics
/// 
/// Panics if `expand_fraction_ntimes()` panics.
///
/// # Examples
///
/// ```
/// use reikna::continued_fraction::expand_fraction;
/// assert_eq!(expand_fraction(&vec![2, 1]), (5, 2));
/// assert_eq!(expand_fraction(&vec![3]), (3, 1));
/// ```
pub fn expand_fraction(fraction: &ContinuedFraction) -> (u64, u64) {
    expand_fraction_ntimes(fraction, 1)
}

/// Expand the continued fraction `fraction` `n` times, storing
/// the result as an `f64`.
///
/// Finite fractions should be expanded with `n = 1`,
/// infinite fractions should be expanded with an `n`
/// large enough to gain the desired precision.
///
/// Note that is `n` is large or the continued fraction
/// is very long, the `f64`s representing the numerator and denominator
/// during the expansion may overflow.
///
/// # Panics
///
/// Panics if `n` is zero or if `fraction` is empty.
///
/// # Examples
///
/// ```
/// use reikna::continued_fraction::expand_f64_ntimes;
/// println!("[1; 2] = {}", expand_f64_ntimes(&vec![1, 2], 1));
/// println!("[14] = {}", expand_f64_ntimes(&vec![14], 5));
/// ```
///
/// Outputs:
///
/// ``` text
/// [1; 2] = 1.2
/// [14] = 14
/// ```
pub fn expand_f64_ntimes(fraction: &ContinuedFraction, 
                              n: u64) -> f64 {
    assert!(fraction.len() != 0, "cannot expand empty continued fraction!");
    assert!(n != 0, "cannot expand continued fraction zero times!");

    if fraction.len() == 1 {
        return fraction[0] as f64
    }

    let mut frac = Vec::with_capacity(fraction.len() * n as usize);
    frac.extend_from_slice(fraction);
    for _ in 0..(n - 1) {
        frac.extend_from_slice(&fraction[1..]);
    }

    let mut num = 1f64; 
    let mut den = *frac.last().unwrap() as f64;
    for i in (1..frac.len()).rev() {
        num += frac[i] as f64 * den;
        mem::swap(&mut den, &mut num);
    }
    
    num += frac[0] as f64 * den;
    num /= den;

    num
}

/// Expand the continued fraction `fraction` one time, storing
/// the result as an `f64`.
///
/// This is a helper function that calls `expand_f64_ntimes()`
/// with `n = 1`. See the documentation for `expand_f64_ntimes()`
/// for more information.
///
/// # Panics
/// 
/// Panics if `expand_f64_ntimes()` panics.
///
/// # Examples
///
/// ```
/// use reikna::continued_fraction::expand_f64;
/// println!("[2, 1] = {}", expand_f64(&vec![2, 1]));
/// println!("[3] = {}", expand_f64(&vec![3]));
/// ```
///
/// Outputs:
///
/// ``` text
/// [2; 1] = 2.5
/// [3] = 3
/// ```
pub fn expand_f64(fraction: &ContinuedFraction) -> f64 {
    expand_f64_ntimes(fraction, 1)
}


/// Return a nicely formatted `String` of the continued fraction
/// `fraction.
///
/// # Examples
///
/// ```
/// use reikna::continued_fraction::to_string;
/// println!("{}", to_string(&vec![1, 2, 3, 4]));
/// println!("{}", to_string(&vec![17]));
/// ```
///
/// Outputs:
///
/// ``` text
/// [1; 2, 3, 4]
/// [17]
/// ```
pub fn to_string(fraction: &ContinuedFraction) -> String {
    if fraction.is_empty() {
        return "[]".to_string();
    }

    let mut string = "[".to_string();
    string.push_str(&fraction[0].to_string());
    if fraction.len() != 1 {
        string.push_str("; ");
        for i in 1..fraction.len() {
            if i != 1 {
                string.push_str(", ");
            }
            string.push_str(&fraction[i].to_string());
        }
    }
    string.push_str("]");

    string
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_square_root() {
        assert_eq!(square_root(0),  vec![0]);
        assert_eq!(square_root(1),  vec![1]);
        assert_eq!(square_root(2),  vec![1, 2]);
        assert_eq!(square_root(4),  vec![2]);
        assert_eq!(square_root(5),  vec![2, 4]);
        assert_eq!(square_root(10), vec![3, 6]);
        assert_eq!(square_root(14), vec![3, 1, 2, 1, 6]);
        assert_eq!(square_root(17), vec![4, 8]);
        assert_eq!(square_root(20), vec![4, 2, 8]);
    }

#[test]
    fn t_e() {
        assert_eq!(e(1),  vec![2]);
        assert_eq!(e(2),  vec![2, 1]);
        assert_eq!(e(3),  vec![2, 1, 2]);
        assert_eq!(e(4),  vec![2, 1, 2, 1]);
        assert_eq!(e(10), vec![2, 1, 2, 1, 1, 4, 1, 1, 6, 1]);
    }

#[test]
#[should_panic]
    fn t_e_panic() {
        e(0);
    }

#[test]
    fn t_expand_fraction() {
        assert_eq!(expand_fraction_ntimes(&square_root(4), 1), (2, 1));
        assert_eq!(expand_fraction_ntimes(&square_root(2), 1), (7, 5));
        assert_eq!(expand_fraction_ntimes(&square_root(2), 2), (17, 12));
        assert_eq!(expand_fraction_ntimes(&square_root(5), 1), (38, 17));
    }

#[test]
#[should_panic]
    fn t_expand_fraction_panic() {
        expand_fraction(&vec![]);
    }

#[test]
#[should_panic]
    fn t_expand_fraction_panic_2() {
        expand_fraction_ntimes(&vec![1, 2], 0);
    }

#[test]
    fn t_expand_f64() {

        assert_fp!(expand_f64_ntimes(&square_root(4), 1), 2.0);
        assert_fp!(expand_f64_ntimes(&square_root(2), 1), 1.4);
        assert_fp!(expand_f64_ntimes(&square_root(2), 2), 1.416);
        assert_fp!(expand_f64_ntimes(&square_root(5), 1), 2.235);
    }

#[test]
#[should_panic]
    fn t_expand_f64_panic() {
        expand_f64(&vec![]);
    }

#[test]
#[should_panic]
    fn t_expand_f64_panic_2() {
        expand_f64_ntimes(&vec![1, 2], 0);
    }

#[test]
    fn t_to_string() {
        assert_eq!(to_string(&vec![]), "[]".to_string());
        assert_eq!(to_string(&vec![17]), "[17]".to_string());
        assert_eq!(to_string(&vec![1, 2, 3]), "[1; 2, 3]".to_string());
    }
}
