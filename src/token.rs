use super::{isdigit, isidc, isnamec1, iswhite, isxdigit, ParseError, Position};
use reader::Reader;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    EOF,
    EOL,
    Space,
    OrOr,
    AndAnd,
    EqEq,
    EqEqCI,
    EqEqCS,
    NotEq,
    NotEqCI,
    NotEqCS,
    GT,
    GTCI,
    GTCS,
    GTEq,
    GTEqCI,
    GTEqCS,
    LT,
    LTCI,
    LTCS,
    LTEq,
    LTEqCI,
    LTEqCS,
    Match,
    MatchCI,
    MatchCS,
    NoMatch,
    NoMatchCI,
    NoMatchCS,
    Is,
    IsCI,
    IsCS,
    IsNot,
    IsNotCI,
    IsNotCS,
    Plus,
    Minus,
    Dot,
    Star,
    Slash,
    Percent,
    Not,
    Question,
    Colon,
    POpen,
    PClose,
    SqOpen,
    SqClose,
    COpen,
    CClose,
    Comma,
    Number,
    SQuote,
    DQuote,
    Option,
    Identifier,
    Env,
    Reg,
    Eq,
    Or,
    Semicolon,
    Backtick,
    DotDotDot,
    Sharp,
    Arrow,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub pos: Position,
}

impl Token {
    pub fn new(kind: TokenKind, value: String, pos: Position) -> Token {
        Token {
            kind: kind,
            value: value,
            pos: pos,
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    reader: Rc<RefCell<Reader>>,
    // TODO: add cache here
}

impl Tokenizer {
    pub fn new(reader: Rc<RefCell<Reader>>) -> Tokenizer {
        Tokenizer { reader: reader }
    }

    pub fn peek(&mut self) -> Result<Token, ParseError> {
        let pos = self.reader.borrow().tell();
        let token = self.get();
        self.reader.borrow_mut().seek_set(pos);
        token
    }

    pub fn get(&mut self) -> Result<Token, ParseError> {
        self.reader.borrow_mut().skip_white();
        self.get2()
    }

    pub fn get2(&mut self) -> Result<Token, ParseError> {
        let c = self.reader.borrow().peek();
        let pos = self.reader.borrow().getpos();
        if c == "<EOF>" {
            return Ok(Token::new(TokenKind::EOF, c, pos));
        }
        if c == "\n" {
            self.reader.borrow_mut().get();
            return Ok(Token::new(TokenKind::EOL, c, pos));
        }
        if iswhite(&c) {
            // I don't think this ever happens (see skip_white() call in self.get())
            return Ok(Token::new(TokenKind::Space, c, pos));
        }
        if isdigit(&c) {
            let x = self.reader.borrow().peek_ahead(1);
            let n = self.reader.borrow().peek_ahead(2);
            if c == "0" && (x == "x" || x == "X") && isxdigit(&n) {
                let mut value = self.reader.borrow_mut().getn(3);
                value.push_str(&self.reader.borrow_mut().read_hex_digit());
                return Ok(Token::new(TokenKind::Number, value, pos));
            }
            if c == "0" && (x == "b" || x == "B") && (n == "0" || n == "1") {
                let mut value = self.reader.borrow_mut().getn(3);
                value.push_str(&self.reader.borrow_mut().read_bin_digit());
                return Ok(Token::new(TokenKind::Number, value, pos));
            }
            let mut value = self.reader.borrow_mut().read_digit();
            if self.reader.borrow().peek() == "." && isdigit(&self.reader.borrow().peek_ahead(1)) {
                value.push_str(&self.reader.borrow_mut().get());
                value.push_str(&self.reader.borrow_mut().read_digit());
                let e = self.reader.borrow().peek();
                let n = self.reader.borrow().peek_ahead(1);
                let n2 = self.reader.borrow().peek_ahead(2);
                if (e == "E" || e == "e")
                    && (isdigit(&n) || ((n == "-" || n == "+") && isdigit(&n2)))
                {
                    value.push_str(&self.reader.borrow_mut().getn(2));
                    value.push_str(&self.reader.borrow_mut().read_digit());
                }
            }
            return Ok(Token::new(TokenKind::Number, value, pos));
        }
        if c == "i"
            && self.reader.borrow().peek_ahead(1) == "s"
            && !isidc(&self.reader.borrow().peek_ahead(2))
        {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::IsCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::IsCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::Is,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(5) == "isnot" && !isidc(&self.reader.borrow().peek_ahead(5)) {
            if self.reader.borrow().peek_ahead(5) == "?" {
                return Ok(Token::new(
                    TokenKind::IsNotCI,
                    self.reader.borrow_mut().getn(6),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(5) == "#" {
                return Ok(Token::new(
                    TokenKind::IsNotCS,
                    self.reader.borrow_mut().getn(6),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::IsNot,
                self.reader.borrow_mut().getn(5),
                pos,
            ));
        }
        if isnamec1(&c) {
            return Ok(Token::new(
                TokenKind::Identifier,
                self.reader.borrow_mut().read_name(),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "||" {
            return Ok(Token::new(
                TokenKind::OrOr,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "&&" {
            return Ok(Token::new(
                TokenKind::AndAnd,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "==" {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::EqEqCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::EqEqCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::EqEq,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "!=" {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::NotEqCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::NotEqCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::NotEq,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == ">=" {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::GTEqCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::GTEqCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::GTEq,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "<=" {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::LTEqCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::LTEqCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::LTEq,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "=~" {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::MatchCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::MatchCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::Match,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        if self.reader.borrow().peekn(2) == "!~" {
            if self.reader.borrow().peek_ahead(2) == "?" {
                return Ok(Token::new(
                    TokenKind::NoMatchCI,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            if self.reader.borrow().peek_ahead(2) == "#" {
                return Ok(Token::new(
                    TokenKind::NoMatchCS,
                    self.reader.borrow_mut().getn(3),
                    pos,
                ));
            }
            return Ok(Token::new(
                TokenKind::NoMatch,
                self.reader.borrow_mut().getn(2),
                pos,
            ));
        }
        match c.as_str() {
            ">" => {
                let c = self.reader.borrow().peek_ahead(1);
                match c.as_str() {
                    "?" => Ok(Token::new(
                        TokenKind::GTCI,
                        self.reader.borrow_mut().getn(2),
                        pos,
                    )),
                    "#" => Ok(Token::new(
                        TokenKind::GTCS,
                        self.reader.borrow_mut().getn(2),
                        pos,
                    )),
                    _ => Ok(Token::new(
                        TokenKind::GT,
                        self.reader.borrow_mut().get(),
                        pos,
                    )),
                }
            }
            "<" => {
                let c = self.reader.borrow().peek_ahead(1);
                match c.as_str() {
                    "?" => Ok(Token::new(
                        TokenKind::LTCI,
                        self.reader.borrow_mut().getn(2),
                        pos,
                    )),
                    "#" => Ok(Token::new(
                        TokenKind::LTCS,
                        self.reader.borrow_mut().getn(2),
                        pos,
                    )),
                    _ => Ok(Token::new(
                        TokenKind::LT,
                        self.reader.borrow_mut().get(),
                        pos,
                    )),
                }
            }
            "+" => {
                return Ok(Token::new(
                    TokenKind::Plus,
                    self.reader.borrow_mut().get(),
                    pos,
                ));
            }
            "-" => {
                if self.reader.borrow().peek_ahead(1) == ">" {
                    return Ok(Token::new(
                        TokenKind::Arrow,
                        self.reader.borrow_mut().getn(2),
                        pos,
                    ));
                }
                Ok(Token::new(
                    TokenKind::Minus,
                    self.reader.borrow_mut().get(),
                    pos,
                ))
            }
            "." => {
                if self.reader.borrow().peekn(3) == "..." {
                    return Ok(Token::new(
                        TokenKind::DotDotDot,
                        self.reader.borrow_mut().getn(3),
                        pos,
                    ));
                }
                Ok(Token::new(
                    TokenKind::Dot,
                    self.reader.borrow_mut().get(),
                    pos,
                ))
            }
            "*" => Ok(Token::new(
                TokenKind::Star,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "/" => Ok(Token::new(
                TokenKind::Slash,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "%" => Ok(Token::new(
                TokenKind::Percent,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "!" => Ok(Token::new(
                TokenKind::Not,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "?" => Ok(Token::new(
                TokenKind::Question,
                self.reader.borrow_mut().get(),
                pos,
            )),
            ":" => Ok(Token::new(
                TokenKind::Colon,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "#" => Ok(Token::new(
                TokenKind::Sharp,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "(" => Ok(Token::new(
                TokenKind::POpen,
                self.reader.borrow_mut().get(),
                pos,
            )),
            ")" => Ok(Token::new(
                TokenKind::PClose,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "[" => Ok(Token::new(
                TokenKind::SqOpen,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "]" => Ok(Token::new(
                TokenKind::SqClose,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "{" => Ok(Token::new(
                TokenKind::COpen,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "}" => Ok(Token::new(
                TokenKind::CClose,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "," => Ok(Token::new(
                TokenKind::Comma,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "'" => Ok(Token::new(
                TokenKind::SQuote,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "\"" => Ok(Token::new(
                TokenKind::DQuote,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "$" => {
                let mut value = self.reader.borrow_mut().get();
                value.push_str(&self.reader.borrow_mut().read_word());
                Ok(Token::new(TokenKind::Env, value, pos))
            }
            "@" => Ok(Token::new(
                TokenKind::Reg,
                self.reader.borrow_mut().getn(2),
                pos,
            )),
            "&" => {
                let p = self.reader.borrow().peek_ahead(1);
                let mut value =
                    if (p == "g" || p == "l") && self.reader.borrow().peek_ahead(2) == ":" {
                        self.reader.borrow_mut().getn(3)
                    } else {
                        self.reader.borrow_mut().get()
                    };
                value.push_str(&self.reader.borrow_mut().read_word());
                Ok(Token::new(TokenKind::Option, value, pos))
            }
            "=" => Ok(Token::new(
                TokenKind::Eq,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "|" => Ok(Token::new(
                TokenKind::Or,
                self.reader.borrow_mut().get(),
                pos,
            )),
            ";" => Ok(Token::new(
                TokenKind::Semicolon,
                self.reader.borrow_mut().get(),
                pos,
            )),
            "`" => Ok(Token::new(
                TokenKind::Backtick,
                self.reader.borrow_mut().get(),
                pos,
            )),
            _ => Err(ParseError {
                msg: format!("unexpected character: {}", c),
                pos: pos,
            }),
        }
    }

    pub fn get_sstring(&mut self) -> Result<String, ParseError> {
        self.reader.borrow_mut().skip_white();
        let c = self.reader.borrow().peek();
        if c != "'" {
            return Err(ParseError {
                msg: format!("unexpected character: {}", c),
                pos: self.reader.borrow_mut().getpos(),
            });
        }
        self.reader.borrow_mut().get();
        let mut value = String::new();
        loop {
            let c = self.reader.borrow().peek();
            if c == "<EOF>" || c == "\n" {
                return Err(ParseError {
                    msg: "unexpected EOL".to_string(),
                    pos: self.reader.borrow_mut().getpos(),
                });
            }
            if c == "'" {
                self.reader.borrow_mut().get();
                if self.reader.borrow().peek() == "'" {
                    self.reader.borrow_mut().get();
                    value.push_str("''")
                } else {
                    break;
                }
            } else {
                value.push_str(&self.reader.borrow_mut().get());
            }
        }
        Ok(value)
    }

    pub fn get_dstring(&mut self) -> Result<String, ParseError> {
        self.reader.borrow_mut().skip_white();
        let c = self.reader.borrow().peek();
        if c != "\"" {
            return Err(ParseError {
                msg: format!("unexpected character: {}", c),
                pos: self.reader.borrow_mut().getpos(),
            });
        }
        self.reader.borrow_mut().get();
        let mut value = String::new();
        loop {
            let c = self.reader.borrow().peek();
            if c == "<EOF>" || c == "\n" {
                return Err(ParseError {
                    msg: "unexpected EOL".to_string(),
                    pos: self.reader.borrow_mut().getpos(),
                });
            }
            if c == "\"" {
                self.reader.borrow_mut().get();
                break;
            } else if c == "\\" {
                value.push_str(&self.reader.borrow_mut().get());
                let c = self.reader.borrow().peek();
                if c == "<EOF>" || c == "\n" {
                    return Err(ParseError {
                        msg: "unexpected EOL".to_string(),
                        pos: self.reader.borrow_mut().getpos(),
                    });
                }
                value.push_str(&self.reader.borrow_mut().get());
            } else {
                value.push_str(&self.reader.borrow_mut().get());
            }
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_eof() {
        let mut reader = Reader::from_lines(vec![]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::EOF,
                "<EOF>".to_string(),
                Position::new(0, 1, 0)
            ))
        );
    }

    #[test]
    fn test_get_eol() {
        let mut reader = Reader::from_lines(vec!["\n"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::EOL,
                "\n".to_string(),
                Position::new(0, 1, 1)
            ))
        );
    }

    #[test]
    fn test_get_number() {
        let mut reader = Reader::from_lines(vec!["0xFF 0Xff 0b01 0B10 0123 1.2e+3 1.2E-3 123"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "0xFF".to_string(),
                Position::new(0, 1, 1),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "0Xff".to_string(),
                Position::new(5, 1, 6),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "0b01".to_string(),
                Position::new(10, 1, 11),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "0B10".to_string(),
                Position::new(15, 1, 16),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "0123".to_string(),
                Position::new(20, 1, 21),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "1.2e+3".to_string(),
                Position::new(25, 1, 26),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "1.2E-3".to_string(),
                Position::new(32, 1, 33),
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Number,
                "123".to_string(),
                Position::new(39, 1, 40),
            ))
        );
    }

    #[test]
    fn test_get_is() {
        let mut reader = Reader::from_lines(vec!["is? is# is"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::IsCI,
                "is?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::IsCS,
                "is#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Is,
                "is".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_is_not() {
        let mut reader = Reader::from_lines(vec!["isnot? isnot# isnot"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::IsNotCI,
                "isnot?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::IsNotCS,
                "isnot#".to_string(),
                Position::new(7, 1, 8)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::IsNot,
                "isnot".to_string(),
                Position::new(14, 1, 15)
            ))
        );
    }

    #[test]
    fn test_get_identifier() {
        let mut reader = Reader::from_lines(vec!["Foobar baz_quux"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Identifier,
                "Foobar".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Identifier,
                "baz_quux".to_string(),
                Position::new(7, 1, 8)
            ))
        );
    }

    #[test]
    fn test_get_or_or() {
        let mut reader = Reader::from_lines(vec!["||"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::OrOr,
                "||".to_string(),
                Position::new(0, 1, 1)
            ))
        );
    }

    #[test]
    fn test_get_and_and() {
        let mut reader = Reader::from_lines(vec!["&&"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::AndAnd,
                "&&".to_string(),
                Position::new(0, 1, 1)
            ))
        );
    }

    #[test]
    fn test_get_eq_eq() {
        let mut reader = Reader::from_lines(vec!["==? ==# =="]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::EqEqCI,
                "==?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::EqEqCS,
                "==#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::EqEq,
                "==".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_not_eq() {
        let mut reader = Reader::from_lines(vec!["!=? !=# !="]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::NotEqCI,
                "!=?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::NotEqCS,
                "!=#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::NotEq,
                "!=".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_gt_eq() {
        let mut reader = Reader::from_lines(vec![">=? >=# >="]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::GTEqCI,
                ">=?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::GTEqCS,
                ">=#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::GTEq,
                ">=".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_lt_eq() {
        let mut reader = Reader::from_lines(vec!["<=? <=# <="]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::LTEqCI,
                "<=?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::LTEqCS,
                "<=#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::LTEq,
                "<=".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_match() {
        let mut reader = Reader::from_lines(vec!["=~? =~# =~"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::MatchCI,
                "=~?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::MatchCS,
                "=~#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Match,
                "=~".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_no_match() {
        let mut reader = Reader::from_lines(vec!["!~? !~# !~"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::NoMatchCI,
                "!~?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::NoMatchCS,
                "!~#".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::NoMatch,
                "!~".to_string(),
                Position::new(8, 1, 9)
            ))
        );
    }

    #[test]
    fn test_get_greater_than() {
        let mut reader = Reader::from_lines(vec![">? ># >"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::GTCI,
                ">?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::GTCS,
                ">#".to_string(),
                Position::new(3, 1, 4)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::GT,
                ">".to_string(),
                Position::new(6, 1, 7)
            ))
        );
    }

    #[test]
    fn test_get_less_than() {
        let mut reader = Reader::from_lines(vec!["<? <# <"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::LTCI,
                "<?".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::LTCS,
                "<#".to_string(),
                Position::new(3, 1, 4)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::LT,
                "<".to_string(),
                Position::new(6, 1, 7)
            ))
        );
    }

    #[test]
    fn test_get_plus() {
        let mut reader = Reader::from_lines(vec!["+"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Plus,
                "+".to_string(),
                Position::new(0, 1, 1)
            ))
        );
    }

    #[test]
    fn test_get_minus_or_arrow() {
        let mut reader = Reader::from_lines(vec!["-> -"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Arrow,
                "->".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Minus,
                "-".to_string(),
                Position::new(3, 1, 4)
            ))
        );
    }

    #[test]
    fn test_get_dot_or_ellipsis() {
        let mut reader = Reader::from_lines(vec!["... ."]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::DotDotDot,
                "...".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Dot,
                ".".to_string(),
                Position::new(4, 1, 5)
            ))
        );
    }

    #[test]
    fn test_get_single_char_tokens() {
        let mut reader = Reader::from_lines(vec!["*/%!?:#()[]{},'\""]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Star,
                "*".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Slash,
                "/".to_string(),
                Position::new(1, 1, 2)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Percent,
                "%".to_string(),
                Position::new(2, 1, 3)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Not,
                "!".to_string(),
                Position::new(3, 1, 4)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Question,
                "?".to_string(),
                Position::new(4, 1, 5)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Colon,
                ":".to_string(),
                Position::new(5, 1, 6)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Sharp,
                "#".to_string(),
                Position::new(6, 1, 7)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::POpen,
                "(".to_string(),
                Position::new(7, 1, 8)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::PClose,
                ")".to_string(),
                Position::new(8, 1, 9)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::SqOpen,
                "[".to_string(),
                Position::new(9, 1, 10)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::SqClose,
                "]".to_string(),
                Position::new(10, 1, 11)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::COpen,
                "{".to_string(),
                Position::new(11, 1, 12)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::CClose,
                "}".to_string(),
                Position::new(12, 1, 13)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Comma,
                ",".to_string(),
                Position::new(13, 1, 14)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::SQuote,
                "'".to_string(),
                Position::new(14, 1, 15)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::DQuote,
                "\"".to_string(),
                Position::new(15, 1, 16)
            ))
        );
    }

    #[test]
    fn test_get_env() {
        let mut reader = Reader::from_lines(vec!["$FOO $bar"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Env,
                "$FOO".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Env,
                "$bar".to_string(),
                Position::new(5, 1, 6)
            ))
        );
    }

    #[test]
    fn test_get_reg() {
        let mut reader = Reader::from_lines(vec![r#"@" @a"#]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Reg,
                "@\"".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Reg,
                "@a".to_string(),
                Position::new(3, 1, 4)
            ))
        );
    }

    #[test]
    fn test_get_option() {
        let mut reader = Reader::from_lines(vec!["&g:foo &l:bar &baz"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Option,
                "&g:foo".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Option,
                "&l:bar".to_string(),
                Position::new(7, 1, 8)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Option,
                "&baz".to_string(),
                Position::new(14, 1, 15)
            ))
        );
    }

    #[test]
    fn test_get_more_single_char_tokens() {
        let mut reader = Reader::from_lines(vec!["=|;`"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Eq,
                "=".to_string(),
                Position::new(0, 1, 1)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Or,
                "|".to_string(),
                Position::new(1, 1, 2)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Semicolon,
                ";".to_string(),
                Position::new(2, 1, 3)
            ))
        );
        assert_eq!(
            tokenizer.get(),
            Ok(Token::new(
                TokenKind::Backtick,
                "`".to_string(),
                Position::new(3, 1, 4)
            ))
        );
    }

    #[test]
    fn test_bad_tokens() {
        let mut reader = Reader::from_lines(vec!["^"]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(
            tokenizer.get(),
            Err(ParseError {
                msg: "unexpected character: ^".to_string(),
                pos: Position::new(0, 1, 1)
            })
        );
    }

    #[test]
    fn test_get_sstring() {
        let mut reader = Reader::from_lines(vec![r#"'foo''"bar'"#]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(tokenizer.get_sstring(), Ok("foo\'\'\"bar".to_string()));
    }

    #[test]
    fn test_get_dstring() {
        let mut reader = Reader::from_lines(vec![r#""foo\"bar""#]);
        let mut tokenizer = Tokenizer::new(Rc::new(RefCell::new(reader)));
        assert_eq!(tokenizer.get_dstring(), Ok("foo\\\"bar".to_string()));
    }
}
