use crate::singlethreaded::collect_marked;

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
    collect_marked(&numbers, &is_prime)
}

#[cfg(test)]
mod tests {

}
