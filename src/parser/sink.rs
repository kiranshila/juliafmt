use super::event::Event;
use crate::lexer::{Lexeme, RawToken};
use crate::syntax::JuliaLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::mem;

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
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];

                    let mut idx = idx;

                    let mut forward_parent = forward_parent;

                    // Walk through the forward parent of the forward parent, etc. until we reach a StartNode event without a forward parent
                    while let Some(fp) = forward_parent {
                        idx += fp;

                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(JuliaLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken { kind, text } => self.token(kind, &text),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
            }
            self.eat_trivia();
        }
        self.builder.finish()
    }

    fn token(&mut self, kind: RawToken, text: &str) {
        self.builder.token(JuliaLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if !lexeme.kind.is_trivia() {
                break;
            }
            self.token(lexeme.kind, lexeme.text);
        }
    }
}
