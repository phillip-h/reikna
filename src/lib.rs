#![warn(missing_docs)]

//! A fast and lightweight math library
//!
//! `reikna` contains implementations of various useful
//! functions, structs, and algorithms from various branches
//! of mathematics, including number theory, graph theory,
//! and calculus. The library is designed with speed and
//! ease of use in mind.
//!
//! # Usage
//!
//! This library is on `crates.io`, and can be added to your
//! project by placing the following into your `Cargo.toml`
//!
//! ``` text
//! [dependencies]
//! reikna = "0.10.0"
//! ```
//!
//! and then importing the crate with
//!
//! ```text
//! #[macro_use] extern crate reikna;
//! ```
//!
//! Make sure to include the #[macro_use] part!
//!
//! # Modules
//!
//! A list of the modules currently included in this crate, along
//! with a brief description of each.
//!
//! * `aliquot` -- Functions for calcuating aliquot sums, divisor sums,
//!                and testing for perfect numbers and similar concepts.
//!
//! * `continued_fraction` -- Generate and expand continued fractions.
//!
//! * `derivative` -- Estimate derivatives of functions, along with slope
//!                   and concavity.
//!
//! * `factor` -- Compute the GCD, LCM, and prime factorization of numbers.
//!
//! * `figurate` -- Compute the value of various kinds of figurate numbers.
//!
//! * `func` -- Utility type alias and macro, used heavily in certain
//!             other modules.
//!
//! * `integral` -- Estimate integrals of functions using numeric integration.
//!
//! * `partition` -- Compute the value of the number theory partition
//!                  function.
//!
//! * `prime` -- Prime sieves, basic factoring algorithms, and primality
//!              tests.
//!
//! * `prime_count` -- Compute the value of the prime-counting function.
//!
//! * `totient` -- Compute Euler's Totient Function.
//!
//! # Examples
//!
//! ## Compute the number of primes under one million
//!
//! ```
//! # extern crate reikna;
//! # fn main() {
//! use reikna::prime::prime_sieve;
//!
//! let primes = prime_sieve(1_000_000);
//! println!("there are {} primes under one million!", primes.len());
//! # }
//! ```
//!
//! ## Factor a large integer
//!
//! ```
//! # extern crate reikna;
//! # fn main() {
//! use reikna::factor::quick_factorize;
//!
//! let my_number = 15_814_272_409_530_912_054;
//! let factors = quick_factorize(my_number);
//! println!("The prime factorization of {} is:", my_number);
//! println!("{:?}", factors);
//! }
//! ```
//!
//! Outputs:
//!
//! ```text
//! The prime factorization of 15814272409530912054 is:
//! [2, 3, 3, 23, 23, 61, 10007, 2720741641]
//! ```
//!
//! ## Relationship between the last digits of prime numbers
//!
//! Primes are considered to be pseudo-random, yet there exists
//! a relationship between the last digit of a prime number and the
//! last digit of the next prime number. For example, a prime ending
//! in `1` has only a 16% chance of being followed by another prime
//! that ends in one, at least in the range [1, 1,000,000].
//!
//!
//! ```
//! extern crate reikna;
//!
//! use reikna::prime::prime_sieve;
//!
//! pub fn main() {
//!
//!     // generate primes less than one million, removing
//!     // the single digit ones.
//!     let primes = &prime_sieve(1_000_000)[4..];
//!
//!
//!     let mut data = [[0u64; 10]; 10]; // 10x10 array to store the data
//!
//!     // loop through the primes and count digit frequency
//!     let mut old_last_digit = primes[0] % 10;
//!     for i in 1..primes.len() {
//!         let last_digit = primes[i] % 10;
//!         data[old_last_digit as usize][last_digit as usize] += 1;
//!         old_last_digit = last_digit;
//!     }
//!
//!     // store the totals into the 0's column, since it's not
//!     // being used for anything
//!     for i in 1..10 {
//!         data[0][i] = data[i].iter().fold(0, |acc, x| acc + x);
//!     }
//!
//!     // print out the data
//!     for i in vec![1, 3, 7, 9] {
//!         println!("primes ending in '{}':", i);
//!         println!(" * total -- {}", data[0][i]);
//!
//!         for k in vec![1, 3, 7, 9] {
//!             println!(" * % next prime ending in '{}' -- {}%",
//!                      k, data[i][k] as f64 / data[0][i] as f64 * 100.0);
//!         }
//!
//!         println!("");
//!     }
//! }
//! ```
//! By changing the max value, it can be observed that the bias
//! shrinks as the max grows.

#[macro_use] mod macros;

#[macro_use] pub mod func;
             pub mod aliquot;
             pub mod continued_fraction;
             pub mod derivative;
             pub mod factor;
             pub mod figurate;
             pub mod integral;
             pub mod partition;
#[macro_use] pub mod prime;
             pub mod prime_count;
             pub mod totient;

