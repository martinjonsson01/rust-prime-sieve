use crate::Number;

/// Finds all of the prime numbers that exist up to the given limit.
///
/// `search_up_to` is an upper limit at which to stop the search.
/// Note that this is not how many primes to find, it's the upper bound for the search space.
///
/// # Examples
/// ```
/// # use prime_sieve::singlethreaded::basic::find_primes;
///
/// let primes: Vec<i32> = find_primes(1_000_000);
/// ```
pub fn find_primes(search_up_to: i32) -> Vec<i32> {
    let mut numbers: Vec<Number> = (0..=search_up_to).map(|n| Number::new(n, true)).collect();
    numbers[0].is_prime = false;
    numbers[1].is_prime = false;
    let mut next_prime: Option<Number> = Some(numbers[2]);
    while let Some(prime) = next_prime {
        // The multiples of p that are smaller than p^2 are already marked.
        let start_multiple = prime.value.pow(2);
        if start_multiple > search_up_to {
            break;
        }

        numbers
            .iter_mut()
            .skip(start_multiple as usize)
            .step_by(prime.value as usize)
            .for_each(|number| number.is_prime = false);

        next_prime = numbers
            .iter()
            .find(|number| number.value > prime.value && number.is_prime)
            .cloned();
    }
    collect_marked_2(numbers)
}

fn collect_marked_2(values: Vec<Number>) -> Vec<i32> {
    values
        .iter()
        .filter(|number| number.is_prime)
        .map(|number| number.value)
        .collect()
}

#[cfg(test)]
mod tests {}
