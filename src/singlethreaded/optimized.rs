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
    let marks = mark_primes(search_up_to);
    collect_marked(marks)
}

pub(crate) fn mark_primes(search_up_to: i32) -> Vec<bool> {
    let mut marks: Vec<bool> = (0..=search_up_to).map(|_| true).collect();
    marks[0] = false;
    marks[1] = false;
    let mut next_prime = Some(2_i32);
    while let Some(prime) = next_prime {
        // The multiples of p that are smaller than p^2 are already marked.
        let start_multiple = prime.pow(2);
        if start_multiple > search_up_to {
            break;
        }

        marks
            .iter_mut()
            .skip(start_multiple as usize)
            .step_by(prime as usize)
            .for_each(|is_prime| *is_prime = false);

        next_prime = marks
            .iter()
            .enumerate()
            .find(|(number, &is_prime)| *number as i32 > prime && is_prime)
            .map(|(number, _is_prime)| number as i32);
    }
    marks
}

fn collect_marked<Iter>(marks: Iter) -> Vec<i32>
where
    Iter: IntoIterator<Item = bool>,
{
    marks
        .into_iter()
        .enumerate()
        .filter(|(_index, is_prime)| *is_prime)
        .map(|(index, _is_prime)| index as i32)
        .collect()
}

#[cfg(test)]
mod tests {}
