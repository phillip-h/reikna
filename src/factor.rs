//! Module for working with integer factorization.
//!
//! This module contains functions for factoring integers, 
//! computing the LCM and GCD of integers, and testing if
//! integers are perfect squares and perfect cubes.

use std::mem;

/// Find the GCD of `a` and `b` using the Euclidian algorithm.
///
/// This function will return `0` if both arguments are zero.
///
/// # Examples
///
/// ```
/// use reikna::factor::gcd;
/// assert_eq!(gcd(76, 54), 2);
/// assert_eq!(gcd(18, 24), 6);
/// assert_eq!(gcd(5, 2), 1);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        mem::swap(&mut a, &mut b);
        b %= a;
    }

    a
}

/// Return the GCD of the set of integers
///
/// This function works by applying the fact that the
/// GCD is both communative and associative, and as such the GCD
/// of a set can be found by computing the GCD of each of its
/// members with a running GCD total.
///
/// If an empty set is given, `0` will be returned.
///
/// # Examples 
///
/// ```
/// use reikna::factor::gcd_all;
/// assert_eq!(gcd_all(&vec![16, 4, 32]), 4);
/// assert_eq!(gcd_all(&vec![3, 10, 18]), 1);
/// ```
pub fn gcd_all(set: &Vec<u64>) -> u64 {
    let mut gcd_: u64 = 0;
    for n in set {
        gcd_ = gcd(*n, gcd_);
    }

    gcd_
}

/// Return `true` if `a` and `b` are coprime.
/// 
/// This is a helper function that calls `gcd(a, b)` and checks
/// if the result is `1`.
///
/// # Examples
///
/// ```
/// use reikna::factor::coprime;
/// assert_eq!(coprime(8, 4), false);
/// assert_eq!(coprime(9, 8), true);
/// ```
pub fn coprime(a: u64, b: u64) -> bool {
    gcd(a, b) == 1
}

/// Return the LCM of `a` and `b`.
///
/// This function works by computing the GCD of `a` and `b`
/// using `gcd()`, then appying the fact that 
/// ```text
///               a * b
/// lcm(a, b) = ---------
///             gcm(a, b)
/// ```
///
/// If both `a` and `b` are zero, `0` is returned.
///
/// # Examples
///
/// ```
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 {
        return 0;
    }

    a * b / gcd(a, b)
}

/// Return the GCD of the set of integers
///
/// This function works by applying the fact that the
/// LCM is both communative and associative, and as such the LCM
/// of a set can be found by computing the LCM of each of its
/// members with a running LCM total.
///
/// If an empty set is given, `1` will be returned.
///
/// # Examples
/// 
/// ```
/// use reikna::factor::lcm_all;
/// assert_eq!(lcm_all(&vec![8, 9, 21]), 504);
/// assert_eq!(lcm_all(&vec![4, 7, 12, 21, 42]), 84);
/// ```
pub fn lcm_all(set: &Vec<u64>) -> u64 {
    let mut lcm_ = 1;
    for n in set {
        lcm_ = lcm(*n, lcm_);
    }

    lcm_
}

/// List of least significant bytes for values 
/// that could be perfect squares.
pub const GOOD_BYTES: [bool; 256] = 
[true , true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 true , true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 true , true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 true , true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, true , false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false, 
 false, true , false, false, false, false, false, false];

/// Return `true` if `n` is a perfect square.
///
/// This function works by taking the first byte of `n`, and
/// checking to see if it is a candidate for being a perfect square.
/// If it is not, `false` is returned. If it is, the square root is
/// taken. If the root is an integral, `n` is a perfect square and `true`
/// is returned, otherwise `false` is returned.
///
/// # Examples
///
/// ``` 
/// use reikna::factor::perfect_square;
/// assert_eq!(perfect_square(435), false);
/// assert_eq!(perfect_square(81), true);
/// ```
pub fn perfect_square(n: u64) -> bool {
    if !GOOD_BYTES[(n & 0xff) as usize] {
        return false;
    }

    let root = (n as f64).sqrt() as u64;
    root * root == n
}

/// Return `true` if `n` is a perfect cube.
///
/// This function works by checking if the digital root of `n`
/// is equal to zero, one, eight, or nine. If it is not, `n` cannot
/// be a perfect cube and the function returns `false`. If the
/// digital root is a valid number, then the cube root of `n` is taken.
/// If the root is an integer, then `n` is a perfect cube and `true` is
/// returned, otherwise `false` is returned.
///
/// # Examples
///
/// ```
/// use reikna::factor::perfect_cube;
/// assert_eq!(perfect_cube(216), true);
/// assert_eq!(perfect_cube(9), false);
/// ```
pub fn perfect_cube(n: u64) -> bool {
    if n == 0 {
        return true;
    }

    let dr = n - 9 * ((n - 1) as f64 / 9.0) as u64;

    if dr == 0 && dr != 1 && dr != 8 && dr != 9 {
        return false;
    }

    let root = (n as f64).cbrt();
    if (root - root.round()).abs() > 0.000000001 {
        return false;
    }

    let root_i = root.round() as u64;
    root_i * root_i * root_i == n
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(24, 12), 12);
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(5125215, 890898), 3);
        assert_eq!(gcd(5125215, 890898), 3);
    }

#[test]
    fn t_gcd_all() {
        assert_eq!(gcd_all(&vec![]), 0);
        assert_eq!(gcd_all(&vec![0, 0, 0]), 0);
        assert_eq!(gcd_all(&vec![0, 1, 0, 1]), 1);
        assert_eq!(gcd_all(&vec![0, 2, 6, 8]), 2);
        assert_eq!(gcd_all(&vec![1, 2, 3, 4]), 1);
        assert_eq!(gcd_all(&vec![9, 27, 81]), 9);
        assert_eq!(gcd_all(&vec![2, 4, 6, 8]), 2);
    }

#[test]
    fn t_coprime() {
        assert_eq!(coprime(0, 0), false);
        assert_eq!(coprime(1, 0), true);
        assert_eq!(coprime(1, 10), true);
        assert_eq!(coprime(4, 9), true);
        assert_eq!(coprime(12, 9), false);
    }

#[test]
    fn t_lcm() {
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(0, 15), 0);
        assert_eq!(lcm(5, 2), 10);
        assert_eq!(lcm(13, 5), 65);
        assert_eq!(lcm(1, 35), 35);
    }

#[test]
    fn t_lcm_all() {
        assert_eq!(lcm_all(&vec![]), 1);
        assert_eq!(lcm_all(&vec![0, 0, 0]), 0);
        assert_eq!(lcm_all(&vec![0, 1, 2, 3]), 0);
        assert_eq!(lcm_all(&vec![1, 2, 3, 4]), 12);
        assert_eq!(lcm_all(&vec![2, 2, 2]), 2);
    }

#[test]
    fn t_perfect_square() {
        assert_eq!(perfect_square(0), true);
        assert_eq!(perfect_square(1), true);
        assert_eq!(perfect_square(2), false);
        assert_eq!(perfect_square(7), false);
        assert_eq!(perfect_square(15), false);
        assert_eq!(perfect_square(144), true);
        assert_eq!(perfect_square(145), false);
        assert_eq!(perfect_square(1_073_741_824), true);
        assert_eq!(perfect_square(1_073_741_823), false);
        assert_eq!(perfect_square(4_611_686_018_427_387_904), true);
        assert_eq!(perfect_square(4_611_686_018_427_387_905), false);
    }

#[test]
    fn t_perfect_cube() {
        assert_eq!(perfect_cube(0), true);
        assert_eq!(perfect_cube(1), true);
        assert_eq!(perfect_cube(3), false);
        assert_eq!(perfect_cube(8), true);
        assert_eq!(perfect_cube(27), true);
        assert_eq!(perfect_cube(28), false);
        assert_eq!(perfect_cube(125), true);
        assert_eq!(perfect_cube(126), false);
        assert_eq!(perfect_cube(262_144), true);
        assert_eq!(perfect_cube(262_143), false);
        assert_eq!(perfect_cube(8_589_934_592), true);
        assert_eq!(perfect_cube(8_589_934_593), false);
        assert_eq!(perfect_cube(11_529_2150_460_6846_976), true);
        assert_eq!(perfect_cube(11_529_2150_460_6846_975), false);
    }

}
