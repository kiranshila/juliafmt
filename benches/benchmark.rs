use criterion::{criterion_group, criterion_main, Criterion};
use juliafmt::lexer::{Lexer, RawToken};
use std::fs;
use walkdir::WalkDir;

fn lex_until_error(s: String) -> Result<(u64, u64), String> {
    let mut tokens = 0u64;
    let mut errors = 0u64;
    for (token, span) in Lexer::new(&s).inner.spanned() {
        if token == RawToken::Error {
            errors += 1;
            let sp = span.clone();
            return Err(format!(
                "Hit a unknown token: {:?} at span {:?}",
                &s[span], sp
            ));
        }
        tokens += 1;
    }
    Ok((tokens, errors))
}

fn lex_all_julia() -> Result<(u64, u64), String> {
    let mut t = 0;
    let mut e = 0;
    for entry in WalkDir::new("/usr/share/julia")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() != None)
        .filter(|e| e.path().extension().unwrap() == "jl")
        .filter(|e| !e.path().is_dir())
    {
        match lex_until_error(fs::read_to_string(entry.path()).unwrap()) {
            Ok((tokens, errors)) => {
                t += tokens;
                e += errors;
            }
            Err(s) => return Err(format!("\nError in {:?}\n{}\n", entry.path(), s)),
        }
    }
    Ok((t, e))
}

fn bench_lex(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexing");
    let (t, e) = lex_all_julia().unwrap();
    println!("Lexed {} tokens with {} errors", t, e);
    group.significance_level(0.1).sample_size(500);
    group.bench_function("lex_all_julia", |b| b.iter(|| lex_all_julia()));
    group.finish();
}

criterion_group!(benches, bench_lex);
criterion_main!(benches);
