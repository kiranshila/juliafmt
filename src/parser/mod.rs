mod event;
mod expr;
mod marker;
mod sink;
mod source;

use crate::lexer::{Lexeme, Lexer, RawToken};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use marker::Marker;
use rowan::GreenNode;
use sink::Sink;
use source::Source;

// The parser will have the same lifetime as the lexer vec
struct Parser<'l, 'input> {
    source: Source<'l, 'input>,
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
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    // Simulate peeking into the token stream by just getting the nth lexeme specified by the current cursor
    pub fn peek(&mut self) -> Option<RawToken> {
        self.source.peek_kind()
    }

    // Pushes the current cursored lexeme into the event stream
    fn bump(&mut self) {
        self.source.next_lexeme().unwrap();
        self.events.push(Event::AddToken);
    }

    // The parser itself returns the event vector to be processed by the top level function parse
    pub fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, RawToken::Root);
        self.events
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
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
