//! Module for working with the prime-counting function.
//!
//! This module has functions for computing the value of
//! the prime-counting function for both single and multiple
//! values.

use super::prime::prime_sieve;

/// Constant string of the uppercase Pi symbol,
/// often used to represent the prime-counting function.
pub const PI_SYMBOL: &'static str = "π";

/// Return the number of prime numbers less than or equal to `x`.
///
/// This function works by using a lookup table if `x` is very small
/// (less than 100), and otherwise using a recursive version of
/// Lehmer's Formula.
///
/// Note that this function can take a very long time to produce a result
/// if `x` is very large.
///
/// If multiple values of the prime-counting function are being calculated,
/// `prime_count_all()` is a better choice because it preserves its caches
/// between calculations. See the documentation for `prime_count_all` for
/// more information.
///
/// # Panics
/// 
/// Panics if `prime_sieve()` panics, see the documentation of
/// `prime_sieve()` for more information.
///
/// # Examples
///
/// ```
/// use reikna::prime_count::prime_count;
/// assert_eq!(prime_count(1_000), 168);
/// assert_eq!(prime_count(10_000), 1_229);
/// ```
pub fn prime_count(x: u64) -> u64 {
    match x {
        0 | 1 => 0,
        2     => 1,
        3 | 4 => 2,
        5     => 3,
        _     => lehmer(x, &prime_sieve((x as f64).sqrt() as u64 + 1),
                        &mut vec![vec![0u64; CACHE_SIZE]; CACHE_SIZE]),
    }
}

/// Calculate the value of the prime-counting function for each
/// value in `data`, and return a new `Vec<u64>` of the results.
///
/// The resulting vector has the same size as the input vector.
/// 
/// This function works in fundamentally the same way as `prime_count()`,
/// with the modification that caches are preserved between calculations.
/// This allows for much faster computation of multiple values.
///
/// # Panics
/// 
/// Panics if `prime_sieve()` panics, see the documentation of
/// `prime_sieve()` for more information.
///
/// # Examples
///
/// ```
/// use reikna::prime_count::prime_count_all;
/// assert_eq!(prime_count_all(&vec![1, 2, 3]), vec![0, 1, 2]);
/// assert_eq!(prime_count_all(&vec![100]), vec![25]);
/// ```
pub fn prime_count_all(data: &Vec<u64>) -> Vec<u64> {
    if data.len() == 0 {
        return Vec::new();
    }

    let mut counts: Vec<u64> = Vec::new();
    let mut phi_cache = vec![vec![0u64; CACHE_SIZE]; CACHE_SIZE];

    let mut largest_index = 0;
    let mut largest_val = 0;
    for i in 0..data.len() {
        if data[i] > largest_val {
            largest_index = i;
            largest_val = data[i];
        }
    }

    let primes = prime_sieve((data[largest_index] as f64).sqrt() as u64 + 1);
    let max_val = lehmer(largest_val, &primes, &mut phi_cache);

    for i in 0..data.len() {
        if i == largest_index {
            counts.push(max_val);
            continue;
        }

        match data[i] < 6 {
            true  => counts.push(prime_count(data[i])),
            false => counts.push(lehmer(data[i], &primes, &mut phi_cache)),
        }
    }

    counts
}

const CACHE_SIZE: usize = 1024;
type CacheT = Vec<Vec<u64>>;

const SMALL_PI: [u64; 100] = 
[0 , 0 , 1 , 2 , 2 , 3 , 3 , 4 , 4 , 4 ,
 4 , 5 , 5 , 6 , 6 , 6 , 6 , 7 , 7 , 8 , 
 8 , 8 , 8 , 9 , 9 , 9 , 9 , 9 , 9 , 10,
 10, 11, 11, 11, 11, 11, 11, 12, 12, 12,
 12, 13, 13, 14, 14, 14, 14, 15, 15, 15,
 15, 15, 15, 16, 16, 16, 16, 16, 16, 17,
 17, 18, 18, 18, 18, 18, 18, 19, 19, 19,
 19, 20, 20, 21, 21, 21, 21, 21, 21, 22,
 22, 22, 22, 23, 23, 23, 23, 23, 23, 24,
 24, 24, 24, 24, 24, 24, 24, 25, 25, 25];

fn lehmer(x: u64, primes: &Vec<u64>, phi_cache: &mut CacheT) -> u64 {
    if x < 100 {
        return SMALL_PI[x as usize];
    }
    
    if x < primes[primes.len() - 1] {
        return num_below(x, primes);
    }


    let a = lehmer((x as f64).powf(0.25).round() as u64, 
                     primes, phi_cache) + 1;
    let b = lehmer((x as f64).sqrt().round() as u64, 
                         primes, phi_cache) + 1;
    let c = lehmer((x as f64).cbrt().round() as u64, 
                     primes, phi_cache);

    let mut pi = phi(x, a - 1, primes, phi_cache) + 
                 ((b + a - 4) * (b - a + 1)) / 2;

    for i in a..b {
        let x_tmp = x / primes[i as usize - 1];
        pi -= lehmer(x_tmp, primes, phi_cache);

        if i > c {
            continue;
        }

        let bi = lehmer((x_tmp as f64).sqrt() as u64, primes, phi_cache) + 1;
        for j in i..bi {
            pi += j - 1;
            pi -= lehmer(x_tmp / primes[j as usize - 1], primes, phi_cache);
        }
    }

    pi
}

fn phi(m: u64, n: u64, primes: &Vec<u64>, cache: &mut CacheT) -> u64 {
    if n == 0 || m == 0 {
        return m;
    }

    if n == 1 {
        return (m + 1) / 2;
    }

    if m <= primes[n as usize - 1] {
        return 1;
    }

    if m < CACHE_SIZE as u64 && n < CACHE_SIZE as u64 {
        if cache[m as usize][n as usize] == 0 {
            let val = phi(m, n - 1, primes, cache) - 
                      phi(m / primes[n as usize - 1], n - 1, primes, cache);
            cache[m as usize][n as usize] = val;
        }

        return cache[m as usize][n as usize];
    }

    phi(m, n - 1, primes, cache) - 
    phi(m / primes[n as usize - 1], n - 1, primes, cache)
}

fn num_below(x: u64, vec: &Vec<u64>) -> u64 {
    for i in 0..vec.len() {
        if vec[i] > x {
            return i as u64;
        }
    }

    vec.len() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

#[test]
    fn t_prime_count() {
        assert_eq!(prime_count(0), 0);
        assert_eq!(prime_count(1), 0);
        assert_eq!(prime_count(2), 1);
        assert_eq!(prime_count(5), 3);
        assert_eq!(prime_count(10), 4);
        assert_eq!(prime_count(100), 25);
        assert_eq!(prime_count(1_000_000), 78_498);
        assert_eq!(prime_count(10_000_000), 664_579);
    }

#[test]
    fn t_prime_count_all() {
        assert_eq!(prime_count_all(&vec![0; 0]), vec![0; 0]);
        assert_eq!(prime_count_all(&vec![1, 2, 3, 4]), vec![0, 1, 2, 2]);
        assert_eq!(prime_count_all(&vec![10, 100, 1_000]), vec![4, 25, 168]);
        assert_eq!(prime_count_all(&vec![1, 2, 3, 4, 5, 6]).len(), 6);
    }

}
