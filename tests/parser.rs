use expect_test::{expect, Expect};
use juliafmt::parser::Parser;

fn check(input: &str, expected_tree: Expect) {
    let parse = Parser::new(input).parse();
    expected_tree.assert_eq(&parse.debug_tree());
}

#[test]
fn parse_nothing() {
    check("", expect![[r#"Root@0..0"#]]);
}

#[test]
fn parse_numbers() {
    check(
        "123",
        expect![[r#"Root@0..3
  Integer@0..3 "123""#]],
    );
}
