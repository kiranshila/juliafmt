extern crate clap;
extern crate pest;
extern crate pest_derive;

use std::{fs, path};

#[derive(clap::Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    #[clap(short, long)]
    path: path::PathBuf,
}

#[derive(pest_derive::Parser)]
#[grammar = "parser/julia.pest"]
pub struct JuliaParser;

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.path).unwrap();
    let result = JuliaParser::parse(Rule::program, &contents);
    println!("{:?}", args.path)
}
