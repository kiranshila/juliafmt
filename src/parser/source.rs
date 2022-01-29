// Holds the lexemes and the current cursor position
use crate::lexer::{Lexeme, RawToken};

pub(super) struct Source<'l, 'input> {
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
}

impl<'l, 'input> Source<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self { lexemes, cursor: 0 }
    }

    pub(super) fn next_lexeme(&mut self) -> Option<&'l Lexeme<'input>> {
        self.eat_trivia();

        let lexeme = self.lexemes.get(self.cursor)?;
        self.cursor += 1;

        Some(lexeme)
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

    // Is the current cursored lexeme trivia?
    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, RawToken::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<RawToken> {
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
    }
}
