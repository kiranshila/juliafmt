use super::event::Event;
use super::Parser;
use crate::lexer::RawToken;
use drop_bomb::DropBomb;

pub(super) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub(super) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub(super) fn complete(mut self, p: &mut Parser, kind: RawToken) -> CompletedMarker {
        // Defuse the bomb (this prevents destruction without calling this fn)
        self.bomb.defuse();

        // The thing we are replacing *must* be a Placeholder
        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        // Rewrite the node
        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        // Push the finish
        p.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

pub(super) struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub(super) fn precede(self, p: &mut Parser) -> Marker {
        let new_m = p.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events[self.pos]
        {
            *forward_parent = Some(new_m.pos - self.pos);
        } else {
            unreachable!();
        }

        new_m
    }
}
