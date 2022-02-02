use crate::lexer::RawToken;

// Event-driven parser that we will collect and apply
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    StartNode {
        kind: RawToken,
        forward_parent: Option<usize>,
    },
    AddToken {
        kind: RawToken,
        text: String,
    },
    FinishNode,
    Placeholder,
}
