use crate::singlethreaded::optimized::mark_primes;
use rayon::prelude::*;

const CHUNK_SIZE: usize = 100_000;

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
    // Faster to find small primes sequentially.
    let small_max = (search_up_to as f64).sqrt().ceil() as i32;
    let mut marks: Vec<bool> = mark_primes(small_max);
    marks.resize(search_up_to as usize, true);

    let (small_marks, large_marks) = marks.split_at_mut(small_max as usize);
    let small_marks = &*small_marks;

    large_marks
        .par_chunks_mut(CHUNK_SIZE)
        .enumerate()
        .for_each(|(chunk_index, chunk)| {
            let start = chunk_index * CHUNK_SIZE + small_max as usize;
            let end = start + chunk.len();
            small_marks
                .iter()
                .enumerate()
                .filter(|(_, &is_prime)| is_prime)
                .flat_map(|(prime, _)| {
                    (prime..end)
                        .step_by(prime)
                        .skip_while(|&number| number < start)
                })
                .for_each(|not_prime| chunk[not_prime - start] = false);
        });

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
