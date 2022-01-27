use crate::expr::expr;
use crate::lexer::{Lexer, RawToken};
use crate::syntax::{JuliaLanguage, SyntaxNode};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

// The parser will have the same lifetime as the lexer
// We'll wrap the lexer in a peekable iterator so we can look at the next token without consuming it
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

// Holding the parse result
pub struct Parse {
    green_node: GreenNode,
}

impl<'a> Parser<'a> {
    // Constructor for the parser
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    // Peek into the token stream and return an Option<RawToken>
    // Peek by itself returns a (&RawToken,&str)
    pub fn peek(&mut self) -> Option<RawToken> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    // Adds the lexeme the lexer is currently at to the current branch of the parse three
    pub fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.builder.token(JuliaLanguage::kind_to_raw(kind), text);
    }

    // The parser itself returns an instance of a Parse
    pub fn parse(mut self) -> Parse {
        self.start_node(RawToken::Root);
        expr(&mut self);
        self.finish_node();
        Parse {
            green_node: self.builder.finish(),
        }
    }

    // Some utilities for the start and finish nodes

    pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: RawToken) {
        self.builder
            .start_node_at(checkpoint, JuliaLanguage::kind_to_raw(kind));
    }

    pub fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub fn start_node(&mut self, kind: RawToken) {
        self.builder.start_node(JuliaLanguage::kind_to_raw(kind));
    }

    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}

// Public methods for the Parse result
impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);
        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}
