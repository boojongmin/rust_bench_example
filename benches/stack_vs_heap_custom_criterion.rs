use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn stack() {
    for i in 0..10_000_000 {
        let arr = [0u8;1000];
        if arr[i % 100] == 100 {
            println!(">>>")
        }
    }
}

fn heap() {
    for i in 0..10_000_000 {
        let arr = Box::new([0u8;1000]);
        if arr[i % 100] == 100 {
            println!(">>>")
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_vs_heap");
    group.sample_size(10);
    group.warm_up_time(Duration::from_nanos(1));
    group.bench_function("stack_vs_heap: stack", |b| b.iter(|| stack()));
    group.bench_function("stack_vs_heap: heap", |b| b.iter(|| heap()));
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);