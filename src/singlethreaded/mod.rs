/// A basic Sieve of Atkin.
pub mod atkin;
/// A basic Sieve of Eratosthenes.
pub mod basic;
/// An optimized Sieve of Eratosthenes.
pub mod optimized;

fn collect_marked(values: &[i32], marks: &[bool]) -> Vec<i32> {
    values
        .iter()
        .zip(marks.iter())
        .filter(|(_, mark)| **mark)
        .map(|(value, _)| *value)
        .collect()
}
