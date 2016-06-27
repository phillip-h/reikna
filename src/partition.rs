//! Module for working with the number theory partition function.
//!
//! This module has functions for calculating the partition function
//! in a fast and safe manner.

use super::figurate::general_pentagonal_number as gpn;

/// Max partition size the functions will calculate
///
/// This limit is put in place to prevent stack and
/// integer overflows.
pub const MAX_PART: i64 = 406;

/// Calculate the partition function of `n` using `cache` to cache 
/// previously calculated values.
///
/// This function will recursively calculate the partition function
/// using Euler's generating function and the Pentagonal Number Theorem.
/// This function uses a `Vec<u64>` to cache previously calculated values,
/// which is necessary to allow the function to run in a reasonable time.
/// 
/// The cache is also useful when calculating multiple partition values,
/// as many of the values only need to be calculated once and can be
/// reused for all other partition calculations.
///
/// The cache MUST have a length of at least `n + 1`.
///
/// An empty cache should contain only zeros, otherwise the
/// function will incorrectly assume that the non-zero values
/// hold a cached value, and will use this incorrect value 
/// in calculations.
///
/// # Panics
/// 
/// Panics if `n` is greater than `MAX_PART`.
/// This limit is put in place to prevent stack and
/// integer overflows.
///
/// Panics if `cache.len()` is less than `n + 1`.
///
/// # Examples
///
/// ```
/// use reikna::partition::part_wc;
/// let mut cache: Vec<u64> = vec![0; 101];
/// assert_eq!(part_wc(100, &mut cache), 190569292);
/// assert_eq!(part_wc(5, &mut cache), 7);
/// assert_eq!(part_wc(3, &mut cache), 3);
/// ```
pub fn part_wc(n: i64, cache: &mut Vec<u64>) -> u64 {
    assert!(n <= MAX_PART, "n value of {} is larger than MAX_PART!", n);
    if n == 0 {
        return 1;
    } else if n < 0 {
        return 0;
    } else if cache[n as usize] != 0 {
        return cache[n as usize];
    }

    let mut part: u64 = 0;
    let mut pent_n: i64 = 1;
    let mut pent: i64 = 0;
    while pent <= n {
        pent = gpn(pent_n);

        if (pent_n - 1) & 0x03 < 2 {
            part += part_wc(n - pent, cache);
        } else{
            part -= part_wc(n - pent, cache);
        }

        pent_n += 1;
    }

    cache[n as usize] = part;
    part
}

/// Calculate the partition function of `n`
///
/// This function works by constructing a temporary cache and
/// calling `part_wc()` with it and `n`. This function is best suited
/// to calculating single partition values, if multiple values will be
/// calculated it would be better to use `part_wc()` with an explicit
/// cache to preserve cached values between calculations.
///
/// # Panics
/// 
/// Panics if `n` is greater than `MAX_PART`.
/// This limit is put in place to prevent stack and
/// integer overflows.
/// 
/// # Examples
///
/// ```
/// use reikna::partition::part;
/// assert_eq!(part(100), 190569292);
/// assert_eq!(part(5), 7);
/// assert_eq!(part(3), 3);
/// ```
pub fn part(n: i64) -> u64 {
    assert!(n <= MAX_PART, "n value of {} is larger than MAX_PART!", n);
    let mut cache: Vec<u64> = vec![0; n as usize + 1];
    part_wc(n, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn t_part() {
        assert_eq!(part(0), 1);
        assert_eq!(part(1), 1);
        assert_eq!(part(2), 2);
        assert_eq!(part(3), 3);
        assert_eq!(part(4), 5);
        assert_eq!(part(5), 7);
        assert_eq!(part(100), 190569292);

        let mut cache: Vec<u64> = vec![0; 101];
        assert_eq!(part_wc(100, &mut cache), 190569292);
        assert_eq!(part_wc(5, &mut cache), 7);
        assert_eq!(part_wc(2, &mut cache), 2);
        assert_eq!(part_wc(1, &mut cache), 1);
        assert_eq!(part_wc(4, &mut cache), 5);
        assert_eq!(part_wc(0, &mut cache), 1);
        assert_eq!(part_wc(3, &mut cache), 3);

        part(MAX_PART);
    }

#[test]
#[should_panic]
    fn t_part_panic() {
        part(MAX_PART + 1);
    }
}
