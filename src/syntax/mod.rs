use crate::lexer::RawToken;
use num_traits::{FromPrimitive, ToPrimitive};

// Syntax utilities that automates the conversion between rowan::SyntaxKind
// and out tokenizer output

pub type SyntaxNode = rowan::SyntaxNode<JuliaLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<JuliaLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<JuliaLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum JuliaLanguage {}

impl rowan::Language for JuliaLanguage {
    type Kind = RawToken;
    // Use the derive methods to give us seamless conversion
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
