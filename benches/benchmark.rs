use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::io::sink;

fn bench_parse(c: &mut Criterion) {
    let input = fs::read_to_string("test/test.jl").unwrap();
    let mut output = sink();
    c.bench_function("Parse test file", |b| {
        b.iter(|| juliafmt::ast(&input, &mut output))
    });
}

fn bench_format(c: &mut Criterion) {
    let input = "begin foo end\n".to_owned();
    let mut output = sink();
    let config = juliafmt::Config::default();
    c.bench_function("simple block", |b| {
        b.iter(|| juliafmt::format(&input, &config, &mut output))
    });
}

criterion_group!(benches, bench_format, bench_parse);
criterion_main!(benches);
