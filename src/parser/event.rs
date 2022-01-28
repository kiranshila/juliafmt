use crate::lexer::RawToken;

// Event-driven parser that we will collect and apply
#[derive(Debug, Clone)]
pub enum Event {
    StartNode { kind: RawToken },
    StartNodeAt { kind: RawToken, checkpoint: usize },
    AddToken { kind: RawToken, text: String },
    FinishNode,
}
