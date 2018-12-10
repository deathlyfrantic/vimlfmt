#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
extern crate regex;

pub use node::Node;
use regex::Regex;
use std::fmt;

mod command;
mod exarg;
mod modifier;
mod node;
mod parser;
mod reader;
mod token;

pub(crate) const EOF: char = '\x04';
pub(crate) const EOL: char = '\n';

pub fn parse_lines(lines: &[&str]) -> Result<node::Node, ParseError> {
    let reader = reader::Reader::from_lines(lines);
    let mut parser = parser::Parser::new(&reader);
    parser.parse()
}

pub fn parse_file(path: &str) -> Result<node::Node, ParseError> {
    let reader = reader::Reader::from_file(path)?;
    let mut parser = parser::Parser::new(&reader);
    parser.parse()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    cursor: usize,
    line: usize,
    col: usize,
}

impl Position {
    pub fn new(cursor: usize, line: usize, col: usize) -> Position {
        Position { cursor, line, col }
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

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError {
            msg: format!("{}", err),
            pos: Position::empty(),
        }
    }
}

pub(crate) trait CharClassification {
    fn is_word(&self) -> bool;
    fn is_word1(&self) -> bool;
    fn is_white(&self) -> bool;
    fn is_name(&self) -> bool;
}

impl CharClassification for char {
    fn is_word(&self) -> bool {
        self.is_ascii_alphanumeric() || *self == '_'
    }

    fn is_word1(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }

    fn is_white(&self) -> bool {
        *self == ' ' || *self == '\t'
    }

    fn is_name(&self) -> bool {
        self.is_ascii_alphanumeric() || ['_', ':', '#'].contains(&self)
    }
}

fn isargname(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[A-Za-z_][0-9A-Za-z_]*$").unwrap();
    }
    RE.is_match(s)
}

fn isvarname(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("^[vgslabwt]:$|^([vgslabwt]:)?[A-Za-z_][0-9A-Za-z_#]*$").unwrap();
    }
    RE.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isargname() {
        assert!(isargname("_foo1"));
        assert!(isargname("Foo2"));
        assert!(isargname("foo_3"));
        assert!(!isargname("2foo"));
    }

    #[test]
    fn test_isvarname() {
        assert!(isvarname("g:"));
        assert!(isvarname("v:Foo_123#bar"));
        assert!(!isvarname("x:foo"));
        assert!(!isvarname("fo|o"));
    }
}
