extern crate viml_parser;

use std::env;
use viml_parser::parse_file;

fn main() {
    let mut args = env::args().into_iter().collect::<Vec<String>>();
    args.remove(0);
    for arg in args {
        match parse_file(&arg, false) {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("{}", e),
        }
    }
}
