use rayon::prelude::*;

/// Concurrently finds all of the prime numbers that exist up to the given limit.
///
/// `search_up_to` is an upper limit at which to stop the search.
/// Note that this is not how many primes to find, it's the upper bound for the search space.
///
/// # Examples
/// ```
/// # use prime_sieve::multithreaded::rayon::find_primes;
///
/// let primes: Vec<i32> = find_primes(1_000_000);
/// ```
pub fn find_primes(search_up_to: i32) -> Vec<i32> {
    let cpus = num_cpus::get_physical();
    let numbers_per_thread = (search_up_to as usize) / cpus;
    let mut marks: Vec<bool> = (0..=search_up_to).map(|_| true).collect();
    marks[0] = false;
    marks[1] = false;
    let mut next_prime: Option<i32> = Some(2_i32);
    while let Some(prime) = next_prime {
        // The multiples of p that are smaller than p^2 are already marked.
        let start_multiple = prime.pow(2);
        if start_multiple > search_up_to {
            break;
        }

        marks
            .par_iter_mut()
            .with_min_len(numbers_per_thread)
            .skip(start_multiple as usize)
            .step_by(prime as usize)
            .for_each(|is_prime| *is_prime = false);

        next_prime = marks
            .par_iter()
            .with_min_len(numbers_per_thread)
            .enumerate()
            .find_first(|(number, &is_prime)| *number as i32 > prime && is_prime)
            .map(|(number, _is_prime)| number as i32);
    }
    collect_marked(marks)
}

fn collect_marked(values: Vec<bool>) -> Vec<i32> {
    let cpus = num_cpus::get_physical();
    let numbers_per_thread = values.len() / cpus;
    values
        .par_iter()
        .with_min_len(numbers_per_thread)
        .enumerate()
        .filter(|(_number, &is_prime)| is_prime)
        .map(|(number, _is_prime)| number as i32)
        .collect()
}
