mod event;
mod expr;
mod sink;

use crate::lexer::{Lexer, RawToken};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use rowan::GreenNode;
use sink::Sink;
use std::iter::Peekable;

// The parser will have the same lifetime as the lexer
// We'll wrap the lexer in a peekable iterator so we can look at the next token without consuming it
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    events: Vec<Event>,
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
            events: Vec::new(),
        }
    }

    // Peek into the token stream and return an Option<RawToken>
    // Peek by itself returns a (&RawToken,&str)
    pub fn peek(&mut self) -> Option<RawToken> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    // Pushes an event that will add the lexeme the lexer is currently at to the current branch of the parse three
    pub fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.events.push(Event::AddToken {
            kind,
            text: text.into(),
        });
    }

    // The parser itself returns an instance of a Parse
    pub fn parse(mut self) -> Parse {
        self.start_node(RawToken::Root);
        expr(&mut self);
        self.finish_node();

        let sink = Sink::new(self.events);

        Parse {
            green_node: sink.finish(),
        }
    }

    // Some utilities for the start and finish nodes
    pub fn start_node_at(&mut self, checkpoint: usize, kind: RawToken) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    pub fn checkpoint(&self) -> usize {
        self.events.len()
    }

    pub fn start_node(&mut self, kind: RawToken) {
        self.events.push(Event::StartNode { kind });
    }

    pub fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
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
