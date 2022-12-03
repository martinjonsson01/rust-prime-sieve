pub fn find_primes(search_up_to: i32) -> Vec<i32> {
    let numbers: Vec<i32> = (0..=search_up_to).collect();
    let mut is_prime: Vec<bool> = vec![true; numbers.len()];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut next_prime: Option<i32> = Some(2);
    while let Some(prime) = next_prime {
        let start_multiple = 2 * prime; // Don't want to mark the prime number itself.
        for multiple in (start_multiple..=search_up_to).step_by(prime as usize) {
            is_prime[multiple as usize] = false;
        }
        let mut numbers_greater_than_prime = (prime + 1)..=search_up_to;
        next_prime = numbers_greater_than_prime.find(|number| is_prime[*number as usize]);
    }
    collect_marked(numbers, is_prime)
}

fn collect_marked(values: Vec<i32>, marks: Vec<bool>) -> Vec<i32> {
    values.iter().zip(marks.iter())
        .filter(|(_, mark)| **mark)
        .map(|(value, _)| *value).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn load_primes(path: String, search_up_to: i32) -> Vec<i32> {
        let content = fs::read_to_string(path).expect("Could not find file");
        content.lines().filter_map(|text: &str| {
            match text.parse::<i32>() {
                Ok(prime) if prime > search_up_to => None,
                Ok(prime) if prime <= search_up_to => Some(prime),
                _ => panic!("Could not parse {text:?} to i32")
            }
        }).collect()
    }

    #[test]
    fn verify_first_primes_up_to_10() {
        let search_up_to = 10;
        let expected_primes = load_primes(String::from("primes.txt"), search_up_to);
        let actual_primes = find_primes(search_up_to);
        assert_eq!(actual_primes, expected_primes)
    }

    #[test]
    fn verify_first_primes_up_to_100() {
        let search_up_to = 100;
        let expected_primes = load_primes(String::from("primes.txt"), search_up_to);
        let actual_primes = find_primes(search_up_to);
        assert_eq!(actual_primes, expected_primes)
    }

    #[test]
    fn verify_first_primes_up_to_100_000() {
        let search_up_to = 100_000;
        let expected_primes = load_primes(String::from("primes.txt"), search_up_to);
        let actual_primes = find_primes(search_up_to);
        assert_eq!(actual_primes, expected_primes)
    }
}