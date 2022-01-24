extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pretty::{Arena, DocAllocator, DocBuilder};
use serde::Deserialize;
use std::io::Write;

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
        rule => {
            let pair_cpy = pair.clone();
            let contents = pair.into_inner();
            if contents.peek().is_some() {
                let sub = |x| intersperse_pairs(contents, allocator, config, x);
                match rule {
                    Rule::statement_sequence | Rule::block_body => sub(allocator.hardline()),
                    Rule::infix_lhs => sub(allocator.space()),
                    Rule::infix => sub(allocator.line()).group(),
                    Rule::parenthetical => sub(allocator.nil()).align().parens(),
                    _ => sub(allocator.nil()),
                }
            } else {
                allocator.text(pair_cpy.as_str().to_owned())
            }
        }
    }
}

// Exported function
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

pub fn ast<W: Write>(input: &str) -> Result<(), pest::error::Error<Rule>> {
    let ast = JuliaParser::parse(Rule::program, input).unwrap();
    println!("{:#?}", ast);
    Ok(())
}
