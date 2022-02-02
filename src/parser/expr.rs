use super::marker::CompletedMarker;
use super::Parser;
use crate::lexer::RawToken;

// Operator precedence
enum InfixOp {
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

enum PrefixOp {
    Plus,
    Not,
    Radical,
}

enum PostfixOp {
    Tick,
}

// Precedence and associativity from
// https://docs.julialang.org/en/v1/manual/mathematical-operations/#Operator-Precedence-and-Associativity
impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Assignment => (2, 1),
            Self::Pair => (4, 3),
            Self::Conditional => (6, 5),
            Self::Or => (8, 7),
            Self::And => (10, 9),
            Self::Comparison => (11, 11),
            Self::PipeL => (16, 15),
            Self::PipeR => (17, 18),
            Self::Colon => (19, 20),
            Self::Plus => (21, 22),
            Self::Times => (23, 24),
            Self::Rational => (25, 26),
            Self::Bitshift => (27, 28),
            // Infix goes here
            Self::Power => (100, 101),
            Self::Decl => (102, 103),
            Self::Dot => (104, 105),
        }
    }
}

// Prefix operators have higher precedence than infix
impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Not => ((), 30),
            Self::Plus => ((), 30),
            Self::Radical => ((), 30),
        }
    }
}

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let mut lhs = if let Some(lhs) = lhs(p) {
        lhs
    } else {
        return; //Handle errors?
    };

    loop {
        // Grab from the precedence table
        let op = match p.peek() {
            Some(RawToken::Assignment) => InfixOp::Assignment,
            Some(RawToken::Pair) => InfixOp::Pair,
            Some(RawToken::Conditional) => InfixOp::Conditional,
            Some(RawToken::Or) => InfixOp::Or,
            Some(RawToken::And) => InfixOp::And,
            Some(RawToken::Comparison) => InfixOp::Comparison,
            Some(RawToken::PipeL) => InfixOp::PipeL,
            Some(RawToken::PipeR) => InfixOp::PipeR,
            Some(RawToken::Colon) => InfixOp::Colon,
            Some(RawToken::Plus) => InfixOp::Plus,
            Some(RawToken::Times) => InfixOp::Times,
            Some(RawToken::Rational) => InfixOp::Rational,
            Some(RawToken::Bitshift) => InfixOp::Bitshift,
            Some(RawToken::Power) => InfixOp::Power,
            Some(RawToken::Decl) => InfixOp::Decl,
            Some(RawToken::Dot) => InfixOp::Dot,
            _ => return, // we’ll handle errors later.
        };

        // Destructure binding power
        let (left_binding_power, right_binding_power) = op.binding_power();

        // Recursion base case
        if left_binding_power < minimum_binding_power {
            return;
        }

        // Eat the token, nom nom nom
        p.bump();

        // Recurse
        let m = lhs.precede(p);
        expr_binding_power(p, right_binding_power);
        lhs = m.complete(p, RawToken::BinaryExpr);
    }
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    // All the things that can be on either side of a binary operator
    let cm = match p.peek() {
        Some(RawToken::Identifier)
        | Some(RawToken::Integer)
        | Some(RawToken::Hex)
        | Some(RawToken::Octal)
        | Some(RawToken::Binary)
        | Some(RawToken::Float)
        | Some(RawToken::Exponential) => {
            let m = p.start();
            p.bump();
            m.complete(p, RawToken::Literal)
        }
        // Prefix operators
        Some(RawToken::Plus) | Some(RawToken::Not) | Some(RawToken::Radical) => {
            let op = match p.peek() {
                Some(RawToken::Plus) => PrefixOp::Plus,
                Some(RawToken::Not) => PrefixOp::Not,
                Some(RawToken::Radical) => PrefixOp::Radical,
                _ => return None, // we’ll handle errors later.
            };
            let m = p.start();
            let ((), right_binding_power) = op.binding_power();
            // Eat the token
            p.bump();

            expr_binding_power(p, right_binding_power);
            m.complete(p, RawToken::UnaryExpr)
        }
        // Parentheticals
        Some(RawToken::LParen) => {
            let m = p.start();
            // Consume the LParen
            p.bump();
            // A paren "resets" operator precedence, so we can just recur
            expr_binding_power(p, 0);
            // Parens must have pairs
            assert_eq!(p.peek(), Some(RawToken::RParen));
            // Consume the closing paren
            p.bump();
            m.complete(p, RawToken::Parenthetical)
        }
        _ => unreachable!(),
    };
    Some(cm)
}
