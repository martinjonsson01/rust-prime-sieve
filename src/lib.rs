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
