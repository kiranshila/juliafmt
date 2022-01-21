extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

use pest::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[grammar = "parser/julia.pest"]
pub struct JuliaParser;

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

pub fn format(input: &str, config: &Config) -> Result<String, pest::error::Error<Rule>> {
    let ast = JuliaParser::parse(Rule::program, input)?;
    Ok(format!("{:?}", ast))
}
