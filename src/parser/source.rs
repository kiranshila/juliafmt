// Holds the tokens and the current cursor position
use crate::lexer::{RawToken, Token};

pub(super) struct Source<'t, 'input> {
    tokens: &'t [Token<'input>],
    cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
    pub(super) fn new(tokens: &'t [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(super) fn next_token(&mut self) -> Option<&'t Token<'input>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(super) fn peek_kind(&mut self) -> Option<RawToken> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    // Cursor past trivia
    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    // Is the current cursored token trivia?
    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, RawToken::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<RawToken> {
        self.tokens.get(self.cursor).map(|Token { kind, .. }| *kind)
    }
}
