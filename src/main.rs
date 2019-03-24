mod formatter;

use crate::formatter::Formatter;
use clap::{App, Arg};
use std::io::{self, BufRead};
use viml_parser::parse_lines;

fn main() {
    let matches = App::new("vimfmt")
        .version("0.1.0")
        .arg(
            Arg::with_name("ast")
                .long("ast")
                .help("Output AST instead of formatted code"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Output formatted Rust debug output (using '{:#?}')"),
        )
        .get_matches();
    let mut formatter = Formatter::new();
    let lines: Vec<String> = io::stdin().lock().lines().filter_map(|l| l.ok()).collect();
    match parse_lines(
        lines
            .iter()
            .map(|l| l.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    ) {
        Ok(output) => {
            if matches.is_present("debug") {
                println!("{:#?}", output);
            } else if matches.is_present("ast") {
                println!("{}", output);
            } else {
                match formatter.format(&output) {
                    Ok(o) => println!("{}", o),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
