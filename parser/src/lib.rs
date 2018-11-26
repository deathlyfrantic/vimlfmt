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

pub fn parse_lines(lines: &[&str], neovim: bool) -> Result<node::Node, ParseError> {
    let reader = reader::Reader::from_lines(lines);
    let mut parser = parser::Parser::new(&reader, neovim);
    parser.parse()
}

pub fn parse_file(path: &str, neovim: bool) -> Result<node::Node, ParseError> {
    let reader = reader::Reader::from_file(path)?;
    let mut parser = parser::Parser::new(&reader, neovim);
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

fn str_is<F>(s: &str, func: F) -> bool
where
    F: Fn(char) -> bool,
{
    for c in s.chars() {
        if !func(c) {
            return false;
        }
    }
    true
}

fn isdigit(s: &str) -> bool {
    str_is(s, |c| c.is_ascii_digit())
}

fn isxdigit(s: &str) -> bool {
    str_is(s, |c| c.is_ascii_hexdigit())
}

fn iswordc(s: &str) -> bool {
    str_is(s, |c| c.is_ascii_alphanumeric() || c == '_')
}

fn iswordc1(s: &str) -> bool {
    str_is(s, |c| c.is_ascii_alphabetic() || c == '_')
}

fn iswhite(s: &str) -> bool {
    str_is(s, |c| c == '\t' || c == ' ')
}

fn isnamec(s: &str) -> bool {
    str_is(s, |c| {
        c.is_ascii_alphanumeric() || c == '_' || c == ':' || c == '#'
    })
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
    fn test_isdigit() {
        assert!(isdigit("0123456789"));
        assert!(!isdigit("abc"));
    }

    #[test]
    fn test_isxdigit() {
        assert!(isxdigit("0123456789ABCDEFabcdef"));
        assert!(!isxdigit("xqz"));
    }

    #[test]
    fn test_iswordc() {
        assert!(iswordc("Abc_123"));
        assert!(!iswordc("*@"));
    }

    #[test]
    fn test_iswordc1() {
        assert!(iswordc1("Abc_foo"));
        assert!(!iswordc1("Abc_123"));
    }

    #[test]
    fn test_iswhite() {
        assert!(iswhite(" \t"));
        assert!(!iswhite("\nX"));
    }

    #[test]
    fn test_isnamec() {
        assert!(isnamec("Abc_123:#"));
        assert!(!isnamec("*@"));
    }

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
