use super::Position;
use std::cell::RefCell;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Reader {
    buf: Vec<char>,
    pos: Vec<(usize, usize)>,
    cursor: RefCell<usize>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            buf: vec![],
            pos: vec![],
            cursor: RefCell::new(0),
        }
    }

    pub fn tell(&self) -> usize {
        *self.cursor.borrow()
    }

    pub fn from_lines(lines: &[&str]) -> Reader {
        let mut reader = Reader::new();
        reader.set_lines(lines);
        reader
    }

    pub fn from_file(path: &str) -> std::io::Result<Reader> {
        let mut reader = Reader::new();
        reader.read_file(path)?;
        Ok(reader)
    }

    fn set_lines(&mut self, lines: &[&str]) {
        let mut col;
        let mut lnum = 0;
        while lnum < lines.len() {
            col = 0;
            for c in lines[lnum].chars() {
                self.buf.push(c);
                self.pos.push((lnum + 1, col + 1));
                col += 1;
            }
            while lnum + 1 < lines.len() && lines[lnum + 1].trim_start().starts_with("\\") {
                let line = lines[lnum + 1];
                let trimmed = line.trim_start();
                col = line.len() - trimmed.len() + 1;
                for c in trimmed[1..].chars() {
                    self.buf.push(c);
                    self.pos.push((lnum + 2, col + 1));
                    col += 1;
                }
                lnum += 1;
            }
            self.buf.push('\n');
            self.pos.push((lnum + 1, col + 1));
            lnum += 1;
        }
        self.pos.push((lnum + 1, 0)); // eof
    }

    fn read_file(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.set_lines(&content.lines().collect::<Vec<&str>>());
        Ok(())
    }

    pub fn seek_set(&self, i: usize) {
        *self.cursor.borrow_mut() = i;
    }

    pub fn seek_cur(&self, i: usize) {
        *self.cursor.borrow_mut() += i;
    }

    pub fn seek_end(&self) {
        *self.cursor.borrow_mut() = self.buf.len();
    }

    pub fn peek_ahead(&self, i: usize) -> String {
        let cursor = *self.cursor.borrow();
        if cursor + i < self.buf.len() {
            self.buf[cursor + i].to_string()
        } else {
            "<EOF>".to_string()
        }
    }

    pub fn peek(&self) -> String {
        self.peek_ahead(0)
    }

    pub fn peekn(&self, n: usize) -> String {
        let cursor = *self.cursor.borrow();
        let mut i = 0;
        while cursor + i < self.buf.len() && self.buf[cursor + i] != '\n' {
            i += 1;
            if i >= n {
                break;
            }
        }
        self.buf[cursor..cursor + i].iter().collect::<String>()
    }

    pub fn peek_line(&self) -> String {
        let cursor = *self.cursor.borrow();
        let mut i = 0;
        while cursor + i < self.buf.len() && self.buf[cursor + i] != '\n' {
            i += 1;
        }
        self.buf[cursor..cursor + i].iter().collect::<String>()
    }

    pub fn get(&self) -> String {
        if *self.cursor.borrow() >= self.buf.len() {
            return "<EOF>".to_string();
        }
        *self.cursor.borrow_mut() += 1;
        self.buf[*self.cursor.borrow() - 1].to_string()
    }

    pub fn getn(&self, n: usize) -> String {
        let cursor = *self.cursor.borrow();
        let start = cursor;
        let mut i = 0;
        while cursor + i < self.buf.len() && self.buf[cursor + i] != '\n' {
            i += 1;
            if i >= n {
                break;
            }
        }
        *self.cursor.borrow_mut() += i;
        self.buf[start..cursor + i].iter().collect::<String>()
    }

    pub fn get_line(&self) -> String {
        let mut cursor = *self.cursor.borrow();
        let start = cursor;
        while cursor < self.buf.len() && self.buf[cursor] != '\n' {
            cursor += 1;
        }
        *self.cursor.borrow_mut() = cursor;
        let rv = self.buf[start..cursor].iter().collect::<String>();
        rv
    }

    pub fn getstr(&self, begin: Position, end: Position) -> String {
        self.buf[begin.cursor..min(end.cursor, self.buf.len())]
            .iter()
            .collect::<String>()
    }

    pub fn getpos(&self) -> Position {
        let cursor = *self.cursor.borrow();
        Position {
            cursor,
            line: self.pos[cursor].0,
            col: self.pos[cursor].1,
        }
    }

    pub fn setpos(&self, pos: Position) {
        *self.cursor.borrow_mut() = pos.cursor;
    }

    fn read_base<F>(&self, func: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut cursor = *self.cursor.borrow();
        let start = cursor;
        while cursor < self.buf.len() {
            if !func(self.buf[cursor]) {
                break;
            }
            cursor += 1;
        }
        *self.cursor.borrow_mut() = cursor;
        self.buf[start..cursor].iter().collect::<String>()
    }

    pub fn read_alpha(&self) -> String {
        self.read_base(|c| c.is_alphabetic())
    }

    pub fn read_alnum(&self) -> String {
        self.read_base(|c| c.is_ascii_alphanumeric())
    }

    pub fn read_digit(&self) -> String {
        self.read_base(|c| c.is_digit(10))
    }

    pub fn read_hex_digit(&self) -> String {
        self.read_base(|c| c.is_digit(16))
    }

    pub fn read_bin_digit(&self) -> String {
        self.read_base(|c| c.is_digit(2))
    }

    pub fn read_integer(&self) -> String {
        let mut rv = String::new();
        let c = self.peek();
        if c == "-" || c == "+" {
            rv.push_str(&self.get());
        }
        rv.push_str(&self.read_digit());
        rv
    }

    pub fn read_word(&self) -> String {
        self.read_base(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    pub fn read_white(&self) -> String {
        self.read_base(|c| c != '\n' && c.is_whitespace())
    }

    pub fn read_nonwhite(&self) -> String {
        self.read_base(|c| !c.is_whitespace())
    }

    pub fn read_name(&self) -> String {
        self.read_base(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':' || c == '#')
    }

    pub fn skip_white(&self) {
        self.read_white();
    }

    pub fn skip_white_and_colon(&self) {
        self.read_base(|c| (c != '\n' && c.is_whitespace()) || c == ':');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_ahead() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.peek_ahead(0), "f");
        assert_eq!(&reader.peek_ahead(1), "o");
        assert_eq!(reader.tell(), 0);
    }

    #[test]
    fn test_peek() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.peek(), "f");
        assert_eq!(reader.tell(), 0);
        *reader.cursor.borrow_mut() = reader.buf.len();
        assert_eq!(&reader.peek(), "<EOF>");
    }

    #[test]
    fn test_peekn() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.peekn(1), "f");
        assert_eq!(&reader.peekn(2), "fo");
        assert_eq!(reader.tell(), 0);
        *reader.cursor.borrow_mut() = 1;
        assert_eq!(&reader.peekn(5), "oo");
        assert_eq!(reader.tell(), 1);
        reader.getn(2);
        assert_eq!(&reader.peekn(1), "");
    }

    #[test]
    fn test_peek_line() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.peek_line(), "foo");
        assert_eq!(reader.tell(), 0);
    }

    #[test]
    fn test_get() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.get(), "f");
        assert_eq!(reader.tell(), 1);
        *reader.cursor.borrow_mut() = reader.buf.len();
        assert_eq!(&reader.get(), "<EOF>");
    }

    #[test]
    fn test_getn() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.getn(2), "fo");
        assert_eq!(reader.tell(), 2);
        assert_eq!(&reader.getn(5), "o");
        assert_eq!(reader.tell(), 3);
        assert_eq!(&reader.getn(1), "");
        assert_eq!(reader.tell(), 3);
    }

    #[test]
    fn test_get_line() {
        let reader = Reader::from_lines(&["foo", "bar"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.get_line(), "foo");
        assert_eq!(reader.tell(), 3);
        assert_eq!(&reader.peek(), "\n");
    }

    #[test]
    fn test_getstr() {
        let reader = Reader::from_lines(&["foobarbazquux"]);
        assert_eq!(
            reader.getstr(Position::new(1, 0, 0), Position::new(6, 0, 0)),
            "oobar"
        );
    }

    #[test]
    fn test_read_alpha() {
        let reader = Reader::from_lines(&["Foobar123"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_alpha(), "Foobar");
        assert_eq!(reader.tell(), 6);
    }

    #[test]
    fn test_read_alnum() {
        let reader = Reader::from_lines(&["Foobar123"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_alnum(), "Foobar123");
        assert_eq!(reader.tell(), 9);
    }

    #[test]
    fn test_read_digit() {
        let reader = Reader::from_lines(&["123 a1f 078 011"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_digit(), "123");
        assert_eq!(reader.tell(), 3);
        reader.get();
        assert_eq!(&reader.read_hex_digit(), "a1f");
        assert_eq!(reader.tell(), 7);
        reader.get();
        assert_eq!(&reader.read_digit(), "078");
        assert_eq!(reader.tell(), 11);
        reader.get();
        assert_eq!(&reader.read_bin_digit(), "011");
        assert_eq!(reader.tell(), 15);
    }

    #[test]
    fn test_read_integer() {
        let reader = Reader::from_lines(&["+123 -456"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_integer(), "+123");
        assert_eq!(reader.tell(), 4);
        reader.get();
        assert_eq!(&reader.read_integer(), "-456");
        assert_eq!(reader.tell(), 9);
    }

    #[test]
    fn test_read_word() {
        let reader = Reader::from_lines(&["Abc_Def123|"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_word(), "Abc_Def123");
        assert_eq!(reader.tell(), 10);
    }

    #[test]
    fn test_read_white() {
        let reader = Reader::from_lines(&[" 	  x"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_white(), " 	  ");
        assert_eq!(reader.tell(), 4);
    }

    #[test]
    fn test_read_nonwhite() {
        let reader = Reader::from_lines(&["abc "]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_nonwhite(), "abc");
        assert_eq!(reader.tell(), 3);
    }

    #[test]
    fn test_read_name() {
        let reader = Reader::from_lines(&["b:abc#foo_bar()"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.read_name(), "b:abc#foo_bar");
        assert_eq!(reader.tell(), 13);
    }

    #[test]
    fn test_skip_white() {
        let reader = Reader::from_lines(&["g ", ": foo"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.get(), "g");
        reader.skip_white();
        assert_eq!(reader.tell(), 2);
        assert_eq!(&reader.peek(), "\n");
    }

    #[test]
    fn test_skip_white_and_colon() {
        let reader = Reader::from_lines(&["g  :	foo"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.get(), "g");
        reader.skip_white_and_colon();
        assert_eq!(reader.tell(), 5);
        assert_eq!(&reader.get(), "f");
        let reader = Reader::from_lines(&["1", "d"]);
        assert_eq!(reader.tell(), 0);
        assert_eq!(&reader.get(), "1");
        assert_eq!(reader.tell(), 1);
        reader.skip_white_and_colon();
        assert_eq!(reader.tell(), 1);
    }

    #[test]
    fn test_set_lines() {
        let vim = r#"function! s:foo() abort
    echoerr "this is a continued line"
    let foo = {
      \ 'bar',
      \ 'baz',
      \ }
endfunction"#;
        let lines = vim.lines().collect::<Vec<&str>>();
        let reader = Reader::from_lines(&lines);
        println!("{:?}", reader);
        println!("reader buf length -> {}", reader.buf.len());
        println!("reader pos length -> {}", reader.pos.len());
    }
}
