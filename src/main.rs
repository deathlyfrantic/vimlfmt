extern crate clap;
extern crate viml_parser;

mod formatter;

use clap::{App, Arg};
use formatter::Formatter;
use viml_parser::parse_file;

fn main() {
    let matches = App::new("vimfmt")
        .arg(
            Arg::with_name("ast")
                .short("A")
                .long("ast")
                .help("Output AST instead of formatted code"),
        ).arg(
            Arg::with_name("files")
                .required(true)
                .min_values(1)
                .help("File(s) to parse"),
        ).arg(
            Arg::with_name("neovim")
                .short("n")
                .long("neovim")
                .help("Parse as Neovim"),
        ).get_matches();
    let use_neovim = matches.is_present("neovim");
    if let Some(files) = matches.values_of("files") {
        for file in files {
            match parse_file(&file, use_neovim) {
                Ok(output) => {
                    if matches.is_present("ast") {
                        println!("{}", output);
                    } else {
                        let mut formatter = Formatter::new();
                        println!("{}", formatter.format(&output));
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}
