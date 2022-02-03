use clap::Parser as CParser;
use lexer::Lexer;
use parser::parse;
use std::io::{self, Write};

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
    let stdin = io::stdin();
    let args = Args::parse();
    let mut stdout = io::stdout();
    let mut input = String::new();

    let prompt = if args.ast { "parser> " } else { "lexer> " };

    loop {
        write!(stdout, "{}", prompt)?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        if args.ast {
            let parse = parse(&input);
            println!("{}", parse.debug_tree());
        } else {
            let lex: Vec<_> = Lexer::new(&input).inner.spanned().collect();
            println!("\n{:?}", lex);
        }

        input.clear();
    }
}
