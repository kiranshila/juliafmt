extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use pest::iterators::Pair;
use pest::Parser;
use serde::Deserialize;

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
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle { indent_width: 4 }
    }
}

// Formatting Details
#[derive(Debug)]
struct Context {
    indent_depth: u16,
}

fn indent(config: &Config, context: &Context) -> String {
    " ".repeat(config.indent_style.indent_width as usize)
        .repeat(context.indent_depth as usize)
}

trait Format {
    fn format(&self, config: &Config, context: &Context) -> String;
}

impl Format for Pair<'_, Rule> {
    fn format(&self, config: &Config, context: &Context) -> String {
        let mut output = String::new();
        let indentation = indent(config, context);
        match self.as_rule() {
            Rule::variable => output.push_str(self.as_str()),
            Rule::statement_sequence => {
                let children: Vec<Pair<Rule>> = self.clone().into_inner().collect();
                for pair in children {
                    output.push_str(&format!("{}{}", indentation, pair.format(config, context)));
                }
            }
            Rule::terminator => output.push_str(";"),
            _ => (),
        };
        output
    }
}

// Exported function
pub fn format(input: &str, config: &Config) -> Result<String, pest::error::Error<Rule>> {
    // The only thing in Rule::inner will be the program, so pull that out
    let ast = JuliaParser::parse(Rule::program, input)?.peek().unwrap();
    let mut output = String::new();
    // Create the program starting context
    let context = Context { indent_depth: 0 };
    // Format each top level pair
    for pair in ast.into_inner() {
        output.push_str(&pair.format(config, &context));
    }
    Ok(output)
}
