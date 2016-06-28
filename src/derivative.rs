//! Module for working with derivatives.
//!
//! This module has functions for estimating and evaluating
//! derivatives of functions with one or more variables.

use super::func::*;

const EPSILON: f64 = 10.0e-8;

/// Return a `function` estimating the `n`th derivative of `f`.
///
/// TODO
pub fn nth_derivative(n: u64, f: &Function) -> Function {

    let f_copy = f.clone();
    let deriv: Function = func!(
        move |x: f64| {
            (f_copy(x + EPSILON) - f_copy(x)) / EPSILON
    });
    
    if n != 1 {
        return nth_derivative(n - 1, &deriv);
    }

    deriv
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::func::*;

#[test]
    fn t_nth_derivative() {
        let f: Function = func!(|x: f64| x * x);

        let deriv = nth_derivative(1, &f.clone());

        let deriv_2 = nth_derivative(2, &f.clone());
    }
}
