use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import the `longest_band` function from your module
use rust_datastructure_algo::vector::{longest_band, max_subarray, triplet}; // Replace with the correct module path

pub fn longest_benchmark(c: &mut Criterion) {
    // Input vector
    let input = vec![1, 3, 2, 0, 4, 6, 5, 7, 9, 10, 12, 14];

    // Benchmark the function
    c.bench_function("Benchmark longest_band", |b| {
        b.iter(|| {
            // Pass the input vector via `black_box` to prevent optimizations
            longest_band(black_box(input.clone()));
        });
    });
}

pub fn max_subarray_benchmark(c: &mut Criterion) {
    // Input vector
    let input = vec![-1, 2, 3, 5, -6, 11, 0, 9, 1];

    // Benchmark the function
    c.bench_function("Benchmark max_subarray", |b| {
        b.iter(|| {
            // Pass the input vector via `black_box` to prevent optimizations
            max_subarray(black_box(input.clone()));
        });
    });
}

criterion_group!(benches, max_subarray_benchmark);
criterion_main!(benches);
