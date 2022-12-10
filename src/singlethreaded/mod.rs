use std::fs;

pub mod basic;
pub mod optimized;
pub mod atkins;

fn collect_marked(values: Vec<i32>, marks: Vec<bool>) -> Vec<i32> {
    values.iter().zip(marks.iter())
        .filter(|(_, mark)| **mark)
        .map(|(value, _)| *value).collect()
}

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

#[cfg(test)]
mod tests {
    use crate::singlethreaded::load_primes;

    fn sieves() -> Vec<fn(i32) -> Vec<i32>> {
        vec![crate::singlethreaded::basic::find_primes,
             crate::singlethreaded::optimized::find_primes,
             crate::singlethreaded::atkins::find_primes]
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