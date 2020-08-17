//! Module for working with integer factorization.
//!
//! This module contains functions for factoring integers, 
//! computing the LCM and GCD of integers, and testing if
//! integers are perfect squares and perfect cubes.

use std::cmp::min;
use std::mem;
use super::prime;

/// Find the GCD of `a` and `b` using the Euclidean algorithm.
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
/// GCD is both commutative and associative, and as such the GCD
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
pub fn gcd_all(set: &[u64]) -> u64 {
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
/// using `gcd()`, then applying the fact that 
///
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

/// Return the LCM of the set of integers
///
/// This function works by applying the fact that the
/// LCM is both commutative and associative, and as such the LCM
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
pub fn lcm_all(set: &[u64]) -> u64 {
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

/// Extract a factor of `val` using `entropy` as a seed
/// value.
///
/// This function will extract a non-trivial factor of `val`
/// using Brent's modification of Pollard's Rho Algorithm.
///
/// This is one of the functions used by `quick_factorize()`,
/// it is applied if value being factor is considered to have
/// "large" magnitude.
///
/// This function is not very useful on its own, and should be
/// integrated into a more general factorization function rather than
/// used directly.
pub fn rho(val: u64, entropy: u64) -> u64 {
    if val == 0 {
        return 1;
    }

    let entropy = entropy.wrapping_mul(val);
    let c = entropy & 0xff;
    let u = entropy & 0x7f;

    let mut r: u64 = 1;
    let mut q: u64 = 1;
    let mut y: u64 = entropy & 0xf;

    let mut fac = 1;

    let mut y_old = 0;
    let mut x = 0;

    let f = |x: u64| (x.wrapping_mul(x) + c) % val;

    while fac == 1 {
        x = y;

        for _ in 0..r {
            y = f(y);
        }

        let mut k = 0;
        while k < r && fac == 1 {
            y_old = y;

            for _ in 0..min(u, r - k) {
                y = f(y);

                if x > y {
                    q = q.wrapping_mul(x - y) % val;
                } else {
                    q = q.wrapping_mul(y - x) % val;
                }
            }

            fac = gcd(q, val);
            k += u;
        }
        
        r *= 2;
    }


    while fac == val || fac <= 1 {
        y_old = f(y_old);

        if x > y_old {
            fac = gcd(x - y_old, val);
        } else if x < y_old {
            fac = gcd(y_old - x, val);
        } else {
            // the algorithm has failed for this entropy,
            // return the factor as-is
            return fac;
        }
    }

    fac
}

/// The largest number considered "small" by `quick_factorize_wsp()`.
///
/// Values less than this will be factored with `prime::factorize_wp()`,
/// this is also the value used as the maximum argument to `prime_sieve()`
/// in the `quick_factorize()` helper function.
pub const MAX_SMALL_NUM: u64 = 65_536;

/// Return a `Vec<u64>` of `value`'s prime factorization,
/// using `sprimes` as a list of small primes;
///
/// `sprimes` should be a sorted list of the prime numbers in
/// `[1, MAX_SMALL_NUM]`, or else this function will not
/// behave properly. A suitable list can be generated using
/// `prime::prime_sieve(MAX_SMALL_NUM)`.
///
/// Alternatively, if only a few values are being factored,
/// `quick_factorize()` can be used in lieu of this function and
/// an explicit list of primes.
///
/// This function will factor "small" values, i.e., those less
/// than `MAX_SMALL_NUM`, using `prime::factorize_wp()`, which
/// in turn factors by trial division over a list of primes. This
/// is the fastest way of factoring relatively small values.
///
/// Large values are factored using Brent's modification of
/// Pollard's Rho Algorithm, implemented in the function `rho()`.
///
/// Note this function can take a long time if the value being 
/// factored is a large prime or a value with one very large factor.
/// The correct factorization will be returned for these values,
/// but it is best to filter them out of any data set being factored
/// before hand.
///
/// The factor list this function returns is sorted.
/// 
/// # Examples
///
/// ```
/// use reikna::factor::*;
/// use reikna::prime;
/// let sprimes = prime::prime_sieve(MAX_SMALL_NUM);
/// assert_eq!(quick_factorize_wsp(65_536, &sprimes), vec![2; 16]);
/// assert_eq!(quick_factorize_wsp(9_223_372_036_854_775_807, &sprimes), 
///            vec![7, 7, 73, 127, 337, 92737, 649657]);
/// ```
pub fn quick_factorize_wsp(mut val: u64, 
                           sprimes: &[u64]) -> Vec<u64> {
    if val < MAX_SMALL_NUM {
        return prime::factorize_wp(val, sprimes);
    }

    let mut factors: Vec<u64> = Vec::with_capacity(64);

    while val & 0x01 == 0 {
        val >>= 1;
        factors.push(2);
    }

    let mut e = 2;
    while val > 1 {
        if prime::is_prime(val) {
            factors.push(val);
            break;
        }

        let factor = rho(val, e);

        if factor == val || factor == 1 {
            e += 1;
            continue;
        } else if prime::is_prime(factor) {
            factors.push(factor);
        } else {
            factors.extend_from_slice(
                   &quick_factorize_wsp(factor, sprimes));
        }

        val /= factor;
    }

    factors.sort();
    factors
}

/// Return a `Vec<u64>` of `value`'s prime factorization.
///
/// This is a helper function that calls `quick_factorize_wsp()`,
/// using `value` and a generated list of primes for the arguments.
/// See `quick_factorize_wsp()` for more information.
///
/// Because this function generates a list of primes each time it
/// is called, it is preferable to use `quick_factorize_wsp()` 
/// directly with an explicit list of primes if numerous factorizations
/// are being computed.
///
/// # Panics
///
/// Panics if `prime_sieve()`, see the documentation for
/// this function for more information.
///
/// # Examples
///
/// ```
/// use reikna::factor::quick_factorize;
/// assert_eq!(quick_factorize(65_536), vec![2; 16]);
/// assert_eq!(quick_factorize(9_223_372_036_854_775_807), 
///            vec![7, 7, 73, 127, 337, 92737, 649657]);
/// ```
pub fn quick_factorize(value: u64) -> Vec<u64> {
    quick_factorize_wsp(value, &prime::prime_sieve(MAX_SMALL_NUM))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::prime::is_prime;

#[test]
    fn t_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(10, 0), 10);
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

#[test]
    fn t_quick_factorize() {
        assert_eq!(quick_factorize(0), Vec::new());
        assert_eq!(quick_factorize(1), Vec::new());

        let test_vals = vec![125, 97, 168, 256, 1789, 34567,
                             97020,
                             103685,
                             653123,
                             4593140,
                             13461780,
                             982357223,
                             72314573234,
                             517825353462,
                             8735263124568,
                             128735128735049,
                             1302131490435579,
                             90977992317385808,
                             (2f64.powf(63.0)) as u64 - 1];

        for val in test_vals.iter() {
            let factors = quick_factorize(*val);

            let prod: u64 = factors.iter().fold(1, |acc, x| acc * *x);
            assert_eq!(*val, prod);

            for fac in factors.iter() {
                assert_eq!(is_prime(*fac), true);
            }
        }
    }

#[test]
#[ignore]
    fn t_quick_factorize_long() {
        let test_vals = vec![(2f64.powf(61.0)) as u64 - 1,
                             (2f64.powf(31.0)) as u64 - 1,
                             (2f64.powf(19.0)) as u64 - 1,
                             (2f64.powf(17.0)) as u64 - 1,];
        
        for val in test_vals.iter() {
            let factors = quick_factorize(*val);

            let prod = factors.iter().fold(1, |acc, x| acc * *x);
            assert_eq!(*val, prod);

            for fac in factors.iter() {
                assert_eq!(super::super::prime::is_prime(*fac), true);
            }
        }
    }

}
