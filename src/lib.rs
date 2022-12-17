//! A collection of different prime sieves, with varying levels of optimization and concurrency.
//!
//! There are two main modules:
//! - ['singlethreaded']
//! - (todo) ['multithreaded']
//!
//! # Examples
//! ```
//! use prime_sieve::singlethreaded;
//!
//! let search_up_to = 1_000_000;
//! let primes: Vec<i32> = singlethreaded::optimized::find_primes(search_up_to);
//! ```

#![feature(strict_provenance)]
// rustc lints
#![warn(
    let_underscore,
    nonstandard_style,
    unused,
    explicit_outlives_requirements,
    fuzzy_provenance_casts,
    lossy_provenance_casts,
    meta_variable_misuse,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    trivial_casts,
    trivial_numeric_casts
)]
// clippy lints
#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::if_then_some_else_none,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_mutex,
    clippy::unwrap_used
)]

/// All multi-threaded prime sieve implementations.
pub mod multithreaded;
/// All single-threaded prime sieve implementations.
pub mod singlethreaded;

#[derive(Copy, Clone, PartialEq)]
struct Number {
    value: i32,
    is_prime: bool,
}

impl Number {
    fn new(number: i32, is_prime: bool) -> Self {
        Number {
            value: number,
            is_prime,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    fn sieves() -> Vec<fn(i32) -> Vec<i32>> {
        vec![
            crate::multithreaded::rayon::find_primes,
            crate::singlethreaded::optimized::find_primes,
            crate::singlethreaded::atkin::find_primes,
            crate::singlethreaded::basic::find_primes,
        ]
    }

    fn load_primes(path: String, search_up_to: i32) -> Vec<i32> {
        let content = fs::read_to_string(path).expect("Could not find file");
        content
            .lines()
            .filter_map(|text: &str| match text.parse::<i32>() {
                Ok(prime) if prime > search_up_to => None,
                Ok(prime) if prime <= search_up_to => Some(prime),
                _ => panic!("Could not parse {text:?} to i32"),
            })
            .collect()
    }

    #[test]
    fn verify_first_primes_up_to_10() {
        let search_up_to = 10;
        let expected_primes = load_primes(String::from("primes.txt"), search_up_to);
        for sieve in sieves() {
            let actual_primes = sieve(search_up_to);
            assert_eq!(actual_primes, expected_primes)
        }
    }

    #[test]
    fn verify_first_primes_up_to_100() {
        let search_up_to = 100;
        let expected_primes = load_primes(String::from("primes.txt"), search_up_to);
        for sieve in sieves() {
            let actual_primes = sieve(search_up_to);
            assert_eq!(actual_primes, expected_primes)
        }
    }

    #[test]
    fn verify_first_primes_up_to_100_000() {
        let search_up_to = 100_000;
        let expected_primes = load_primes(String::from("primes.txt"), search_up_to);
        for sieve in sieves() {
            let actual_primes = sieve(search_up_to);
            assert_eq!(actual_primes, expected_primes)
        }
    }
}
