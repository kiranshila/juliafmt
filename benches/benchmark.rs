use criterion::{criterion_group, criterion_main, Criterion};
use std::io::sink;

fn bench_format(c: &mut Criterion) {
    let input = "begin foo end\n".to_owned();
    let mut output = sink();
    let config = juliafmt::Config::default();
    c.bench_function("simple block", |b| {
        b.iter(|| juliafmt::format(&input, &config, &mut output))
    });
}

criterion_group!(benches, bench_format);
criterion_main!(benches);
