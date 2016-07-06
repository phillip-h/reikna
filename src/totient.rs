//! Module for working with Euler's totient function.
//!
//! This module has functions for computing the value of
//! the totient function, both for single and multiple
//! values.

use super::prime;
use super::factor;

/// Constant string of the uppercase Phi symbol,
/// often used to represent the totient function.
pub const PHI_SYMBOL: &'static str = "Φ";

// helper function to do the totient calculation
fn totient_calc(n: u64, factors: Vec<u64>) -> u64 {
    let mut totient = n as f64;
    for factor in factors {
        totient *= 1.0 - (1.0 / factor as f64);
    }

    totient as u64
}

/// Calculate the value of Euler's totient function for `n`.
///
/// This function uses Euler's product formula to compute the
/// totient function. This involves factoring `n`, meaning that
/// the computation of the totient function can take a long time
/// if `n` is a large prime or has a large prime factor.
///
/// # Examples
///
/// ```
/// use reikna::totient::totient;
/// assert_eq!(totient(17), 16);
/// assert_eq!(totient(36), 12);
/// ```
pub fn totient(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    if prime::is_prime(n) {
        return n - 1;
    }

    let mut factors = factor::quick_factorize(n);
    factors.dedup();

    totient_calc(n, factors)
}

/// Calculate the value of Euler's totient function for each
/// value in `data`, and return a new `Vec<u64>` of the results.
///
/// The resulting vector has the same size as the input vector.
///
/// The computation of the totient function itself is done
/// through `totient()`, see the documentation for that function
/// for more information.
///
/// # Examples
///
/// ```
/// use reikna::totient::totient_all;
/// assert_eq!(totient_all(vec![1, 2, 3]), vec![1, 1, 2]);
/// assert_eq!(totient_all(vec![81]), vec![54]);
/// ```
pub fn totient_all(data: Vec<u64>) -> Vec<u64> {
    if data.len() == 0 {
        return Vec::new();
    }

    let sprimes = prime::prime_sieve(factor::MAX_SMALL_NUM);

    let mut totients: Vec<u64> = Vec::new();

    for n in data {
        if n <= 2 {
            totients.push(1);
        } else if prime::is_prime(n) {
            totients.push(n - 1);
        } else {
            let mut factors = factor::quick_factorize_wsp(n, &sprimes);
            factors.dedup();
            totients.push(totient_calc(n, factors));
        }
    }

    totients
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_totient() {
        assert_eq!(totient(0), 1);
        assert_eq!(totient(1), 1);
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(4), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(6), 2);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(8), 4);
        assert_eq!(totient(9), 6);
        assert_eq!(totient(99), 60);
        assert_eq!(totient(10_809_483_705_896), 5_404_726_850_224);
    }

#[test]
    fn t_totient_all() {
        assert_eq!(totient_all(vec![]), vec![]);
        assert_eq!(totient_all(vec![45])[0], totient(45));
        assert_eq!(totient_all(vec![134]).len(), 1);
        assert_eq!(totient_all(vec![1, 2, 3, 4]).len(), 4);
        assert_eq!(totient_all(vec![10, 20, 30, 40]), vec![4, 8, 8, 16]);
    }

}
