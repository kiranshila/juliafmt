extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use pest::iterators::Pair;
use pest::Parser;
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
    indent_width: u16,
    newline: String,
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle {
            indent_width: 4,
            newline: "\n".to_owned(),
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq, Eq)]
enum FormatAction {
    FreshLine,
    Whitespace,
    Content,
}

// Formatting Details
#[derive(Debug, Copy, Clone)]
struct Context {
    indent_depth: u16,
    last_action: FormatAction,
}

fn indent(config: &Config, context: &Context) -> String {
    " ".repeat(config.indent_style.indent_width as usize)
        .repeat(context.indent_depth as usize)
}

trait Format {
    fn format<W: Write>(self, config: &Config, context: &mut Context, out: &mut W);
}

impl Format for Pair<'_, Rule> {
    fn format<W: Write>(self, config: &Config, context: &mut Context, out: &mut W) {
        let indentation = indent(config, context);
        match self.as_rule() {
            Rule::statement_sequence => {
                if context.last_action == FormatAction::FreshLine {
                    write!(out, "{}", indentation).unwrap();
                    context.last_action = FormatAction::Whitespace;
                }
                for pair in self.into_inner() {
                    pair.format(config, context, out);
                }
            }
            Rule::block_generic => {
                context.indent_depth += 1;
                write!(out, "begin").unwrap();
                context.last_action = FormatAction::Content;
                for pair in self.into_inner() {
                    pair.format(config, context, out);
                }
                context.indent_depth -= 1;
                if context.last_action == FormatAction::FreshLine {
                    write!(out, "{}", indentation).unwrap();
                    context.last_action = FormatAction::Whitespace;
                }
                write!(out, "end").unwrap();
                context.last_action = FormatAction::Content;
            }
            Rule::program => {
                for pair in self.into_inner() {
                    pair.format(config, context, out);
                }
            }
            Rule::SEMICOLON => {
                write!(out,";{}",config.indent_style.newline).unwrap();
                context.last_action = FormatAction::FreshLine;
            }
            Rule::NL => {
                if context.last_action != FormatAction::FreshLine {
                    write!(out,"{}",config.indent_style.newline).unwrap();
                    context.last_action = FormatAction::FreshLine;
                }
            }
            Rule::WHITESPACE => {
                if context.last_action == FormatAction::Content {
                    write!(out," ").unwrap();
                    context.last_action = FormatAction::Whitespace;
                }
            }
            _ => {
                write!(out, "{}", self.as_str()).unwrap();
                context.last_action = FormatAction::Content;
            }
        };
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
    // Create the program starting context
    let mut context = Context {
        indent_depth: 0,
        last_action: FormatAction::FreshLine,
    };
    // Format each top level pair
    for pair in ast {
        pair.format(config, &mut context, out);
    }
    Ok(())
}
