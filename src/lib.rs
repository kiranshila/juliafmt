extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate logos;
extern crate serde;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pretty::{Arena, DocAllocator, DocBuilder};
use serde::Deserialize;
use std::io::Write;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum RawToken {
    #[error]
    Error,
    #[regex(r"[ \t\u{a0}]+")]
    Whitespace,
    // Numbers
    #[regex(r"-?[0-9]+")]
    Integer,
    #[regex(r"-?0x[0-9a-fA-F]+")]
    Hex,
    #[regex(r"-?0o[0-7]+")]
    Octal,
    #[regex(r"-?0b[01]+")]
    Binary,
    #[regex(r"[+-]?([0-9]*[.][0-9]+)|([0-9]+[.][0-9]*)")]
    Float,
    #[regex(r"[+-]?[0-9]?[.]?[0-9]+[eEfF][0-9]+")]
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
    #[regex(r"[_a-zA-Z\u{FF}-\u{10FFFF}][!_a-zA-Z\u{FF}-\u{10FFFF}0-9]*")]
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
    #[regex(r#"'(\\[\\"'nrbtfav\?]|\\[0-7]+|\\x[0-9a-fA-F]|\\[uU][0-9a-fA-F]+)'"#)]
    EscapedCharacter,
    // Kill me - this is broken when we look at the tick operator
    #[regex(r"'(.|[\u{FF}-\u{10FFFF}])'")]
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
    #[token("'")]
    Tick,
}

// Parser Configuration and Construction
#[derive(Parser)]
#[grammar = "parser/julia.pest"]
pub struct JuliaParser;

//Config File Definition
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    indent_style: IndentStyle,
}

#[derive(Deserialize, Debug)]
pub struct IndentStyle {
    indent_width: usize,
    column_width: usize,
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle {
            indent_width: 4,
            column_width: 92,
        }
    }
}

fn intersperse_pairs<'a, A: 'a, D: DocAllocator<'a, A>>(
    pairs: Pairs<Rule>,
    allocator: &'a D,
    config: &Config,
    separator: DocBuilder<'a, D, A>,
) -> DocBuilder<'a, D, A>
where
    D::Doc: Clone,
    A: Clone,
{
    allocator.intersperse(pairs.map(|x| to_doc(x, allocator, config)), separator)
}

fn intersperse_children<'a, A: 'a, D: DocAllocator<'a, A>>(
    pair: Pair<Rule>,
    allocator: &'a D,
    config: &Config,
    separator: DocBuilder<'a, D, A>,
) -> DocBuilder<'a, D, A>
where
    D::Doc: Clone,
    A: Clone,
{
    intersperse_pairs(pair.into_inner(), allocator, config, separator)
}

fn to_doc<'a, A: 'a, D: DocAllocator<'a, A>>(
    pair: Pair<Rule>,
    allocator: &'a D,
    config: &Config,
) -> DocBuilder<'a, D, A>
where
    D::Doc: Clone,
    A: Clone,
{
    match pair.as_rule() {
        Rule::block_generic => allocator
            .text("begin")
            .append(allocator.hardline())
            .append(
                intersperse_children(pair, allocator, config, allocator.nil())
                    .indent(config.indent_style.indent_width),
            )
            .append(allocator.hardline())
            .append("end")
            .align(),
        Rule::statement => {
            let mut contents = pair.into_inner();
            let expr_doc = to_doc(contents.next().unwrap(), allocator, config);
            if contents.any(|x| x.as_rule() == Rule::SEMICOLON) {
                expr_doc.append(";")
            } else {
                expr_doc
            }
        }
        Rule::property => {
            let mut contents = pair.into_inner();
            let expr_doc = to_doc(contents.next().unwrap(), allocator, config)
                .append(intersperse_pairs(contents, allocator, config, allocator.line_()).align());
            expr_doc.clone().parens().flat_alt(expr_doc).group()
        }
        Rule::assignment => {
            let mut contents = pair.into_inner();
            let lhs_doc = to_doc(contents.next().unwrap(), allocator, config);
            let eq_doc = to_doc(contents.next().unwrap(), allocator, config);
            let rhs_doc = to_doc(contents.next().unwrap(), allocator, config);
            lhs_doc
                .append(allocator.space())
                .append(eq_doc)
                .append(allocator.space())
                .append(rhs_doc.align().group())
        }
        Rule::funcall => {
            let mut contents = pair.into_inner();
            let function = to_doc(contents.next().unwrap(), allocator, config);
            let expression = to_doc(contents.next().unwrap(), allocator, config);
            function.append(expression.align().parens())
        }
        rule => {
            let pair_cpy = pair.clone();
            let contents = pair.into_inner();
            if contents.peek().is_some() {
                let sub = |x| intersperse_pairs(contents, allocator, config, x);
                match rule {
                    Rule::statement_sequence | Rule::block_body => sub(allocator.hardline()),
                    Rule::infix_lhs | Rule::ret => sub(allocator.space()),
                    Rule::infix | Rule::tuple => sub(allocator.line()).group(),
                    Rule::parenthetical => sub(allocator.nil()).align().parens(),
                    _ => sub(allocator.nil()),
                }
            } else {
                allocator.text(pair_cpy.as_str().to_owned())
            }
        }
    }
}

// Exported functions
pub fn lex<W: Write>(input: &str, out: &mut W) -> Result<(), ()> {
    let lex: Vec<_> = RawToken::lexer(input).spanned().collect();
    write!(out, "\n{:?}", lex).unwrap();
    Ok(())
}

pub fn cst<W: Write>(input: &str, out: &mut W) -> Result<(), pest::error::Error<Rule>> {
    let cst = JuliaParser::parse(Rule::program, input).unwrap();
    write!(out, "{:#?}", cst).unwrap();
    Ok(())
}

pub fn format<W: Write>(
    input: &str,
    config: &Config,
    out: &mut W,
) -> Result<(), pest::error::Error<Rule>> {
    // The only thing in Rule::inner will be the program, so pull that out
    let ast = JuliaParser::parse(Rule::program, input).unwrap();
    let allocator = Arena::<()>::new();
    //let allocator = BoxAllocator;
    for pair in ast {
        to_doc(pair, &allocator, config)
            .render(config.indent_style.column_width, out)
            .unwrap();
    }
    Ok(())
}

pub fn lex_until_error(s: String) -> (u64, u64) {
    let mut tokens = 0u64;
    let mut errors = 0u64;
    for (token, span) in RawToken::lexer(&s).spanned() {
        if token == RawToken::Error {
            errors += 1;
            //return Err(format!("Hit a unknown token: {:?}", &s[span]));
        }
        tokens += 1;
    }
    (tokens, errors)
}
