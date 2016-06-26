//! Module for generating various kinds of figurate numbers.
//!
//! This module has functions for generating normal, generalized, 
//! and centered figurate numbers, as well as helper functions for
//! generating commonly used figurate numbers such 
//! as triangular numbers.

/// Return the `n`th figurate number with `s` sides
///
/// # Panics
///
/// Panics if `s` is less than three
///
/// # Examples
///
/// ```
/// use reikna::figurate::figurate;
/// let a = figurate(3, 3);
/// let b = figurate(45, 91);
/// let c = figurate(17, 5);
/// assert_eq!(vec![a, b, c], vec![6, 176176, 155]);
/// ```
pub fn figurate(s: i64, n: i64) -> i64 {
    assert!(s >= 3, "cannot generate figurate numbers with less than
                         three sides!");

    ((s - 2) * n * (n - 1) / 2) + n
}

/// Return the `n`th general figurate number with `s` sides
///
/// # Panics
///
/// Panics if `s` is less than three
///
/// # Examples
///
/// ```
/// use reikna::figurate::general_figurate;
/// let a = general_figurate(5, 0);
/// let b = general_figurate(5, 1);
/// let c = general_figurate(5, 2);
/// assert_eq!(vec![a, b, c], vec![0, 1, 2]);
/// ```
pub fn general_figurate(s: i64, n: i64) -> i64 {
    let mut n = n + 1;
    if n % 2 == 1 {
        n *= -1;
    }

    figurate(s, n / 2)
}

/// Return the `n`th centered figurate number with `s` sides
///
/// # Panics
///
/// Panics if `s` is less than three
///
/// # Examples
///
/// ```
/// use reikna::figurate::centered_figurate;
/// let a = centered_figurate(3, 3);
/// let b = centered_figurate(4, 9);
/// let c = centered_figurate(8, 5);
/// assert_eq!(vec![a, b, c], vec![10, 145, 81]);
/// ```
pub fn centered_figurate(s: i64, n: i64) -> i64 {
    assert!(s >= 3, "cannot generate figurate numbers with less than
                         three sides!");
    
    (s * n * (n - 1) / 2) + 1
}

/// Return the `n`th triangular number
///
/// # Examples
///
/// ```
/// use reikna::figurate::triangular_number;
/// assert_eq!(triangular_number(5), 15);
/// ```
pub fn triangular_number(n: i64) -> i64 { figurate(3, n) }

/// Return the `n`th square number
///
/// # Examples
///
/// ```
/// use reikna::figurate::square_number;
/// assert_eq!(square_number(5), 25);
/// ```
pub fn square_number(n: i64) -> i64 { figurate(4, n) }

/// Return the `n`th pentagonal number
///
/// # Examples
///
/// ```
/// use reikna::figurate::pentagonal_number;
/// assert_eq!(pentagonal_number(5), 35);
/// ```
pub fn pentagonal_number(n: i64) -> i64 { figurate(5, n) }

/// Return the `n`th hexagonal number
///
/// # Examples
///
/// ```
/// use reikna::figurate::hexagonal_number;
/// assert_eq!(hexagonal_number(5), 45);
/// ```
pub fn hexagonal_number(n: i64) -> i64 { figurate(6, n) }

/// return the `n`th general pentagonal number
///
/// # Examples
///
/// ```
/// use reikna::figurate::general_pentagonal_number;
/// assert_eq!(general_pentagonal_number(5), 12);
/// ```
pub fn general_pentagonal_number(n: i64) -> i64 { general_figurate(5, n) }


#[cfg(test)]
mod tests {
    use super::*;

#[test]
#[should_panic]
    fn t_figurate_panic() {
        figurate(2, 1);
    }

#[test]
    fn t_figurate() {
        assert_eq!(figurate(3, 0), 0);
        assert_eq!(figurate(3, 1), 1);
        assert_eq!(figurate(3, 2), 3);
        assert_eq!(figurate(3, 10), 55);

        assert_eq!(figurate(4, 0), 0);
        assert_eq!(figurate(4, 1), 1);
        assert_eq!(figurate(4, 2), 4);
        assert_eq!(figurate(4, 10), 100);

        assert_eq!(figurate(5, 0), 0);
        assert_eq!(figurate(5, 1), 1);
        assert_eq!(figurate(5, 2), 5);
        assert_eq!(figurate(5, 10), 145);

        assert_eq!(figurate(9, 0), 0);
        assert_eq!(figurate(9, 1), 1);
        assert_eq!(figurate(9, 2), 9);
        assert_eq!(figurate(9, 10), 325);
    }

#[test]
    fn t_general_figurate() {
        assert_eq!(general_figurate(5, 0), 0);
        assert_eq!(general_figurate(5, 1), 1);
        assert_eq!(general_figurate(5, 2), 2);
        assert_eq!(general_figurate(5, 3), 5);
        assert_eq!(general_figurate(5, 4), 7);
        assert_eq!(general_figurate(5, 5), 12);
        assert_eq!(general_figurate(5, 6), 15);
    }

#[test]
    fn t_centered_figurate() {
        assert_eq!(centered_figurate(3, 1), 1);
        assert_eq!(centered_figurate(3, 2), 4);
        assert_eq!(centered_figurate(3, 4), 19);

        assert_eq!(centered_figurate(4, 1), 1);
        assert_eq!(centered_figurate(4, 2), 5);
        assert_eq!(centered_figurate(4, 4), 25);

        assert_eq!(centered_figurate(7, 1), 1);
        assert_eq!(centered_figurate(7, 2), 8);
        assert_eq!(centered_figurate(7, 4), 43);
    }
}
