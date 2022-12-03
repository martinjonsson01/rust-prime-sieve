﻿use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput, Bencher};
use prime_sieve::singlethreaded;

fn criterion_benchmark(c: &mut Criterion) {
    let group_name = "single-threaded prime sieve";
    let to_bench = |bencher: &mut Bencher, search_up_to: &i32| {
        bencher.iter(|| singlethreaded::basic::find_primes(*search_up_to));
    };
    benchmark_sieve(c, group_name, to_bench);
}

fn benchmark_sieve(c: &mut Criterion, group_name: &str, to_bench: fn(&mut Bencher, &i32)) {
    let mut group = c.benchmark_group(group_name);
    for magnitude in 1..=6 {
        let search_up_to = 10_i32.pow(magnitude);
        group.throughput(Throughput::Elements(search_up_to as u64));
        group.bench_with_input(BenchmarkId::from_parameter(format!("10^{magnitude:?}")),
                               &search_up_to,
                               to_bench);
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);