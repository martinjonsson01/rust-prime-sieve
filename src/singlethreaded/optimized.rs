#[derive(Copy, Clone, PartialEq)]
struct Number {
    value: i32,
    is_prime: bool,
}

impl Number {
    fn new(number: i32) -> Self {
        Number { value: number, is_prime: true }
    }
}

pub fn find_primes(search_up_to: i32) -> Vec<i32> {
    let mut numbers: Vec<Number> = (2..=search_up_to)
        .map(Number::new)
        .collect();
    let mut next_prime: Option<Number> = Some(numbers[0]);
    while let Some(prime) = next_prime {
        // The multiples of p that are smaller than p^2 are already marked.
        let start_multiple = match prime.value.checked_mul(prime.value) {
            None => break, // If outside of range for i32, then there's nothing left to sieve.
            Some(product) => product,
        };
        if start_multiple > search_up_to {
            break;
        }
        for multiple in (start_multiple..=search_up_to).step_by(prime.value as usize) {
            numbers[multiple as usize - 2].is_prime = false;
        }
        
        next_prime = numbers.iter().find(|number| number.value > prime.value && number.is_prime).cloned();
    }
    collect_marked_2(numbers)
}

fn collect_marked_2(values: Vec<Number>) -> Vec<i32> {
    values.iter().filter(|number| number.is_prime)
        .map(|number| number.value).collect()
}

#[cfg(test)]
mod tests {}