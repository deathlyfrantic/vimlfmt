pub use crate::node::{BinaryOpKind, Node, UnaryOpKind};
use lazy_static::lazy_static;
use regex::Regex;

mod command;
mod exarg;
mod modifier;
mod node;
mod parser;
mod reader;
mod token;

pub(crate) const EOF: char = '\x04';
pub(crate) const EOL: char = '\n';

/// Parse a list of lines, returning a Node upon success, or a [ParseError](struct.ParseError.html)
/// upon failure. The node will be a [TopLevel](enum.Node.html#variant.TopLevel) variant.
pub fn parse_lines(lines: &[&str]) -> Result<node::Node, ParseError> {
    let reader = reader::Reader::from_lines(lines);
    let mut parser = parser::Parser::new(&reader);
    parser.parse()
}

/// Parse a file, returning a Node upon success, or a [ParseError](struct.ParseError.html)
/// upon failure. The node will be a [TopLevel](enum.Node.html#variant.TopLevel) variant.
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
    #[cfg(test)]
    pub(crate) fn new(cursor: usize, line: usize, col: usize) -> Position {
        Position { cursor, line, col }
    }

    pub(crate) fn empty() -> Position {
        Position {
            cursor: 0,
            line: 0,
            col: 0,
        }
    }

    /// The column of a given position.
    pub fn column(&self) -> usize {
        self.col
    }

    /// The line of a given position.
    pub fn line(&self) -> usize {
        self.line
    }
}

/// Any error encountered when parsing VimL.
#[derive(Debug, PartialEq)]
pub struct ParseError {
    msg: String,
    /// The position of the error.
    pub pos: Position,
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    fn test_is_word() {
        assert!('_'.is_word());
        assert!('a'.is_word());
        assert!('A'.is_word());
        assert!('0'.is_word());
        assert!(!':'.is_word());
    }

    #[test]
    fn test_is_word1() {
        assert!('_'.is_word1());
        assert!('a'.is_word1());
        assert!('A'.is_word1());
        assert!(!'0'.is_word1());
        assert!(!':'.is_word1());
    }

    #[test]
    fn test_is_white() {
        assert!(' '.is_white());
        assert!('\t'.is_white());
        assert!(!'A'.is_white());
    }

    #[test]
    fn test_is_name() {
        assert!('_'.is_name());
        assert!('a'.is_name());
        assert!('A'.is_name());
        assert!('0'.is_name());
        assert!(':'.is_name());
        assert!('#'.is_name());
        assert!(!'!'.is_name());
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
