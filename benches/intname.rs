use criterion::{black_box, criterion_group, criterion_main, Criterion};
use intname::integer_name;

fn intname_benchmark(c: &mut Criterion) {
    c.bench_function("integer names", |b| b.iter(|| integer_name(black_box(0i128))));
}

criterion_group!(benches, intname_benchmark);
criterion_main!(benches);