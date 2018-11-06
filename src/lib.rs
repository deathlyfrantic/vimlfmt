extern crate regex;

use regex::Regex;
use std::fmt;

mod command;
mod exarg;
mod modifier;
mod node;
mod parser;
mod reader;
mod token;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    cursor: usize,
    line: usize,
    col: usize,
}

impl Position {
    pub fn new(cursor: usize, line: usize, col: usize) -> Position {
        Position {
            cursor: cursor,
            line: line,
            col: col,
        }
    }

    pub fn empty() -> Position {
        Position {
            cursor: 0,
            line: 0,
            col: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    msg: String,
    pos: Position,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, col {}: {}",
            self.pos.line, self.pos.col, self.msg
        )
    }
}

fn isalpha(s: &str) -> bool {
    Regex::new("^[A-Za-z]$").unwrap().is_match(s)
}

fn isalnum(s: &str) -> bool {
    Regex::new("^[0-9A-Za-z]$").unwrap().is_match(s)
}

fn isdigit(s: &str) -> bool {
    Regex::new("^[0-9]$").unwrap().is_match(s)
}

fn isodigit(s: &str) -> bool {
    Regex::new("^[0-7]$").unwrap().is_match(s)
}

fn isxdigit(s: &str) -> bool {
    Regex::new("^[0-9A-Fa-f]$").unwrap().is_match(s)
}

fn iswordc(s: &str) -> bool {
    Regex::new("^[0-9A-Za-z_]$").unwrap().is_match(s)
}

fn iswordc1(s: &str) -> bool {
    Regex::new("^[A-Za-z_]$").unwrap().is_match(s)
}

fn iswhite(s: &str) -> bool {
    Regex::new("^[ \\t]$").unwrap().is_match(s)
}

fn isnamec(s: &str) -> bool {
    Regex::new("^[0-9A-Za-z_:#]$").unwrap().is_match(s)
}

fn isnamec1(s: &str) -> bool {
    iswordc1(s)
}

fn isargname(s: &str) -> bool {
    Regex::new("^[A-Za-z_][0-9A-Za-z_]*$").unwrap().is_match(s)
}

fn isvarname(s: &str) -> bool {
    Regex::new("^[vgslabwt]:$|^([vgslabwt]:)?[A-Za-z_][0-9A-Za-z_#]*$")
        .unwrap()
        .is_match(s)
}

fn isidc(s: &str) -> bool {
    iswordc(s)
}

fn isupper(s: &str) -> bool {
    Regex::new("^[A-Z]$").unwrap().is_match(s)
}

fn islower(s: &str) -> bool {
    Regex::new("^[a-z]$").unwrap().is_match(s)
}
