extern crate clap;
extern crate toml;
extern crate walkdir;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::{fs, io, path};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    files: Vec<path::PathBuf>,
    /// Specifies the path to the config file
    #[clap(short, long, default_value = ".juliafmt.toml")]
    config: path::PathBuf,
    /// Modifies the input files inplace
    #[clap(short, long)]
    inplace: bool,
    /// Prints the concrete syntax tree
    #[clap(long)]
    cst: bool,
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
//         } else if args.cst {
//             juliafmt::cst(&input, &mut out).unwrap();
//         } else {
//             juliafmt::format(&input, &config, &mut out).unwrap_or_else(|e| panic!("{}", e))
//         }
//     } else {
//         // If we have multiple files and inplace is false, panic
//         if args.files.len() > 1 && !args.inplace {
//             return Err(-1);
//         }
//         // Read files from file vec and format
//         for file in args.files {
//             let mut f = File::open(file).unwrap();
//             f.read_to_string(&mut input).unwrap();
//             if args.inplace {
//                 juliafmt::format(&input, &config, &mut f).unwrap_or_else(|e| panic!("{}", e))
//             } else {
//                 juliafmt::format(&input, &config, &mut out).unwrap_or_else(|e| panic!("{}", e))
//             }
//         }
//     }
//     Ok(())
// }

fn main() {
    let mut errors = 0u64;
    let mut tokens = 0u64;
    let mut files = 0u64;
    for entry in WalkDir::new("/usr/share/julia")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() != None)
        .filter(|e| e.path().extension().unwrap() == "jl")
        .filter(|e| !e.path().is_dir())
    {
        files += 1;
        let (t, e) = juliafmt::lex_until_error(fs::read_to_string(entry.path()).unwrap());
        tokens += t;
        errors += e;
    }
    println!(
        "Lexed {} files, with a total of {} tokens and {} errors",
        files, tokens, errors
    );
}
