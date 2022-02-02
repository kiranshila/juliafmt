use juliafmt::lexer::{Lexeme, Lexer, RawToken};
use logos::Logos;

fn check(input: &str, token: RawToken) {
    let mut lexer = Lexer::new(input);
    assert_eq!(
        lexer.next(),
        Some(Lexeme {
            kind: token,
            text: input
        })
    );
}

fn check_stream(input: &str, tokens: Vec<RawToken>) {
    let mut lexer = RawToken::lexer(input);
    for token in tokens {
        assert_eq!(lexer.next(), Some(token));
    }
}

#[test]
fn whitespace() {
    check(" ", RawToken::Whitespace);
}

#[test]
fn lots_of_whitespace() {
    check("   \t ", RawToken::Whitespace);
}

#[test]
fn integer() {
    check("123", RawToken::Integer);
    check("1", RawToken::Integer);
}

#[test]
fn hex() {
    check("0xabc123", RawToken::Hex);
    check("0xDEADBEEF", RawToken::Hex);
}

#[test]
fn octal() {
    check("0o01234567", RawToken::Octal);
}

#[test]
fn binary() {
    check("0b10010010", RawToken::Binary);
}

#[test]
fn floats() {
    check("123.123", RawToken::Float);
    check("123.", RawToken::Float);
    check(".123", RawToken::Float);
}

#[test]
fn expontentials() {
    check("e3123", RawToken::Exponential);
    check("E233", RawToken::Exponential);
    check("f433", RawToken::Exponential);
    check("F3", RawToken::Exponential);
}

#[test]
fn compond_exponentials() {
    check_stream("123.123e123", vec![RawToken::Float, RawToken::Exponential]);
    check_stream(".123e123", vec![RawToken::Float, RawToken::Exponential]);
    check_stream("123e123", vec![RawToken::Integer, RawToken::Exponential]);
    check_stream("123.123e123", vec![RawToken::Float, RawToken::Exponential]);
    check_stream(".123f123", vec![RawToken::Float, RawToken::Exponential]);
    check_stream("123f123", vec![RawToken::Integer, RawToken::Exponential]);
}
