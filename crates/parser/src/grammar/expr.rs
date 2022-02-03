use super::{CompletedMarker, Parser};
use std::convert::TryFrom;
use syntax::SyntaxKind;

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

// Operator precedence
enum BinaryOp {
    Assignment,
    Pair,
    Conditional,
    Or,
    And,
    Comparison,
    PipeL,
    PipeR,
    Colon,
    Plus,
    Times,
    Rational,
    Bitshift,
    Power,
    Decl,
    Dot,
}

enum UnaryOp {
    Plus,
    Not,
    Radical,
}

// Precedence and associativity from
// https://docs.julialang.org/en/v1/manual/mathematical-operations/#Operator-Precedence-and-Associativity
impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        use BinaryOp::*;
        match self {
            Assignment => (2, 1),
            Pair => (4, 3),
            Conditional => (6, 5),
            Or => (8, 7),
            And => (10, 9),
            Comparison => (11, 11),
            PipeL => (16, 15),
            PipeR => (17, 18),
            Colon => (19, 20),
            Plus => (21, 22),
            Times => (23, 24),
            Rational => (25, 26),
            Bitshift => (27, 28),
            // Infix goes here
            Power => (100, 101),
            Decl => (102, 103),
            Dot => (104, 105),
        }
    }
}

// Prefix operators have higher precedence than infix
impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        use UnaryOp::*;
        match self {
            Not => ((), 30),
            Plus => ((), 30),
            Radical => ((), 30),
        }
    }
}

// Raw token to expr tokens
impl TryFrom<SyntaxKind> for BinaryOp {
    type Error = SyntaxKind;
    fn try_from(item: SyntaxKind) -> Result<Self, Self::Error> {
        use BinaryOp::*;
        match item {
            SyntaxKind::Assignment => Ok(Assignment),
            SyntaxKind::Pair => Ok(Pair),
            SyntaxKind::Conditional => Ok(Conditional),
            SyntaxKind::Or => Ok(Or),
            SyntaxKind::And => Ok(And),
            SyntaxKind::Comparison => Ok(Comparison),
            SyntaxKind::PipeL => Ok(PipeL),
            SyntaxKind::PipeR => Ok(PipeR),
            SyntaxKind::Colon => Ok(Colon),
            SyntaxKind::Plus => Ok(Plus),
            SyntaxKind::Times => Ok(Times),
            SyntaxKind::Rational => Ok(Rational),
            SyntaxKind::Bitshift => Ok(Bitshift),
            SyntaxKind::Power => Ok(Power),
            SyntaxKind::Decl => Ok(Decl),
            SyntaxKind::Dot => Ok(Dot),
            tok => Err(tok),
        }
    }
}

impl TryFrom<SyntaxKind> for UnaryOp {
    type Error = SyntaxKind;
    fn try_from(item: SyntaxKind) -> Result<Self, Self::Error> {
        use UnaryOp::*;
        match item {
            SyntaxKind::Plus => Ok(Plus),
            SyntaxKind::Not => Ok(Not),
            SyntaxKind::Radical => Ok(Radical),
            tok => Err(tok),
        }
    }
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        // Grab from the precedence table
        let op: BinaryOp = if let Ok(op) = p.peek()?.try_into() {
            op
        } else {
            break;
        };

        // Destructure binding power
        let (left_binding_power, right_binding_power) = op.binding_power();

        // Recursion base case
        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the token, nom nom nom
        p.bump();

        // Recurse
        let m = lhs.precede(p);
        expr_binding_power(p, right_binding_power);
        lhs = m.complete(p, SyntaxKind::InfixExpr);
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = match p.peek() {
        // Everything that can be a literal
        Some(SyntaxKind::Identifier) => literal(p),
        Some(SyntaxKind::Integer) => literal(p),
        Some(SyntaxKind::Hex) => literal(p),
        Some(SyntaxKind::Octal) => literal(p),
        Some(SyntaxKind::Binary) => literal(p),
        Some(SyntaxKind::Float) => literal(p),
        Some(SyntaxKind::Exponential) => literal(p),
        // Prefix Operators
        Some(SyntaxKind::Plus) => prefix_expr(p),
        Some(SyntaxKind::Not) => prefix_expr(p),
        Some(SyntaxKind::Radical) => prefix_expr(p),
        // Parenthetical
        Some(SyntaxKind::LParen) => paren_expr(p),
        _ => return None,
    };

    Some(cm)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

/// # Panic
/// p.next() must  be a `UnaryOp`
fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    let op: UnaryOp = p
        .peek()
        .expect("Called prefix_expr when the next token None")
        .try_into()
        .expect("Expected UnaryOp");

    let ((), right_binding_power) = op.binding_power();
    // Eat the token
    p.bump();
    // Recur to build tree
    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert_eq!(p.peek(), Some(SyntaxKind::LParen));

    let m = p.start();
    // Eat
    p.bump();
    // Recur, resetting binding precedence
    expr_binding_power(p, 0);
    assert_eq!(p.peek(), Some(SyntaxKind::RParen));
    // Consume the closing paren
    p.bump();
    m.complete(p, SyntaxKind::ParenExpr)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
                Root@0..3
                  Literal@0..3
                    Integer@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
                Root@0..7
                  Whitespace@0..3 "   "
                  Literal@3..7
                    Integer@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
                Root@0..6
                  Literal@0..6
                    Integer@0..3 "999"
                    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
                Root@0..9
                  Whitespace@0..1 " "
                  Literal@1..9
                    Integer@1..4 "123"
                    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
                Root@0..7
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
                      Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    InfixExpr@0..5
                      Literal@0..1
                        Integer@0..1 "1"
                      Plus@1..2 "+"
                      InfixExpr@2..5
                        Literal@2..3
                          Integer@2..3 "2"
                        Times@3..4 "*"
                        Literal@4..5
                          Integer@4..5 "3"
                    Plus@5..6 "-"
                    Literal@6..7
                      Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
                Root@0..12
                  Whitespace@0..1 " "
                  InfixExpr@1..12
                    Literal@1..3
                      Integer@1..2 "1"
                      Whitespace@2..3 " "
                    Plus@3..4 "+"
                    Whitespace@4..7 "   "
                    InfixExpr@7..12
                      Literal@7..8
                        Integer@7..8 "2"
                      Times@8..9 "*"
                      Whitespace@9..10 " "
                      Literal@10..12
                        Integer@10..11 "3"
                        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_binary_expression_interspersed_with_comments() {
        check(
            "1 + # What's next?
2 + # Add Two
3 # Add three",
            expect![[r##"
                Root@0..46
                  InfixExpr@0..46
                    InfixExpr@0..21
                      Literal@0..2
                        Integer@0..1 "1"
                        Whitespace@1..2 " "
                      Plus@2..3 "+"
                      Whitespace@3..4 " "
                      InlineComment@4..19 "# What's next?\n"
                      Literal@19..21
                        Integer@19..20 "2"
                        Whitespace@20..21 " "
                    Plus@21..22 "+"
                    Whitespace@22..23 " "
                    InlineComment@23..33 "# Add Two\n"
                    Literal@33..46
                      Integer@33..34 "3"
                      Whitespace@34..35 " "
                      InlineComment@35..46 "# Add three""##]],
        )
    }
}
