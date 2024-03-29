use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn sample_benchmark(c: &mut Criterion) {
    /*
    c.bench_function("fibonacci_recursive(20)", |b| {
        b.iter(|| fibonacci::fibonacci_recursive(black_box(20)))
    });
    c.bench_function("fibonacci_iterative(20)", |b| {
        b.iter(|| fibonacci::fibonacci_iterative(black_box(20)))
    });
     */
}

fn sample_multiple_benchmark(c: &mut Criterion) {
    /*
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
     */
}

criterion_group!(
    benches,
    sample_benchmark,
    sample_multiple_benchmark,
);

criterion_main!(benches);
