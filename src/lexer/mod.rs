use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

// We're deriving the *Primitive stuff here to provide seamless conversion
// to rowan's types
#[derive(
    Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive, Eq, PartialOrd, Ord, Hash,
)]
pub enum RawToken {
    // Root node for the parser, not a lexing token
    Root,
    #[error]
    Error,
    #[regex(r"[ \t\u{a0}]+")]
    Whitespace,
    // Numbers
    #[regex(r"[0-9]+")]
    Integer,
    #[regex(r"-?0x[0-9a-fA-F]+")]
    Hex,
    #[regex(r"-?0o[0-7]+")]
    Octal,
    #[regex(r"-?0b[01]+")]
    Binary,
    #[regex(r"([0-9]*[.][0-9]+)|([0-9]+[.][0-9]*)")]
    Float,
    #[regex(r"[eEfF][0-9]+")]
    Exponential,
    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    RBrace,
    #[token("}")]
    LBrace,
    #[regex(r"\r?\n")]
    Newline,
    #[token(";")]
    Semicolon,
    // Comments
    #[regex(r"#[^\r\n]*(\r\n|\n)?")]
    InlineComment,
    #[regex(r"#=([^=]|=[^#])*=#")]
    BlockComment,
    // Keywords
    #[token("baremodule")]
    Baremodule,
    #[token("begin")]
    Begin,
    #[token("break")]
    Break,
    #[token("catch")]
    Catch,
    #[token("const")]
    Const,
    #[token("continue")]
    Continue,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("elseif")]
    Elseif,
    #[token("end")]
    End,
    #[token("export")]
    Export,
    #[token("false")]
    False,
    #[token("finally")]
    Finally,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("global")]
    Global,
    #[token("if")]
    If,
    #[token("import")]
    Import,
    #[token("let")]
    Let,
    #[token("local")]
    Local,
    #[token("macro")]
    Macro,
    #[token("module")]
    Module,
    #[token("quote")]
    Quote,
    #[token("return")]
    Return,
    #[token("struct")]
    Struct,
    #[token("true")]
    True,
    #[token("try")]
    Try,
    #[token("using")]
    Using,
    #[token("while")]
    While,
    // Identifiers
    #[regex(r"[a-zA-Z_\u{A1}-\u{10FFFF}][0-9!a-zA-Z_\u{A1}-\u{10FFFF}]*")]
    Identifier,
    // Operators
    #[token(".")]
    Dot,
    #[token("$")]
    Dollar,
    #[regex(r"\$=|:=|~|=|\+=|-=|−=|\*=|/=|//=|\\=|\^=|÷=|%=|<<=|>>=|>>>=|\|=|\&=|⊻=|≔|⩴|≕")]
    Assignment,
    #[token("=>")]
    Pair,
    #[token("?")]
    Conditional,
    #[regex(r"←|→|↔|↚|↛|↞|↠|↢|↣|↦|↤|↮|⇎|⇍|⇏|⇐|⇒|⇔|⇴|⇶|⇷|⇸|⇹|⇺|⇻|⇼|⇽|⇾|⇿|⟵|⟶|⟷|⟹|⟺|⟻|⟼|⟽|⟾|⟿|⤀|⤁|⤂|⤃|⤄|⤅|⤆|⤇|⤌|⤍|⤎|⤏|⤐|⤑|⤔|⤕|⤖|⤗|⤘|⤝|⤞|⤟|⤠|⥄|⥅|⥆|⥇|⥈|⥊|⥋|⥎|⥐|⥒|⥓|⥖|⥗|⥚|⥛|⥞|⥟|⥢|⥤|⥦|⥧|⥨|⥩|⥪|⥫|⥬|⥭|⥰|⧴|⬱|⬰|⬲|⬳|⬴|⬵|⬶|⬷|⬸|⬹|⬺|⬻|⬼|⬽|⬾|⬿|⭀|⭁|⭂|⭃|⭄|⭇|⭈|⭉|⭊|⭋|⭌|￩|￫|⇜|⇝|↜|↝|↩|↪|↫|↬|↼|↽|⇀|⇁|⇄|⇆|⇇|⇉|⇋|⇌|⇚|⇛|⇠|⇢|↷|↶|↺|↻|-->|<--|<-->")]
    Arrow,
    #[token("||")]
    Or,
    #[token("&&")]
    And,
    #[regex(r"in|isa|>|<|>=|≥|<=|≤|==|===|≡|!=|≠|!==|≢|∈|∉|∋|∌|⊆|⊈|⊂|⊄|⊊|∝|∊|∍|∥|∦|∷|∺|∻|∽|∾|≁|≃|≂|≄|≅|≆|≇|≈|≉|≊|≋|≌|≍|≎|≐|≑|≒|≓|≖|≗|≘|≙|≚|≛|≜|≝|≞|≟|≣|≦|≧|≨|≩|≪|≫|≬|≭|≮|≯|≰|≱|≲|≳|≴|≵|≶|≷|≸|≹|≺|≻|≼|≽|≾|≿|⊀|⊁|⊃|⊅|⊇|⊉|⊋|⊏|⊐|⊑|⊒|⊜|⊩|⊬|⊮|⊰|⊱|⊲|⊳|⊴|⊵|⊶|⊷|⋍|⋐|⋑|⋕|⋖|⋗|⋘|⋙|⋚|⋛|⋜|⋝|⋞|⋟|⋠|⋡|⋢|⋣|⋤|⋥|⋦|⋧|⋨|⋩|⋪|⋫|⋬|⋭|⋲|⋳|⋴|⋵|⋶|⋷|⋸|⋹|⋺|⋻|⋼|⋽|⋾|⋿|⟈|⟉|⟒|⦷|⧀|⧁|⧡|⧣|⧤|⧥|⩦|⩧|⩪|⩫|⩬|⩭|⩮|⩯|⩰|⩱|⩲|⩳|⩵|⩶|⩷|⩸|⩹|⩺|⩻|⩼|⩽|⩾|⩿|⪀|⪁|⪂|⪃|⪄|⪅|⪆|⪇|⪈|⪉|⪊|⪋|⪌|⪍|⪎|⪏|⪐|⪑|⪒|⪓|⪔|⪕|⪖|⪗|⪘|⪙|⪚|⪛|⪜|⪝|⪞|⪟|⪠|⪡|⪢|⪣|⪤|⪥|⪦|⪧|⪨|⪩|⪪|⪫|⪬|⪭|⪮|⪯|⪰|⪱|⪲|⪳|⪴|⪵|⪶|⪷|⪸|⪹|⪺|⪻|⪼|⪽|⪾|⪿|⫀|⫁|⫂|⫃|⫄|⫅|⫆|⫇|⫈|⫉|⫊|⫋|⫌|⫍|⫎|⫏|⫐|⫑|⫒|⫓|⫔|⫕|⫖|⫗|⫘|⫙|⫷|⫸|⫹|⫺|⊢|⊣|⟂|⫪|⫫")]
    Comparison,
    #[regex(r"<\||\|>")]
    Pipe,
    #[regex(r":|\.\.|…|⁝|⋮|⋱|⋰|⋯")]
    Colon,
    #[regex(r"\+|-|−|¦|\||⊕|⊖|⊞|⊟|\+\+|∪|∨|⊔|±|∓|∔|∸|≏|⊎|⊻|⊽|⋎|⋓|⧺|⧻|⨈|⨢|⨣|⨤|⨥|⨦|⨧|⨨|⨩|⨪|⨫|⨬|⨭|⨮|⨹|⨺|⩁|⩂|⩅|⩊|⩌|⩏|⩐|⩒|⩔|⩖|⩗|⩛|⩝|⩡|⩢|⩣")]
    Plus,
    #[regex(r"\*|/|⌿|÷|%|&|·|·|⋅|∘|×|\\|∩|∧|⊗|⊘|⊙|⊚|⊛|⊠|⊡|⊓|∗|∙|∤|⅋|≀|⊼|⋄|⋆|⋇|⋉|⋊|⋋|⋌|⋏|⋒|⟑|⦸|⦼|⦾|⦿|⧶|⧷|⨇|⨰|⨱|⨲|⨳|⨴|⨵|⨶|⨷|⨸|⨻|⨼|⨽|⩀|⩃|⩄|⩋|⩍|⩎|⩑|⩓|⩕|⩘|⩚|⩜|⩞|⩟|⩠|⫛|⊍|▷|⨝|⟕|⟖|⟗|⨟")]
    Times,
    #[token("//")]
    Rational,
    #[regex(r"<<|>>|>>>")]
    Bitshift,
    #[regex(r"\^|↑|↓|⇵|⟰|⟱|⤈|⤉|⤊|⤋|⤒|⤓|⥉|⥌|⥍|⥏|⥑|⥔|⥕|⥘|⥙|⥜|⥝|⥠|⥡|⥣|⥥|⥮|⥯|￪|￬")]
    Power,
    #[token("::")]
    Decl,
    #[token("->")]
    LambdaArrow,
    #[token("<:")]
    Subtype,
    #[token(">:")]
    Supertype,
    #[token("!")]
    Not,
    #[regex(r"√|∛|∜")]
    Radical,
    // Other Literals
    #[regex(r#"'(\\.|\\[0-7]+|\\x[0-9a-fA-F]|\\[uU][0-9a-fA-F]+)'"#)]
    EscapedCharacter,
    // Kill me - this is broken when we look at the tick operator
    #[regex(r"'[\u{21}-\u{10FFFF}]'")]
    LiteralCharacter,
    #[regex(r#""(?s:[^"\\]|\\.)*""#)]
    String,
    // Hacky, the regex version doesn't work
    // r#""""(?s:.*)[^"]?"#
    #[token(r#"""""#, |lex| {
        let len = lex.remainder().find(r#"""""#)?;
        lex.bump(len+3);
        Some(())
    })]
    TripleString,
    #[token(",")]
    Comma,
    #[token("@")]
    At,
    #[regex(r#"`(?s:[^`\\]|\\.)*`"#)]
    Cmd,
    // Cursed edge cases
    #[token(")'")]
    RParenTick,
    #[token("}'")]
    RBraceTick,
    #[token("]'")]
    RBracketTick,
    #[regex(r"[a-zA-Z_\u{A1}-\u{10FFFF}][0-9!a-zA-Z_\u{A1}-\u{10FFFF}]*'")]
    IdentifierTick,
    #[token("'")]
    Tick,
}

// We need to wrap the lexer to make it play nice with rowan
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, RawToken>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: RawToken::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (RawToken, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        Some((kind, text))
    }
}
