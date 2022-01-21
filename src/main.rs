extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{fs, path};

use clap::Parser as ClapParser;
use pest::Parser as PestParser;

#[derive(ClapParser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    #[clap(short, long)]
    path: path::PathBuf,
}

#[derive(Parser)]
#[grammar = "parser/julia.pest"]
struct JuliaParser;

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.path).unwrap();
    let result = JuliaParser::parse(Rule::program, &contents);
    println!("{:?}", result.unwrap())
}
