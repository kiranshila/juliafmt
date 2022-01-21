extern crate clap;
extern crate toml;

use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::{fs, io, path};

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    files: Vec<path::PathBuf>,
    #[clap(short, long, default_value = ".juliafmt.toml")]
    config: path::PathBuf,
    #[clap(short, long)]
    inplace: bool,
}

fn main() -> Result<(), i8> {
    let args = Args::parse();
    let config: juliafmt::Config = if let Ok(string) = fs::read_to_string(&args.config) {
        toml::from_str(&string).expect("Invalid config file")
    } else {
        Default::default()
    };
    if args.files.is_empty() {
        // Read stdin to EOF
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer).unwrap();
        let in_str = String::from_utf8(buffer).unwrap();
        // Format code and print to stdout
        println!("{}", juliafmt::format(&in_str, &config).unwrap());
    } else {
        // If we have multiple files and inplace is false, panic
        if args.files.len() > 1 && !args.inplace {
            return Err(-1);
        }
        // Read files from file vec and format
        for file in args.files {
            let mut f = File::open(file).unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            let formatted = juliafmt::format(&contents, &config).unwrap();
            if args.inplace {
                f.write_all(formatted.as_bytes()).unwrap();
            } else {
                println!("{}", formatted);
            }
        }
    }
    Ok(())
}
