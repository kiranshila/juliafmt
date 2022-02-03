pub(crate) mod marker;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use marker::Marker;
use syntax::SyntaxKind;

// The parser will have the same lifetime as the lexer vec
pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    // Constructor for the parser
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    // The parser itself returns the event vector to be processed by the top level function parse
    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    // Pushes the current cursored token into the event stream
    pub(crate) fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    // Simulate peeking into the token stream by just getting the nth token specified by the current cursor
    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_inline_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  InlineComment@0..8 "# hello!""##]],
        );
    }

    #[test]
    fn parse_block_comment() {
        check(
            "#=block boi=#",
            expect![[r##"
Root@0..13
  BlockComment@0..13 "#=block boi=#""##]],
        );
    }
}
