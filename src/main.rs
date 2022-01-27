pub mod expr;
pub mod formatter;
pub mod lexer;
pub mod parser;
pub mod syntax;

use clap::Parser as CParser;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};

#[derive(CParser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    // files: Vec<path::PathBuf>,
    // /// Specifies the path to the config file
    // #[clap(short, long, default_value = ".juliafmt.toml")]
    // config: path::PathBuf,
    // /// Modifies the input files inplace
    // #[clap(short, long)]
    // inplace: bool,
    /// Prints the lossless abstract syntax tree
    #[clap(long)]
    ast: bool,
    /// Prints the lexing output
    #[clap(long)]
    lex: bool,
}

// fn main() -> Result<(), i8> {
//     // Read config and args
//     let args = Args::parse();
//     let config: juliafmt::Config = if let Ok(string) = fs::read_to_string(&args.config) {
//         toml::from_str(&string).expect("Invalid config file")
//     } else {
//         Default::default()
//     };

//     // Create sources and sinks
//     let mut out = io::stdout();
//     let mut input = String::new();

//     // Grab data
//     if args.files.is_empty() {
//         // Read stdin to EOF
//         let mut buffer = Vec::new();
//         io::stdin().read_to_end(&mut buffer).unwrap();
//         input = String::from_utf8(buffer).unwrap();
//         // Either lex, parse, or format
//         // Lexing and parsing are debug-level and we can probably remove later
//         if args.lex {
//             juliafmt::lex(&input, &mut out).unwrap();
//         }
//     }
//     Ok(())
// }

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
            let parse = Parser::new(&input).parse();
            println!("{}", parse.debug_tree());
        } else {
            let lex: Vec<_> = Lexer::new(&input).inner.spanned().collect();
            println!("\n{:?}", lex);
        }

        input.clear();
    }
}
