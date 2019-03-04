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
                .short("A")
                .long("ast")
                .help("Output AST instead of formatted code"),
        )
        .arg(
            Arg::with_name("indent")
                .short("i")
                .long("indent")
                .default_value("2")
                .help("Number of spaces to use for indentation; use \"tab\" for tabs"),
        )
        .arg(
            Arg::with_name("continuation")
                .short("c")
                .long("continuation")
                .default_value("3")
                .help("Number of indents to use for continued lines"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .default_value("80")
                .help("max length of formatted lines"),
        )
        .get_matches();
    let indent = matches.value_of("indent").unwrap();
    let indent = if indent.to_lowercase().starts_with("tab") {
        "\t".to_string()
    } else {
        " ".repeat(
            indent
                .parse::<usize>()
                .expect("indent must be a positive integer or \"tab\""),
        )
    };
    let continuation = matches
        .value_of("continuation")
        .unwrap()
        .parse::<usize>()
        .expect("continuation must be a positive integer");
    let length = matches
        .value_of("length")
        .unwrap()
        .parse::<usize>()
        .expect("length must be a positive integer");
    let mut formatter = Formatter::new(&indent, continuation, length);
    let lines: Vec<String> = io::stdin().lock().lines().filter_map(|l| l.ok()).collect();
    match parse_lines(
        lines
            .iter()
            .map(|l| l.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    ) {
        Ok(output) => {
            if matches.is_present("ast") {
                println!("{}", output);
            } else {
                println!("{}", formatter.format(&output));
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
