mod event;
mod expr;
mod sink;

use crate::lexer::{Lexeme, Lexer, RawToken};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use rowan::GreenNode;
use sink::Sink;

// The parser will have the same lifetime as the lexer vec
struct Parser<'l, 'input> {
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

// Holding the parse result
pub struct Parse {
    green_node: GreenNode,
}

impl<'l, 'input> Parser<'l, 'input> {
    // Constructor for the parser
    fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self {
            lexemes,
            cursor: 0,
            events: Vec::new(),
        }
    }

    // Simulate peeking into the token stream by just getting the nth lexeme specified by the current cursor
    pub fn peek(&mut self) -> Option<RawToken> {
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
    }

    // Pushes the current cursored lexeme into the event stream
    fn bump(&mut self) {
        let Lexeme { kind, text } = self.lexemes[self.cursor];

        self.cursor += 1;
        self.events.push(Event::AddToken {
            kind,
            text: text.into(),
        });
    }

    // Keep tracking past whitespace
    fn eat_whitespace(&mut self) {
        while self.peek_raw() == Some(RawToken::Whitespace) {
            self.cursor += 1;
        }
    }

    // The parser itself returns the event vector to be processed by the top level function parse
    pub fn parse(mut self) -> Vec<Event> {
        self.start_node(RawToken::Root);
        expr(&mut self);
        self.finish_node();
        self.events
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

    fn peek_raw(&self) -> Option<RawToken> {
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
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

pub fn parse(input: &str) -> Parse {
    let lexemes: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&lexemes);
    let events = parser.parse();
    let sink = Sink::new(&lexemes, events);

    Parse {
        green_node: sink.finish(),
    }
}
