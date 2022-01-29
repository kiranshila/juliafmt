use super::event::Event;
use crate::lexer::{Lexeme, RawToken};
use crate::syntax::JuliaLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

// The sink holds its own copy of the lexemes
pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        // Rewrite history and remove all start nodes objs and rebuild them with checkpoints in place
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(JuliaLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self.token(kind, &text),
                Event::FinishNode => self.builder.finish_node(),
            }
            self.eat_whitespace();
        }
        self.builder.finish()
    }

    fn token(&mut self, kind: RawToken, text: &str) {
        self.builder.token(JuliaLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn eat_whitespace(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if lexeme.kind != RawToken::Whitespace {
                break;
            }
            self.token(lexeme.kind, lexeme.text);
        }
    }
}
