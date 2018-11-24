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
        ).get_matches();
    if let Some(files) = matches.values_of("files") {
        for file in files {
            match parse_file(&file, false) {
                Ok(output) => {
                    if matches.is_present("ast") {
                        println!("{}", output);
                    } else {
                        let mut formatter = Formatter::new(&output);
                        println!("{}", formatter.format());
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}
