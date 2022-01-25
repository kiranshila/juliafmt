use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::io::sink;
use walkdir::WalkDir;

fn bench_parse(c: &mut Criterion) {
    let input = fs::read_to_string("test/test.jl").unwrap();
    let mut output = sink();
    c.bench_function("Parse test file", |b| {
        b.iter(|| juliafmt::cst(&input, &mut output))
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

fn lex_all_julia() {
    for entry in WalkDir::new("/usr/share/julia")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() != None)
        .filter(|e| e.path().extension().unwrap() == "jl")
        .filter(|e| !e.path().is_dir())
    {
        juliafmt::lex_until_error(fs::read_to_string(entry.path()).unwrap());
    }
}

fn bench_lex(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexing");
    group.significance_level(0.1).sample_size(500);
    group.bench_function("lex_all_julia", |b| b.iter(|| lex_all_julia()));
    group.finish();
}

criterion_group!(benches, bench_lex);
criterion_main!(benches);
