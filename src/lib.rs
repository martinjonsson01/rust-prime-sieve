pub mod singlethreaded;

#[derive(Copy, Clone, PartialEq)]
struct Number {
    value: i32,
    is_prime: bool,
}

impl Number {
    fn new(number: i32, is_prime: bool) -> Self {
        Number { value: number, is_prime }
    }
}