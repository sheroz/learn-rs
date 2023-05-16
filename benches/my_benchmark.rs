use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use my_rust::my_algorithms::{fibonacci, palindrome};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci_recursive(20)", |b| {
        b.iter(|| fibonacci::fibonacci_recursive(black_box(20)))
    });
    c.bench_function("fibonacci_iterative(20)", |b| {
        b.iter(|| fibonacci::fibonacci_iterative(black_box(20)))
    });
}

fn fibonacci_multiple_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [20, 21].iter() {
        group.bench_with_input(BenchmarkId::new("fibonacci_recursive", i), i, |b, i| {
            b.iter(|| fibonacci::fibonacci_recursive(*i))
        });
        group.bench_with_input(BenchmarkId::new("fibonacci_iterative", i), i, |b, i| {
            b.iter(|| fibonacci::fibonacci_iterative(*i))
        });
    }
    group.finish();
}

fn palindrome_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Palindrome");

    let samples = [
        "Madam, I'm Adam!",
        "A man, a plan, a canal: Panama!",
        "Нажал кабан на баклажан.",
    ];

    for sample in samples.iter() {
        group.bench_with_input(
            BenchmarkId::new("is_palindrome_regex", sample),
            sample,
            |b, sample| b.iter(|| palindrome::is_palindrome_regex(sample)),
        );
        group.bench_with_input(
            BenchmarkId::new("is_palindrome_raw", sample),
            sample,
            |b, sample| b.iter(|| palindrome::is_palindrome_raw(sample)),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    fibonacci_benchmark,
    fibonacci_multiple_benchmark,
    palindrome_benchmark
);

criterion_main!(benches);
