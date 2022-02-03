use super::Parser;
use crate::event::Event;
use drop_bomb::DropBomb;
use syntax::SyntaxKind;

pub(crate) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub(crate) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
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

pub(crate) struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub(crate) fn precede(self, p: &mut Parser) -> Marker {
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
