use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

// Nodes for the grammar
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive, Hash, PartialOrd, Eq, Ord)]
pub enum SyntaxKind {
    Root,
    ParenExpr,
    PrefixExpr,
    InfixExpr,
    Literal,
    // Lexer Tokens
    Error,
    Whitespace,
    Integer,
    Hex,
    Octal,
    Binary,
    Float,
    Exponential,
    LParen,
    RParen,
    LBracket,
    RBracket,
    RBrace,
    LBrace,
    Newline,
    Semicolon,
    InlineComment,
    BlockComment,
    Baremodule,
    Begin,
    Break,
    Catch,
    Const,
    Continue,
    Do,
    Else,
    Elseif,
    End,
    Export,
    False,
    Finally,
    For,
    Function,
    Global,
    If,
    Import,
    Let,
    Local,
    Macro,
    Module,
    Quote,
    Return,
    Struct,
    True,
    Try,
    Using,
    While,
    Identifier,
    Dot,
    Dollar,
    Assignment,
    Pair,
    Conditional,
    Arrow,
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
    LambdaArrow,
    Subtype,
    Supertype,
    Not,
    LogicalNot,
    Radical,
    EscapedCharacter,
    LiteralCharacter,
    String,
    TripleString,
    Comma,
    At,
    Cmd,
    RParenTick,
    RBraceTick,
    RBracketTick,
    IdentifierTick,
    Tick,
}

// Convert from our raw lexer tokens into the grammar
impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        use TokenKind::*;
        match token_kind {
            Error => Self::Error,
            Whitespace => Self::Whitespace,
            Integer => Self::Integer,
            Hex => Self::Hex,
            Octal => Self::Octal,
            Binary => Self::Binary,
            Float => Self::Float,
            Exponential => Self::Exponential,
            LParen => Self::LParen,
            RParen => Self::RParen,
            LBracket => Self::LBracket,
            RBracket => Self::RBracket,
            RBrace => Self::RBrace,
            LBrace => Self::LBrace,
            Newline => Self::Newline,
            Semicolon => Self::Semicolon,
            InlineComment => Self::InlineComment,
            BlockComment => Self::BlockComment,
            Baremodule => Self::Baremodule,
            Begin => Self::Begin,
            Break => Self::Break,
            Catch => Self::Catch,
            Const => Self::Const,
            Continue => Self::Continue,
            Do => Self::Do,
            Else => Self::Else,
            Elseif => Self::Elseif,
            End => Self::End,
            Export => Self::Export,
            False => Self::False,
            Finally => Self::Finally,
            For => Self::For,
            Function => Self::Function,
            Global => Self::Global,
            If => Self::If,
            Import => Self::Import,
            Let => Self::Let,
            Local => Self::Local,
            Macro => Self::Macro,
            Module => Self::Module,
            Quote => Self::Quote,
            Return => Self::Return,
            Struct => Self::Struct,
            True => Self::True,
            Try => Self::Try,
            Using => Self::Using,
            While => Self::While,
            Identifier => Self::Identifier,
            Dot => Self::Dot,
            Dollar => Self::Dollar,
            Assignment => Self::Assignment,
            Pair => Self::Pair,
            Conditional => Self::Conditional,
            Arrow => Self::Arrow,
            Or => Self::Or,
            And => Self::And,
            Comparison => Self::Comparison,
            PipeL => Self::PipeL,
            PipeR => Self::PipeR,
            Colon => Self::Colon,
            Plus => Self::Plus,
            Times => Self::Times,
            Rational => Self::Rational,
            Bitshift => Self::Bitshift,
            Power => Self::Power,
            Decl => Self::Decl,
            LambdaArrow => Self::LambdaArrow,
            Subtype => Self::Subtype,
            Supertype => Self::Supertype,
            Not => Self::Not,
            LogicalNot => Self::LogicalNot,
            Radical => Self::Radical,
            EscapedCharacter => Self::EscapedCharacter,
            LiteralCharacter => Self::LiteralCharacter,
            String => Self::String,
            TripleString => Self::TripleString,
            Comma => Self::Comma,
            At => Self::At,
            Cmd => Self::Cmd,
            RParenTick => Self::RParenTick,
            RBraceTick => Self::RBraceTick,
            RBracketTick => Self::RBracketTick,
            IdentifierTick => Self::IdentifierTick,
            Tick => Self::Tick,
        }
    }
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace | Self::BlockComment | Self::InlineComment
        )
    }
}

// Syntax utilities that automates the conversion between rowan::SyntaxKind
// and out tokenizer output

pub type SyntaxNode = rowan::SyntaxNode<JuliaLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<JuliaLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<JuliaLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum JuliaLanguage {}

impl rowan::Language for JuliaLanguage {
    type Kind = SyntaxKind;
    // Use the derive methods to give us seamless conversion
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
