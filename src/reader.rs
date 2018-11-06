use super::Position;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Reader {
    buf: Vec<char>,
    pos: Vec<(usize, usize)>,
    cursor: usize,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            buf: vec![],
            pos: vec![],
            cursor: 0,
        }
    }

    pub fn tell(&self) -> usize {
        self.cursor
    }

    pub fn from_lines(lines: Vec<&str>) -> Reader {
        let mut reader = Reader::new();
        reader.set_lines(lines);
        reader
    }

    pub fn from_file(path: &str) -> std::io::Result<Reader> {
        let mut reader = Reader::new();
        reader.read_file(path)?;
        Ok(reader)
    }

    fn set_lines(&mut self, lines: Vec<&str>) {
        let mut col = 0;
        let mut lnum = 0;
        while lnum < lines.len() {
            col = 0;
            for c in lines[lnum].chars() {
                self.buf.push(c);
                self.pos.push((lnum + 1, col + 1));
                col += 1;
            }
            while lnum + 1 < lines.len() && lines[lnum + 1].trim_start().starts_with("\\") {
                col = 0;
                for c in lines[lnum + 1].trim_start()[1..].chars() {
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
        self.set_lines(content.lines().collect::<Vec<&str>>());
        Ok(())
    }

    pub fn seek_set(&mut self, i: usize) {
        self.cursor = i;
    }

    pub fn seek_cur(&mut self, i: usize) {
        self.cursor += i;
    }

    pub fn seek_end(&mut self) {
        self.cursor = self.buf.len();
    }

    pub fn peek_ahead(&self, i: usize) -> String {
        if self.cursor + i < self.buf.len() {
            self.buf[self.cursor + i].to_string()
        } else {
            "<EOF>".to_string()
        }
    }

    pub fn peek(&self) -> String {
        self.peek_ahead(0)
    }

    pub fn peekn(&self, n: usize) -> String {
        let mut i = 0;
        while self.cursor + i < self.buf.len() && self.buf[self.cursor + i] != '\n' {
            i += 1;
            if i >= n {
                break;
            }
        }
        self.buf[self.cursor..self.cursor + i]
            .iter()
            .collect::<String>()
    }

    pub fn peek_line(&self) -> String {
        let mut i = 0;
        while self.buf[self.cursor + i] != '\n' && self.cursor + i < self.buf.len() {
            i += 1;
        }
        self.buf[self.cursor..self.cursor + i]
            .iter()
            .collect::<String>()
    }

    pub fn get(&mut self) -> String {
        if self.cursor >= self.buf.len() {
            return "<EOF>".to_string();
        }
        self.cursor += 1;
        self.buf[self.cursor - 1].to_string()
    }

    pub fn getn(&mut self, n: usize) -> String {
        let start = self.cursor;
        let mut i = 0;
        while self.cursor + i < self.buf.len() && self.buf[self.cursor + i] != '\n' {
            i += 1;
            if i >= n {
                break;
            }
        }
        self.cursor += i;
        self.buf[start..self.cursor].iter().collect::<String>()
    }

    pub fn get_line(&mut self) -> String {
        let start = self.cursor;
        while self.buf[self.cursor] != '\n' && self.cursor < self.buf.len() {
            self.cursor += 1;
        }
        let rv = self.buf[start..self.cursor].iter().collect::<String>();
        rv
    }

    pub fn getstr(&mut self, begin: Position, end: Position) -> String {
        self.buf[begin.cursor..min(end.cursor, self.buf.len())]
            .iter()
            .collect::<String>()
    }

    pub fn getpos(&self) -> Position {
        Position {
            cursor: self.cursor,
            line: self.pos[self.cursor].0,
            col: self.pos[self.cursor].1,
        }
    }

    pub fn setpos(&mut self, pos: Position) {
        self.cursor = pos.cursor;
    }

    fn read_base<F>(&mut self, func: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.cursor;
        while self.cursor < self.buf.len() {
            if !func(self.buf[self.cursor]) {
                break;
            }
            self.cursor += 1;
        }
        self.buf[start..self.cursor].iter().collect::<String>()
    }

    pub fn read_alpha(&mut self) -> String {
        self.read_base(|c| c.is_alphabetic())
    }

    pub fn read_alnum(&mut self) -> String {
        self.read_base(|c| c.is_ascii_alphanumeric())
    }

    pub fn read_digit(&mut self) -> String {
        self.read_base(|c| c.is_digit(10))
    }

    pub fn read_hex_digit(&mut self) -> String {
        self.read_base(|c| c.is_digit(16))
    }

    pub fn read_oct_digit(&mut self) -> String {
        self.read_base(|c| c.is_digit(8))
    }

    pub fn read_bin_digit(&mut self) -> String {
        self.read_base(|c| c.is_digit(2))
    }

    pub fn read_integer(&mut self) -> String {
        let mut rv = String::new();
        let c = self.peek();
        if c == "-" || c == "+" {
            rv.push_str(&self.get());
        }
        rv.push_str(&self.read_digit());
        rv
    }

    pub fn read_word(&mut self) -> String {
        self.read_base(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    pub fn read_white(&mut self) -> String {
        self.read_base(|c| c != '\n' && c.is_whitespace())
    }

    pub fn read_nonwhite(&mut self) -> String {
        self.read_base(|c| c == '\n' || !c.is_whitespace())
    }

    pub fn read_name(&mut self) -> String {
        self.read_base(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':' || c == '#')
    }

    pub fn skip_white(&mut self) {
        self.read_white();
    }

    pub fn skip_white_and_colon(&mut self) {
        self.read_base(|c| c.is_whitespace() || c == ':');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_ahead() {
        let reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.peek_ahead(0), "f");
        assert_eq!(&reader.peek_ahead(1), "o");
        assert_eq!(reader.cursor, 0);
    }

    #[test]
    fn test_peek() {
        let mut reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.peek(), "f");
        assert_eq!(reader.cursor, 0);
        reader.cursor = reader.buf.len();
        assert_eq!(&reader.peek(), "<EOF>");
    }

    #[test]
    fn test_peekn() {
        let mut reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.peekn(1), "f");
        assert_eq!(&reader.peekn(2), "fo");
        assert_eq!(reader.cursor, 0);
        reader.cursor = 1;
        assert_eq!(&reader.peekn(5), "oo");
        assert_eq!(reader.cursor, 1);
        reader.getn(2);
        assert_eq!(&reader.peekn(1), "");
    }

    #[test]
    fn test_peek_line() {
        let mut reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.peek_line(), "foo");
        assert_eq!(reader.cursor, 0);
    }

    #[test]
    fn test_get() {
        let mut reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.get(), "f");
        assert_eq!(reader.cursor, 1);
        reader.cursor = reader.buf.len();
        assert_eq!(&reader.get(), "<EOF>");
    }

    #[test]
    fn test_getn() {
        let mut reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.getn(2), "fo");
        assert_eq!(reader.cursor, 2);
        assert_eq!(&reader.getn(5), "o");
        assert_eq!(reader.cursor, 3);
        assert_eq!(&reader.getn(1), "");
        assert_eq!(reader.cursor, 3);
    }

    #[test]
    fn test_get_line() {
        let mut reader = Reader::from_lines(vec!["foo", "bar"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.get_line(), "foo");
        assert_eq!(reader.cursor, 4);
        assert_eq!(&reader.peek(), "\n");
    }

    #[test]
    fn test_getstr() {
        let mut reader = Reader::from_lines(vec!["foobarbazquux"]);
        assert_eq!(
            reader.getstr(Position::new(1, 0, 0), Position::new(6, 0, 0)),
            "oobar"
        );
    }

    #[test]
    fn test_read_alpha() {
        let mut reader = Reader::from_lines(vec!["Foobar123"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_alpha(), "Foobar");
        assert_eq!(reader.cursor, 6);
    }

    #[test]
    fn test_read_alnum() {
        let mut reader = Reader::from_lines(vec!["Foobar123"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_alnum(), "Foobar123");
        assert_eq!(reader.cursor, 9);
    }

    #[test]
    fn test_read_digit() {
        let mut reader = Reader::from_lines(vec!["123 a1f 078 011"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_digit(), "123");
        assert_eq!(reader.cursor, 3);
        reader.get();
        assert_eq!(&reader.read_hex_digit(), "a1f");
        assert_eq!(reader.cursor, 7);
        reader.get();
        assert_eq!(&reader.read_digit(), "078");
        assert_eq!(reader.cursor, 11);
        reader.get();
        assert_eq!(&reader.read_bin_digit(), "011");
        assert_eq!(reader.cursor, 15);
    }

    #[test]
    fn test_read_integer() {
        let mut reader = Reader::from_lines(vec!["+123 -456"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_integer(), "+123");
        assert_eq!(reader.cursor, 4);
        reader.get();
        assert_eq!(&reader.read_integer(), "-456");
        assert_eq!(reader.cursor, 9);
    }

    #[test]
    fn test_read_word() {
        let mut reader = Reader::from_lines(vec!["Abc_Def123|"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_word(), "Abc_Def123");
        assert_eq!(reader.cursor, 10);
    }

    #[test]
    fn test_read_white() {
        let mut reader = Reader::from_lines(vec![" 	  x"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_white(), " 	  ");
        assert_eq!(reader.cursor, 4);
    }

    #[test]
    fn test_read_nonwhite() {
        let mut reader = Reader::from_lines(vec!["abc "]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_nonwhite(), "abc");
        assert_eq!(reader.cursor, 3);
    }

    #[test]
    fn test_read_name() {
        let mut reader = Reader::from_lines(vec!["b:abc#foo_bar()"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.read_name(), "b:abc#foo_bar");
        assert_eq!(reader.cursor, 13);
    }

    #[test]
    fn test_skip_white() {
        let mut reader = Reader::from_lines(vec!["g ", ": foo"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.get(), "g");
        reader.skip_white();
        assert_eq!(reader.cursor, 2);
        assert_eq!(&reader.peek(), "\n");
    }

    #[test]
    fn test_skip_white_and_colon() {
        let mut reader = Reader::from_lines(vec!["g  :	foo"]);
        assert_eq!(reader.cursor, 0);
        assert_eq!(&reader.get(), "g");
        reader.skip_white_and_colon();
        assert_eq!(reader.cursor, 5);
        assert_eq!(&reader.get(), "f");
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
        let reader = Reader::from_lines(lines);
        println!("{:?}", reader);
        println!("reader buf length -> {}", reader.buf.len());
        println!("reader pos length -> {}", reader.pos.len());
    }

    #[test]
    fn test_from_file() {
        let reader = Reader::from_file("auto-gutters.vim").unwrap();
        println!("{:?}", reader);
        println!("reader buf length -> {}", reader.buf.len());
        println!("reader pos length -> {}", reader.pos.len());
    }
}
