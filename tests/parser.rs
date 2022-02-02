use expect_test::{expect, Expect};
use juliafmt::parser::parse;

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[test]
fn parse_numbers() {
    check(
        "123",
        expect![[r#"
            Root@0..3
              Literal@0..3
                Integer@0..3 "123""#]],
    );
}
