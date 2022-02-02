use expect_test::expect;
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

#[test]
fn parse_plus() {
    check(
        "1+2+3+4+5+6",
        expect![[r#"
            Root@0..11
              InfixExpr@0..11
                InfixExpr@0..9
                  InfixExpr@0..7
                    InfixExpr@0..5
                      InfixExpr@0..3
                        Literal@0..1
                          Integer@0..1 "1"
                        Plus@1..2 "+"
                        Literal@2..3
                          Integer@2..3 "2"
                      Plus@3..4 "+"
                      Literal@4..5
                        Integer@4..5 "3"
                    Plus@5..6 "+"
                    Literal@6..7
                      Integer@6..7 "4"
                  Plus@7..8 "+"
                  Literal@8..9
                    Integer@8..9 "5"
                Plus@9..10 "+"
                Literal@10..11
                  Integer@10..11 "6""#]],
    );
}

#[test]
fn parse_complex() {
    check(
        "1+2^(3+4)>>5",
        expect![[r#"
            Root@0..12
              InfixExpr@0..12
                Literal@0..1
                  Integer@0..1 "1"
                Plus@1..2 "+"
                InfixExpr@2..12
                  InfixExpr@2..9
                    Literal@2..3
                      Integer@2..3 "2"
                    Power@3..4 "^"
                    ParenExpr@4..9
                      LParen@4..5 "("
                      InfixExpr@5..8
                        Literal@5..6
                          Integer@5..6 "3"
                        Plus@6..7 "+"
                        Literal@7..8
                          Integer@7..8 "4"
                      RParen@8..9 ")"
                  Bitshift@9..11 ">>"
                  Literal@11..12
                    Integer@11..12 "5""#]],
    );
}

#[test]
fn parse_with_whitespace() {
    check(
        "1 + 2 >> 3",
        expect![[r#"
            Root@0..10
              InfixExpr@0..10
                Literal@0..2
                  Integer@0..1 "1"
                  Whitespace@1..2 " "
                Plus@2..3 "+"
                Whitespace@3..4 " "
                InfixExpr@4..10
                  Literal@4..6
                    Integer@4..5 "2"
                    Whitespace@5..6 " "
                  Bitshift@6..8 ">>"
                  Whitespace@8..9 " "
                  Literal@9..10
                    Integer@9..10 "3""#]],
    );
}
