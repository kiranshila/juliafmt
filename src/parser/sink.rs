use super::event::Event;
use crate::lexer::RawToken;
use crate::syntax::JuliaLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};

// The sink holds its own copy of the lexemes
pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [(RawToken, &'input str)],
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [(RawToken, &'input str)], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        // Rewrite history and remove all start nodes objs and rebuild them with checkpoints in place
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.into_iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(checkpoint, Event::StartNode { kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(JuliaLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => {
                    self.builder.token(JuliaLanguage::kind_to_raw(kind), &text)
                }
                Event::FinishNode => self.builder.finish_node(),
            }
        }
        self.builder.finish()
    }
}
