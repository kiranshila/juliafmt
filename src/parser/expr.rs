use super::marker::CompletedMarker;
use super::Parser;
use crate::lexer::RawToken;
use std::convert::TryInto;

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
impl TryFrom<Option<RawToken>> for BinaryOp {
    type Error = Option<RawToken>;
    fn try_from(item: Option<RawToken>) -> Result<Self, Self::Error> {
        use BinaryOp::*;
        match item {
            Some(RawToken::Assignment) => Ok(Assignment),
            Some(RawToken::Pair) => Ok(Pair),
            Some(RawToken::Conditional) => Ok(Conditional),
            Some(RawToken::Or) => Ok(Or),
            Some(RawToken::And) => Ok(And),
            Some(RawToken::Comparison) => Ok(Comparison),
            Some(RawToken::PipeL) => Ok(PipeL),
            Some(RawToken::PipeR) => Ok(PipeR),
            Some(RawToken::Colon) => Ok(Colon),
            Some(RawToken::Plus) => Ok(Plus),
            Some(RawToken::Times) => Ok(Times),
            Some(RawToken::Rational) => Ok(Rational),
            Some(RawToken::Bitshift) => Ok(Bitshift),
            Some(RawToken::Power) => Ok(Power),
            Some(RawToken::Decl) => Ok(Decl),
            Some(RawToken::Dot) => Ok(Dot),
            tok => Err(tok),
        }
    }
}

impl TryFrom<Option<RawToken>> for UnaryOp {
    type Error = Option<RawToken>;
    fn try_from(item: Option<RawToken>) -> Result<Self, Self::Error> {
        use UnaryOp::*;
        match item {
            Some(RawToken::Plus) => Ok(Plus),
            Some(RawToken::Not) => Ok(Not),
            Some(RawToken::Radical) => Ok(Radical),
            tok => Err(tok),
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
        let op: BinaryOp = match p.peek().try_into() {
            Ok(x) => x,
            Err(_) => return,
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
        lhs = m.complete(p, RawToken::InfixExpr);
    }
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = match p.peek() {
        // Everything that can be a literal
        Some(RawToken::Identifier) => literal(p),
        Some(RawToken::Integer) => literal(p),
        Some(RawToken::Hex) => literal(p),
        Some(RawToken::Octal) => literal(p),
        Some(RawToken::Binary) => literal(p),
        Some(RawToken::Float) => literal(p),
        Some(RawToken::Exponential) => literal(p),
        // Prefix Operators
        Some(RawToken::Plus) => prefix_expr(p),
        Some(RawToken::Not) => prefix_expr(p),
        Some(RawToken::Radical) => prefix_expr(p),
        // Parenthetical
        Some(RawToken::LParen) => paren_expr(p),
        _ => return None,
    };

    Some(cm)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump();
    m.complete(p, RawToken::Literal)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    let op: UnaryOp = match p.peek().try_into() {
        Ok(x) => x,
        Err(t) => match t {
            Some(_) => panic!("Expected a unary operator"),
            _ => unreachable!(),
        },
    };

    let ((), right_binding_power) = op.binding_power();
    // Eat the token
    p.bump();
    // Recur to build tree
    expr_binding_power(p, right_binding_power);

    m.complete(p, RawToken::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert_eq!(p.peek(), Some(RawToken::LParen));

    let m = p.start();
    // Eat
    p.bump();
    // Recur, resetting binding precedence
    expr_binding_power(p, 0);
    assert_eq!(p.peek(), Some(RawToken::RParen));
    // Consume the closing paren
    p.bump();
    m.complete(p, RawToken::ParenExpr)
}
