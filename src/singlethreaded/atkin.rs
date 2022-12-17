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
    let mut sieve: Vec<Number> = (0..=search_up_to).map(|n| Number::new(n, false)).collect();

    for x in (1..=search_up_to).take_while(|x| x.pow(2) <= search_up_to) {
        for y in (1..=search_up_to).take_while(|y| y.pow(2) <= search_up_to) {
            //condition one
            let n = 4 * x.pow(2) + y.pow(2);
            if n <= search_up_to && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize].is_prime = !sieve[n as usize].is_prime;
            }
            //condition two
            let n = 3 * x.pow(2) + y.pow(2);
            if n <= search_up_to && n % 12 == 7 {
                sieve[n as usize].is_prime = !sieve[n as usize].is_prime;
            }
            //condition three
            let n = 3 * x.pow(2) - y.pow(2);
            if x > y && n <= search_up_to && n % 12 == 11 {
                sieve[n as usize].is_prime = !sieve[n as usize].is_prime;
            }
        }
    }

    for r in (5_i32..).take_while(|r: &i32| r.pow(2) <= search_up_to) {
        if !sieve[r as usize].is_prime {
            continue;
        }
        let r_squared = r.pow(2);
        for i in (r_squared..=search_up_to).step_by(r_squared as usize) {
            sieve[i as usize].is_prime = false;
        }
    }

    let mut primes = vec![2, 3];
    for n in 5..=search_up_to {
        if sieve[n as usize].is_prime {
            primes.push(n);
        }
    }
    primes
}
