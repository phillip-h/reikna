//! Module for working with prime numbers.
//!
//! This module has functions for generating prime numbers
//! using a variety of different sieves, testing if numbers
//! are prime or composite, and preforming simple factorizations.

/// Return a `Vec<u64>` of the primes in [1, `max_u64`] using the 
/// Sieve of Atkin.
///
/// This function is best suited for sieving with relatively
/// small maximums, in which case it is very fast. Large maximums
/// will start to incur negative performance impacts from
/// memory allocation, which increases linearly with `max_u64`.
/// For large maximums, `segmented_eratosthenes()` is a better choice.
/// `prime_sieve()` can be used to choose between the two automatically.
///
/// # Panics
///
/// Panics if `max_u64` cannot be cast into a `usize`.
///
/// Can panic if `max_u64` is so large that not enough
/// memory can be allocated for the sieve.
///
/// # Examples
///
/// ```
/// use reikna::prime::atkin;
/// assert_eq!(atkin(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// ```
pub fn atkin(max_u64: u64) -> Vec<u64> {
    assert!(max_u64 < ::std::usize::MAX as u64, 
            "sieve max {} is larger than machine word size!", max_u64);
    let max = max_u64 as usize;

    let mut primes: Vec<u64> = Vec::new();

    match max {
        0 | 1 => (),
        2     => primes.extend_from_slice(&[2]),
        3 | 4 => primes.extend_from_slice(&[2, 3]),
        _     => primes.extend_from_slice(&[2, 3, 5]),
    }

    if max < 6 {
        return primes
    }

    let mut sieve = Bitset::new(max);
    let limit = (max as f64).sqrt() as usize + 1;

    let mut index: usize;
    for x in 1..(limit + 1) {
        for y in 1..(limit + 1) {

            index = 4 * x * x + y * y;
            if index <= max {
                match index % 60 {
                    1 | 13 | 17 | 29 | 37 | 41 |49 | 53  => sieve.flip(index),
                    _ => (),
                }
            }

            index = 3 * x * x + y * y;
            if index <= max {
                match index % 60 {
                    7 | 19 | 31 | 43  => sieve.flip(index),
                    _ => (),
                }
            }
          
            if x <= y {
                continue;
            }

            index = 3 * x * x - y * y;
            if index <= max {
                match index % 60 {
                    11 | 23 | 47 | 59 => sieve.flip(index),
                    _ => (),
                }
            }
        }
    }

    let mut val;
    for i in 7..(limit + 1) {
        if sieve.read(i) {
            val = i * i;
            let mut k = val;
            while k <= max {
                sieve.set(k, false);
                k += val;
            }
        }
    }

    primes.extend(sieve.collect_true_indices());
    primes
}

/// Return a `Vec<u64>` of the primes in [1, `max_u64`] using the 
/// Sieve of Eratosthenes.
///
/// This function is probably not very useful to most users, 
/// and is used primarily in validating the other prime sieves.
///
/// # Panics
///
/// Panics if `max_u64` cannot be cast into a `usize`.
///
/// Can panic if `max_u64` is so large that not enough
/// memory can be allocated for the sieve.
///
/// # Examples
///
/// ```
/// use reikna::prime::eratosthenes;
/// assert_eq!(eratosthenes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// ```
pub fn eratosthenes(max_u64: u64) -> Vec<u64> {
    assert!(max_u64 < ::std::usize::MAX as u64, 
            "sieve max {} is larger than machine word size!", max_u64);
    let max = max_u64 as usize;

    if max == 0 {
        return Vec::new();
    }

    let mut sieve = Bitset::new(max + 1);
    sieve.one();
    let mut primes: Vec<u64> = Vec::new();

    let mut not_prime;
    for pos in 2..(max + 1) {
        if sieve.read(pos) {
            primes.push(pos as u64);

            not_prime = pos * 2; 
            while not_prime < max + 1 {
                sieve.set(not_prime, false);
                not_prime += pos;
            }
        }
    }

    primes
}

/// Size of the segmented sieve segments in `segmented_eratosthenes()`
///
/// Also used to determine when `prime_sieve()` should
/// switch to using the segmented sieve from the Sieve of Atkin.
pub const S_SIEVE_SIZE: u64 = 65_536;

/// Macro representing the body of a segmented Sieve
/// of Eratosthenes.
///
/// This macro is useful for preforming operations on
/// ranges of prime numbers that would be too large to
/// store in memory (such as the `prime_sieve()` family of
/// functions.
///
/// The syntax for this macro is
///
/// ```text
/// segmented_sieve(max (identifier), candidate (identifier),
///                 callback (expression));
/// ```
///
/// Where `max` is an identifier where the max value for the sieve
/// can be found, `candidate` is an identifier that exposes each
/// prime found by the sieve, and `expression` is a callback expression
/// that is ran every time a prime is found.
///
/// Note -- this routine will never include the number `2`! It is a
/// special case and must be dealt with separately.
///
/// Note -- this macro assumes that several members of `prime` are
/// also in scope! The following `use` statement should work:
///
/// ```
/// use reikna::prime::{Bitset, S_SIEVE_SIZE, prime_sieve};
/// ```
///
/// # Panics
/// 
/// Panics if `prime_sieve()` panics. See the documentation of 
/// `prime_sieve()` for more information.
///
/// # Examples
///
/// Find the sum of the primes in `[0..1,000,000]`.
///
/// ```
/// #[macro_use] extern crate reikna;
/// use reikna::prime::{Bitset, S_SIEVE_SIZE, prime_sieve};
/// fn main() {
///     let max = 1_000_000;
///     let mut sum = 2;
///
///     segmented_sieve!(max, candidate, sum += candidate);
///
///     println!("Sum of primes in [0, {}] -- {}", max, sum);
/// }
/// ```
#[macro_export]
macro_rules! segmented_sieve {
    ($max:ident, $candidate:ident, $callback:expr) => {
        // generate small primes used for sieving
        let limit = ($max as f64).sqrt() as u64 + 1;
        let small_primes = prime_sieve(limit);

        // create the sieve 
        let mut sieve = Bitset::new(S_SIEVE_SIZE as usize);

        // create a vec of active sieving primes and their offsets
        let mut sieve_primes: Vec<u64> = Vec::new();
        let mut offsets: Vec<u64> = Vec::new();

        // cross-loop variables
        let mut small = 2;
        let mut $candidate = 3; 

        // calculate sieve end condition
        let end = ($max as f64 / S_SIEVE_SIZE as f64).ceil() as u64;
        for pos in (0..end).map(|pos| pos * S_SIEVE_SIZE) {
            sieve.one();

            // calculate the upper boundary
            let mut pos_h = pos + S_SIEVE_SIZE as u64 - 1;
            if pos_h > $max { pos_h = $max;}

            // add any new small primes to the sieve vec
            while small * small <= pos_h {
                if small_primes.iter().any(|x| *x == small) {
                    sieve_primes.push(small);
                    offsets.push(small * small - pos);
                }
                small += 1;
            }

            // preform the sieve
            for i in 1..sieve_primes.len() {
                let mut j = offsets[i];
                let k = sieve_primes[i] * 2;

                while j < S_SIEVE_SIZE as u64 {
                    sieve.set(j as usize, false);
                    j += k;
                }
                offsets[i] = j - S_SIEVE_SIZE as u64;
            }

            // collect primes, call the callback expression
            while $candidate <= pos_h {
                if sieve.read(($candidate - pos) as usize) {
                    $callback
                }
                $candidate += 2;
            }

        }
    }
}

/// Return a `Vec<u64>` of the primes in [1, max] using a segmented
/// Sieve of Eratosthenes.
///
/// This function is best suited for sieving with a large max,
/// otherwise `atkin()` is preferable. `prime_sieve()` can be
/// used to chose between the two automatically.
///
/// The size of the segments is determined by `S_SIEVE_SIZE`.
///
/// # Panics
///
/// Panics if `max` cannot be cast into a `usize`.
///
/// # Examples
///
/// ```
/// use reikna::prime::segmented_eratosthenes;
/// assert_eq!(segmented_eratosthenes(10), vec![2, 3, 5, 7]);
/// ```
pub fn segmented_eratosthenes(max: u64) -> Vec<u64> {
    if max < 2 {
        return Vec::new();
    }

    let mut primes: Vec<u64> = vec![2];
    segmented_sieve!(max, candidate, {primes.push(candidate);});

    primes
}

/// Return the Nth prime number, starting with `P0 = 2`.
///
/// This function works by sieving the range `[0..u64::MAX]`,
/// and returning after the Nth prime is found.
///
/// If the Nth prime is not in this range, this function will
/// panic.
///
/// # Panics
///
/// Panics if the Nth prime is greater than `u64::MAX`.
///
/// # Examples
///
/// ```
/// use reikna::prime::nth_prime;
/// assert_eq!(nth_prime(3), 7);
/// assert_eq!(nth_prime(24), 97);
/// ```
pub fn nth_prime(n: u64) -> u64 {
    match n {
        0 => return 2,
        1 => return 3,
        2 => return 5,
        3 => return 7,
        _ => (),
    }

    let nf = (n + 1) as f64;
    let max = ((nf * nf.ln()) + (nf * nf.ln().ln())).ceil() as u64;
    let mut count = 1;
    segmented_sieve!(max, candidate, { 
                                        if count == n { return candidate; }
                                        count += 1; 
                                     });

    panic!("Nth prime of N = {} is larger than u64::MAX!", n);
}

/// Idiomatic prime sieve, returns a `Vec<u64>` of primes in [1, max].
///
/// If you want to generate primes, this is probably the function
/// you want.
///
/// This function will use `atkin()` to generate primes if
/// `max` is less than `S_SIEVE_SIZE`, otherwise it will use 
/// `segmented_eratosthenes()`.
///
/// See `atkin()` and `segmented_eratosthenes()` for more
/// information.
///
/// # Panics
/// 
/// Panics if `max` is too large to cast into a `usize`.
///
/// # Examples
///
/// ```
/// use reikna::prime::prime_sieve;
/// assert_eq!(prime_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// ```
pub fn prime_sieve(max: u64) -> Vec<u64> { 
    if max < S_SIEVE_SIZE { // 2^16
        return atkin(max);
    }

    segmented_eratosthenes(max)
}

/// Return `true` if `value` is prime, and false if it is composite.
///
/// This function works by checking if `value` is a small prime,
/// the checking if it is divisible by two or three.
///
/// Next, a loop is preformed to check if `value` can be represented
/// in the form `6x +/- 1`, if it can, `value` is composite. Otherwise
/// it is prime.
///
/// # Examples
///
/// ```
/// use reikna::prime::is_prime;
/// assert_eq!(is_prime(64), false);
/// assert_eq!(is_prime(97), true);
/// assert_eq!(is_prime(113), true);
/// assert_eq!(is_prime(128), false);
/// ```
pub fn is_prime(value: u64) -> bool {
    if value < 2 {
        return false;
    } 

    if value < 4 {
        return true;
    }

    if value % 2 == 0 || value % 3 == 0 {
        return false;
    }

    let max_fac = (value as f64).sqrt() as u64 + 1;
    let mut test_fac = 5;
    while test_fac <= max_fac {
        if value % test_fac == 0 || value % (test_fac + 2) == 0 {
            return false;
        }
        test_fac += 6;
    }

    true
}

/// Return a `Vec<u64>` of the value's factorization,
/// using the provided list of primes.
///
/// This function preforms factorization by test dividing
/// `value` for all provided primes in [1, value].
///
/// This function is best suited for computing multiple
/// factorizations, so the primes list can be cached between
/// calls, or for computing factorizations with a custom list
/// of "prime factors". For other uses, such as factoring a
/// single value, `factorize()` may be a wiser choice.
///
/// # Examples
///
/// Please note this function assumes that `primes` is sorted.
///
/// ```
/// use reikna::prime::factorize;
/// assert_eq!(factorize(200), vec![2, 2, 2, 5, 5]); 
/// ```
pub fn factorize_wp(mut value: u64, primes: &[u64]) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    if value <= 1 {
        return factors;
    }

    for prime in primes {
        if *prime > value {
            break;
        }

        while value % *prime == 0 {
            factors.push(*prime);
            value /= *prime;
        }
    }

    factors
}

/// Return a `Vec<u64>` of the value's factorization
///
/// This is a helper function that generates a `Vec` of
/// primes internally, rather than requiring one to be
/// explicitly passed in.
///
/// # Panics
///
/// Panics if `value` causes a panic when provided to
/// `prime_sieve()`.
///
/// # Examples
///
/// ```
/// use reikna::prime::factorize;
/// assert_eq!(factorize(100), vec![2, 2, 5, 5]); 
/// ```
pub fn factorize(value: u64) -> Vec<u64> {
    factorize_wp(value, &prime_sieve(value))
}

/// Return the smallest prime number greater than `n`.
///
/// This function works by adding `2` to `n`, then testing
/// `n`'s primality with `is_prime()`. If `n` is initially
/// even, `1` is added to it. If the resulting value is not
/// prime, the normal sequence continues
///
/// # Examples
///
/// ```
/// use reikna::prime::next_prime;
/// assert_eq!(next_prime(5), 7);
/// assert_eq!(next_prime(95), 97);
/// ```
pub fn next_prime(mut n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 2;
    }

    if n & 0x01 == 0 {
        n |= 0x01;
        if is_prime(n) { return n; }
    }
    
    loop {
        n += 2;
        if is_prime(n) { return n; }
    }
}

/// Simple bit set implementation for prime sieves
///
/// Please note that this struct is not intended for
/// general use, and is only publicly exposed so
/// the `segmented_sieve!` macro can work.
pub struct Bitset {
    data: Vec<u8>,
    size: usize
}

impl Bitset {
#![allow(missing_docs)]

    pub fn new(size: usize) -> Bitset {
        let size_bytes = size + (size % 8);
        Bitset { data: vec![0; size_bytes], size: size }
    }

    pub fn one(&mut self) {
        for byte in &mut self.data {
            *byte = 0xff;
        }
    }

    pub fn read(&self, pos: usize) -> bool {
        self.data[pos / 8] & 0x01 << (pos % 8) != 0x00
    }

    fn flip(&mut self, pos: usize) {
        self.data[pos / 8] ^= 0x01 << (pos % 8);
    }

    pub fn set(&mut self, pos: usize, value: bool) {
        if self.read(pos) != value {
            self.flip(pos);
        }
    }

    fn collect_true_indices(&self) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new(); 
        for i in 0..self.size + 1 {
            if self.read(i) {
                res.push(i as u64);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_prime_sieves() {
        let primes = prime_sieve(0);
        assert_eq!(primes.len(), 0);

        let primes = prime_sieve(2);
        assert_eq!(primes.len(), 1);

        let primes = prime_sieve(100);
        assert_eq!(primes.len(), 25);

        assert_eq!(eratosthenes(0), atkin(0));
        assert_eq!(eratosthenes(1), atkin(1));
        assert_eq!(eratosthenes(2), atkin(2));
        assert_eq!(eratosthenes(3), atkin(3));
        assert_eq!(eratosthenes(4), atkin(4));
        assert_eq!(eratosthenes(5), atkin(5));
        assert_eq!(eratosthenes(6), atkin(6));
        assert_eq!(eratosthenes(10), atkin(10));
        assert_eq!(eratosthenes(1000), atkin(1000));

        assert_eq!(segmented_eratosthenes(0), atkin(0));
        assert_eq!(segmented_eratosthenes(1), atkin(1));
        assert_eq!(segmented_eratosthenes(2), atkin(2));
        assert_eq!(segmented_eratosthenes(3), atkin(3));
        assert_eq!(segmented_eratosthenes(4), atkin(4));
        assert_eq!(segmented_eratosthenes(5), atkin(5));
        assert_eq!(segmented_eratosthenes(6), atkin(6));
        assert_eq!(segmented_eratosthenes(10), atkin(10));
        assert_eq!(segmented_eratosthenes(100000), atkin(100000));
    }

#[test]
    fn t_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(1232), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(97), true);
        assert_eq!(is_prime(9973), true);
    }

#[test]
    fn t_factorize() {
        let vec: Vec<u64> = Vec::new();

        assert_eq!(factorize(0), vec);
        assert_eq!(factorize(1), vec);

        let vec: Vec<u64> = vec![7];
        assert_eq!(factorize(7), vec);

        let vec: Vec<u64> = vec![2, 2, 3];
        assert_eq!(factorize(12), vec);

        let vec: Vec<u64> = vec![2, 2, 5, 5];
        assert_eq!(factorize(100), vec);
    }

#[test]
    fn t_next_prime() {
        assert_eq!(next_prime(0), 2);
        assert_eq!(next_prime(1), 2);
        assert_eq!(next_prime(2), 3);
        assert_eq!(next_prime(3), 5);
        assert_eq!(next_prime(4), 5);
        assert_eq!(next_prime(15), 17);
        assert_eq!(next_prime(39), 41);
        assert_eq!(next_prime(98), 101);
        assert_eq!(next_prime(1_299_821), 1_299_827);
    }

#[test]
    fn t_nth_prime() {
        assert_eq!(nth_prime(0), 2);
        assert_eq!(nth_prime(1), 3);
        assert_eq!(nth_prime(2), 5);
        assert_eq!(nth_prime(3), 7);
        assert_eq!(nth_prime(4), 11);
        assert_eq!(nth_prime(5), 13);
        assert_eq!(nth_prime(25), 101);
        assert_eq!(nth_prime(1_000_000), 15_485_867);
    }

#[test]
#[ignore]
    fn t_nth_prime_long() {
        assert_eq!(nth_prime(1_000_000_000), 22_801_763_513);
    }
}

