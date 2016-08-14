//! Module for working with aliquot and divisor sums.
//!
//! This module contains functions for calculating the 
//! aliquot and divisor sums of numbers, along with functions
//! for testing for perfect numbers and similar concepts.

/// Return the aliquot sum of a positive integer `n`, 
/// that is, the sum of all of `n`'s proper divisors.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::aliquot_sum;
/// assert_eq!(aliquot_sum(28), 28);
/// assert_eq!(aliquot_sum(29), 1);
/// ```
pub fn aliquot_sum(n: u64) -> u64 {
    assert!(n != 0, "aliquot sum is only defined for positive integers!");
    if n == 1 { return 0; }

    let mut sum = 1;
    for i in 2..((n as f64).sqrt() as u64 + 1) {
        if n % i == 0 {
            sum += i;
            if n / i != i { sum += n / i; }
        }
    }

    sum
}

/// Return the divisor sum of a positive integer `n`,
/// that is, the sum of all of `n`'s divisors.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::divisor_sum;
/// assert_eq!(divisor_sum(28), 56);
/// assert_eq!(divisor_sum(29), 30);
/// ```
pub fn divisor_sum(n: u64) -> u64 {
    aliquot_sum(n) + n
}

/// Return `true` if `n` is an abundant number,
/// that is, a number whose aliquot sum is greater
/// than itself.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::abundant_number;
/// assert_eq!(abundant_number(24), true);
/// assert_eq!(abundant_number(28), false);
/// ```
pub fn abundant_number(n: u64) -> bool {
    aliquot_sum(n) > n
}

/// Return `true` if `n` is a perfect number,
/// that is, a number whose aliquot sum is equal
/// to itself.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::perfect_number;
/// assert_eq!(perfect_number(6), true);
/// assert_eq!(perfect_number(8), false);
/// ```
pub fn perfect_number(n: u64) -> bool {
    aliquot_sum(n) == n
} 

/// Return `true` if `n` is a deficient number,
/// that is, a number whose aliquot sum is less
/// than itself.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::deficient_number;
/// assert_eq!(deficient_number(7), true);
/// assert_eq!(deficient_number(28), false);
/// ```
pub fn deficient_number(n: u64) -> bool {
    aliquot_sum(n) < n
} 

/// Return `true` if `n` is a superperfect number,
/// that is, a number which satisfies
///
/// ```text
/// σ(σ(n)) = 2n
/// ```
///
/// Where 'σ(x)' is the divisor sum function.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::superperfect_number;
/// assert_eq!(superperfect_number(4), true);
/// assert_eq!(superperfect_number(5), false);
/// ```
pub fn superperfect_number(n: u64) -> bool {
    divisor_sum(divisor_sum(n)) == 2 * n
}

/// Return `true` if `n` is a quasiperfect number,
/// that is, a number whose aliquot sum is exactly
/// one greater than itself.
///
/// No quasiperfect numbers are known.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::quasiperfect_number;
/// assert_eq!(quasiperfect_number(28), false);
/// assert_eq!(quasiperfect_number(145_688), false);
/// ```
pub fn quasiperfect_number(n: u64) -> bool {
    aliquot_sum(n) == n + 1
}

/// Return `true` if `n` is a member of an amicable pair,
/// that is, a pair of numbers whose aliquot sums equal
/// each other.
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::amicable_number;
/// assert_eq!(amicable_number(2620), true);
/// assert_eq!(amicable_number(2621), false);
/// ```
pub fn amicable_number(n: u64) -> bool {
    aliquot_sum(aliquot_sum(n)) == n
}

/// Return `true` if `n` is a sociable number,
/// that is, a number whose aliquot sums form a
/// cyclic pattern, e.g.
///
/// ```text
/// 14288 -> 15472 -> 14536 -> 14264 -> 12496 -> 14288
/// ```
///
/// # Panics
/// 
/// Panics if `n` is zero.
///
/// # Examples
///
/// ```
/// use reikna::aliquot::sociable_number;
/// assert_eq!(sociable_number(14288), true);
/// assert_eq!(sociable_number(14289), false);
/// ```
pub fn sociable_number(n: u64) -> bool {
    let mut x = aliquot_sum(n);
    loop {
        if x == 1 { return false; }
        if x == n { return true;  }

        x = aliquot_sum(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
#[test]
    fn t_aliquot() {
        assert_eq!(aliquot_sum(1), 0);
        assert_eq!(aliquot_sum(2), 1);
        assert_eq!(aliquot_sum(3), 1);
        assert_eq!(aliquot_sum(4), 3);
        assert_eq!(aliquot_sum(5), 1);
        assert_eq!(aliquot_sum(6), 6);
        assert_eq!(aliquot_sum(7), 1);
        assert_eq!(aliquot_sum(8), 7);
        assert_eq!(aliquot_sum(9), 4);

        assert_eq!(divisor_sum(1), 1);
        assert_eq!(divisor_sum(2), 3);
        assert_eq!(divisor_sum(3), 4);
        assert_eq!(divisor_sum(4), 7);
        assert_eq!(divisor_sum(5), 6);
        assert_eq!(divisor_sum(6), 12);
        assert_eq!(divisor_sum(7), 8);
        assert_eq!(divisor_sum(8), 15);
        assert_eq!(divisor_sum(9), 13);

        assert_eq!(aliquot_sum(97), 1);
        assert_eq!(aliquot_sum(100), 117);
    }

#[test]
#[should_panic]
    fn t_aliquot_p() {
        aliquot_sum(0);
    }
#[test]
    fn t_numerology() {
        assert!(abundant_number(12));
        assert!(abundant_number(20));
        assert!(abundant_number(42));
        assert!(abundant_number(88));
        assert!(abundant_number(96));
        assert!(abundant_number(945));
        assert!(!abundant_number(1));
        assert!(!abundant_number(946));

        assert!(perfect_number(6));
        assert!(perfect_number(28));
        assert!(perfect_number(496));
        assert!(perfect_number(8128));
        assert!(perfect_number(33550336));
        assert!(perfect_number(8589869056));
        assert!(!perfect_number(1));
        assert!(!perfect_number(8589869055));

        assert!(deficient_number(1));
        assert!(deficient_number(3));
        assert!(deficient_number(23));
        assert!(deficient_number(32));
        assert!(deficient_number(49));
        assert!(deficient_number(50));
        assert!(!deficient_number(88));
        
        assert!(superperfect_number(2));
        assert!(superperfect_number(4));
        assert!(superperfect_number(16));
        assert!(superperfect_number(64));
        assert!(superperfect_number(4096));
        assert!(superperfect_number(1073741824));
        assert!(!superperfect_number(1));
        assert!(!superperfect_number(1073741825));

        assert!(!quasiperfect_number(1));
        assert!(!quasiperfect_number(6));
        assert!(!quasiperfect_number(28));
        assert!(!quasiperfect_number(128));
        assert!(!quasiperfect_number(75312));
        assert!(!quasiperfect_number(891770));
    }

#[test]
    fn t_sociable() {
        assert!(amicable_number(220));
        assert!(amicable_number(284));
        assert!(amicable_number(5020));
        assert!(amicable_number(10744));
        assert!(!amicable_number(10745));

        assert!(sociable_number(8128));
        assert!(sociable_number(220));
        assert!(sociable_number(1264460));
        assert!(sociable_number(14316));
        assert!(!sociable_number(14313));
    }
}
