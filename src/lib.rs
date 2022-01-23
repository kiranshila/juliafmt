extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use pest::iterators::Pair;
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
            column_width: 80,
        }
    }
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
        Rule::program => allocator.concat(pair.into_inner().map(|x| to_doc(x, allocator, config))),
        Rule::statement_sequence => allocator.intersperse(
            pair.into_inner().map(|x| to_doc(x, allocator, config)),
            allocator.hardline(),
        ),
        Rule::block_body => allocator
            .concat(pair.into_inner().map(|x| to_doc(x, allocator, config)))
            .indent(config.indent_style.indent_width),

        Rule::block_generic => allocator
            .text("begin")
            .append(allocator.hardline())
            .append(
                allocator
                    .concat(pair.into_inner().map(|x| to_doc(x, allocator, config)))
                    .group(),
            )
            .append(allocator.hardline())
            .append("end"),
        Rule::statement => {
            let mut contents = pair.into_inner();
            let expr_doc = to_doc(contents.next().unwrap(), allocator, config);
            if contents.any(|x| x.as_rule() == Rule::SEMICOLON) {
                expr_doc.append(";")
            } else {
                expr_doc
            }
        }
        _ => allocator.text(pair.as_str().to_owned()),
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
    for pair in ast {
        to_doc(pair, &allocator, config)
            .render(config.indent_style.column_width, out)
            .unwrap();
    }
    Ok(())
}
