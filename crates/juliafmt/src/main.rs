use clap::Parser as CParser;
use lexer::Lexer;
use parser::parse;
use std::io::{self, Read, Write};
use std::str::from_utf8;

#[derive(CParser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    /// Prints the lossless abstract syntax tree
    #[clap(long)]
    ast: bool,
    /// Prints the lexing output
    #[clap(long)]
    lex: bool,
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let args = Args::parse();
    let mut stdout = io::stdout();
    let mut input = Vec::new();

    let prompt = if args.ast { "parser> " } else { "lexer> " };

    loop {
        write!(stdout, "{}", prompt)?;
        stdout.flush()?;

        stdin.read_to_end(&mut input)?;

        // Create string representation
        let in_str = from_utf8(&input).unwrap();

        if args.ast {
            let parse = parse(in_str);
            println!("{}", parse.debug_tree());
        } else {
            let lex: Vec<_> = Lexer::new(in_str).inner.spanned().collect();
            println!("\n{:?}", lex);
        }

        input.clear();
    }
}
