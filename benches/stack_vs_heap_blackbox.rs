use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn stack() {
    for i in 0..10_000_000 {
        let arr = [0u8;1000];
        black_box(arr);
    }
}

fn heap() {
    for i in 0..10_000_000 {
        let arr = Box::new([0u8;1000]);
        black_box(arr);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = black_box(1);
    c.bench_function("stack_vs_heap: stack", |b| b.iter(|| stack()));
}


fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("stack_vs_heap: heap", |b| b.iter(|| heap()));
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
criterion_main!(benches);