use super::{isargname, isnamec, iswhite, iswordc, ParseError, Position};
use exarg::ExArg;
use reader::Reader;
use std::cell::RefCell;
use std::rc::Rc;
use token::{Token, TokenKind, Tokenizer};

const MAX_FUNC_ARGS: usize = 20;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    Add,
    And,
    Break,
    Call,
    Catch,
    Comment,
    Concat,
    Continue,
    CurlyName,
    CurlyNameExpr,
    CurlyNamePart,
    DelFunction,
    Dict,
    Divide,
    Dot,
    Dummy, // for use in returning from a parser early when result won't be used
    Echo,
    EchoErr,
    EchoHl,
    EchoMsg,
    EchoN,
    Else,
    ElseIf,
    EndFor,
    EndFunction,
    EndTry,
    EndWhile,
    EndIf,
    Env,
    Equal,
    EqualCI,
    EqualCS,
    ExCall,
    ExCmd,
    Execute,
    Finally,
    For,
    Function,
    GEqual,
    GEqualCI,
    GEqualCS,
    Greater,
    GreaterCI,
    GreaterCS,
    Identifier,
    If,
    Is,
    IsCI,
    IsCS,
    IsNot,
    IsNotCI,
    IsNotCS,
    Lambda,
    Let,
    List,
    LockVar,
    Match,
    MatchCI,
    MatchCS,
    Minus,
    Multiply,
    NEqual,
    NEqualCI,
    NEqualCS,
    NoMatch,
    NoMatchCI,
    NoMatchCS,
    Not,
    Number,
    Option,
    Or,
    Plus,
    Reg,
    Remainder,
    Return,
    SEqual,
    SEqualCI,
    SEqualCS,
    Shebang,
    Slice,
    Smaller,
    SmallerCI,
    SmallerCS,
    String,
    Subscript,
    Subtract,
    Ternary,
    Throw,
    TopLevel,
    Try,
    Unlet,
    UnlockVar,
    While,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub body: Vec<Box<Node>>,
    pub pos: Position,
    pub string: String,
    pub ea: Option<ExArg>,
    pub cond: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub value: String,
    pub list: Vec<Box<Node>>,
    pub rlist: Vec<Box<Node>>,
    pub dict: Vec<(Box<Node>, Box<Node>)>,
    pub pattern: Option<String>,
    pub catch: Vec<Box<Node>>,
    pub else_: Option<Box<Node>>,
    pub elseif: Vec<Box<Node>>,
    pub end: Option<Box<Node>>,
    pub attrs: Vec<String>,
    pub finally: Option<Box<Node>>,
    pub rest: Vec<Box<Node>>,
    pub op: String,
    pub depth: Option<usize>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Node {
        Node {
            kind: kind,
            body: vec![],
            pos: Position::empty(),
            string: String::new(),
            ea: None,
            cond: None,
            left: None,
            right: None,
            value: String::new(),
            list: vec![],
            rlist: vec![],
            dict: vec![],
            pattern: None,
            catch: vec![],
            else_: None,
            elseif: vec![],
            end: None,
            attrs: vec![],
            finally: None,
            rest: vec![],
            op: String::new(),
            depth: None,
        }
    }
}

#[derive(Debug)]
pub struct NodeParser {
    reader: Rc<RefCell<Reader>>,
    tokenizer: Tokenizer,
}

impl NodeParser {
    pub fn new(reader: Rc<RefCell<Reader>>) -> NodeParser {
        let tokenizer = Tokenizer::new(Rc::clone(&reader));
        NodeParser {
            reader: reader,
            tokenizer: tokenizer,
        }
    }

    fn token_err<T>(&self, token: Token) -> Result<T, ParseError> {
        Err(ParseError {
            msg: format!("unexpected token: {}", token.value),
            pos: token.pos,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.parse_expr1()
    }

    fn parse_expr1(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr2()?;
        let pos = self.reader.borrow().tell();
        let mut token = self.tokenizer.get()?;
        if token.kind == TokenKind::Question {
            let mut node = Node::new(NodeKind::Ternary);
            node.pos = token.pos;
            node.cond = Some(Box::new(left));
            node.left = Some(Box::new(self.parse_expr1()?));
            token = self.tokenizer.get()?;
            if token.kind != TokenKind::Colon {
                return self.token_err(token);
            }
            node.right = Some(Box::new(self.parse_expr1()?));
            left = node;
        } else {
            self.reader.borrow_mut().seek_set(pos);
        }
        Ok(left)
    }

    fn parse_expr2(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr3()?;
        loop {
            let pos = self.reader.borrow().tell();
            let token = self.tokenizer.get()?;
            if token.kind == TokenKind::OrOr {
                let mut node = Node::new(NodeKind::Or);
                node.pos = token.pos;
                node.left = Some(Box::new(left));
                node.right = Some(Box::new(self.parse_expr3()?));
                left = node;
            } else {
                self.reader.borrow_mut().seek_set(pos);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr3(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr4()?;
        loop {
            let pos = self.reader.borrow().tell();
            let token = self.tokenizer.get()?;
            if token.kind == TokenKind::AndAnd {
                let mut node = Node::new(NodeKind::And);
                node.pos = token.pos;
                node.left = Some(Box::new(left));
                node.right = Some(Box::new(self.parse_expr4()?));
                left = node;
            } else {
                self.reader.borrow_mut().seek_set(pos);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr4(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr5()?;
        let pos = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let mut node = Node::new(NodeKind::Dummy);
        node.pos = token.pos;
        node.left = Some(Box::new(left.clone()));
        match token.kind {
            TokenKind::EqEq => {
                node.kind = NodeKind::Equal;
            }
            TokenKind::EqEqCI => {
                node.kind = NodeKind::EqualCI;
            }
            TokenKind::EqEqCS => {
                node.kind = NodeKind::EqualCS;
            }
            TokenKind::NotEq => {
                node.kind = NodeKind::NEqual;
            }
            TokenKind::NotEqCI => {
                node.kind = NodeKind::NEqualCI;
            }
            TokenKind::NotEqCS => {
                node.kind = NodeKind::NEqualCS;
            }
            TokenKind::GT => {
                node.kind = NodeKind::Greater;
            }
            TokenKind::GTCI => {
                node.kind = NodeKind::GreaterCI;
            }
            TokenKind::GTCS => {
                node.kind = NodeKind::GreaterCS;
            }
            TokenKind::GTEq => {
                node.kind = NodeKind::GEqual;
            }
            TokenKind::GTEqCI => {
                node.kind = NodeKind::GEqualCI;
            }
            TokenKind::GTEqCS => {
                node.kind = NodeKind::GEqualCS;
            }
            TokenKind::LT => {
                node.kind = NodeKind::Smaller;
            }
            TokenKind::LTCI => {
                node.kind = NodeKind::SmallerCI;
            }
            TokenKind::LTCS => {
                node.kind = NodeKind::SmallerCS;
            }
            TokenKind::LTEq => {
                node.kind = NodeKind::SEqual;
            }
            TokenKind::LTEqCI => {
                node.kind = NodeKind::SEqualCI;
            }
            TokenKind::LTEqCS => {
                node.kind = NodeKind::SEqualCS;
            }
            TokenKind::Match => {
                node.kind = NodeKind::Match;
            }
            TokenKind::MatchCI => {
                node.kind = NodeKind::MatchCI;
            }
            TokenKind::MatchCS => {
                node.kind = NodeKind::MatchCS;
            }
            TokenKind::NoMatch => {
                node.kind = NodeKind::NoMatch;
            }
            TokenKind::NoMatchCI => {
                node.kind = NodeKind::NoMatchCI;
            }
            TokenKind::NoMatchCS => {
                node.kind = NodeKind::NoMatchCS;
            }
            TokenKind::Is => {
                node.kind = NodeKind::Is;
            }
            TokenKind::IsCI => {
                node.kind = NodeKind::IsCI;
            }
            TokenKind::IsCS => {
                node.kind = NodeKind::IsCS;
            }
            TokenKind::IsNot => {
                node.kind = NodeKind::IsNot;
            }
            TokenKind::IsNotCI => {
                node.kind = NodeKind::IsNotCI;
            }
            TokenKind::IsNotCS => {
                node.kind = NodeKind::IsNotCS;
            }
            _ => {
                self.reader.borrow_mut().seek_set(pos);
                return Ok(left);
            }
        };
        if node.kind != NodeKind::Dummy {
            left = node;
        }
        node.right = Some(Box::new(self.parse_expr5()?));
        Ok(left)
    }

    fn parse_expr5(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr6()?;
        loop {
            let pos = self.reader.borrow().tell();
            let token = self.tokenizer.get()?;
            let mut node = Node::new(NodeKind::Dummy);
            node.pos = token.pos;
            match token.kind {
                TokenKind::Plus => {
                    node.kind = NodeKind::Add;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(self.parse_expr6()?));
                    left = node;
                }
                TokenKind::Minus => {
                    node.kind = NodeKind::Subtract;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(self.parse_expr6()?));
                    left = node;
                }
                TokenKind::Dot => {
                    node.kind = NodeKind::Concat;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(self.parse_expr6()?));
                    left = node;
                }
                _ => {
                    self.reader.borrow_mut().seek_set(pos);
                    break;
                }
            };
        }
        Ok(left)
    }

    fn parse_expr6(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr7()?;
        loop {
            let pos = self.reader.borrow().tell();
            let token = self.tokenizer.get()?;
            let mut node = Node::new(NodeKind::Dummy);
            node.pos = token.pos;
            match token.kind {
                TokenKind::Star => {
                    node.kind = NodeKind::Multiply;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(self.parse_expr7()?));
                    left = node;
                }
                TokenKind::Slash => {
                    node.kind = NodeKind::Divide;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(self.parse_expr7()?));
                    left = node;
                }
                TokenKind::Percent => {
                    node.kind = NodeKind::Remainder;
                    node.left = Some(Box::new(left));
                    node.right = Some(Box::new(self.parse_expr7()?));
                    left = node;
                }
                _ => {
                    self.reader.borrow_mut().seek_set(pos);
                    break;
                }
            }
        }
        Ok(left)
    }

    fn parse_expr7(&mut self) -> Result<Node, ParseError> {
        let pos = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let mut node = Node::new(NodeKind::Dummy);
        node.pos = token.pos;
        match token.kind {
            TokenKind::Not => {
                node.kind = NodeKind::Not;
                node.left = Some(Box::new(self.parse_expr7()?));
            }
            TokenKind::Minus => {
                node.kind = NodeKind::Minus;
                node.left = Some(Box::new(self.parse_expr7()?));
            }
            TokenKind::Plus => {
                node.kind = NodeKind::Plus;
                node.left = Some(Box::new(self.parse_expr7()?));
            }
            _ => {
                self.reader.borrow_mut().seek_set(pos);
                node = self.parse_expr8()?;
            }
        }
        Ok(node)
    }

    fn parse_expr8(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr9()?;
        loop {
            let pos = self.reader.borrow().tell();
            let c = self.reader.borrow().peek();
            let mut token = self.tokenizer.get()?;
            if !iswhite(&c) && token.kind == TokenKind::SqOpen {
                let npos = token.pos;
                if self.tokenizer.peek()?.kind == TokenKind::Colon {
                    self.tokenizer.get()?;
                    let mut node = Node::new(NodeKind::Slice);
                    node.pos = npos;
                    node.left = Some(Box::new(left.clone()));
                    token = self.tokenizer.peek()?;
                    if token.kind != TokenKind::SqClose {
                        node.rlist.push(Box::new(Node::new(NodeKind::Dummy)));
                        node.rlist.push(Box::new(self.parse_expr1()?));
                    }
                    token = self.tokenizer.get()?;
                    if token.kind != TokenKind::SqClose {
                        return self.token_err(token);
                    }
                } else {
                    let right = self.parse_expr1()?;
                    if self.tokenizer.peek()?.kind == TokenKind::Colon {
                        self.tokenizer.get()?;
                        let mut node = Node::new(NodeKind::Slice);
                        node.pos = npos;
                        node.left = Some(Box::new(left.clone()));
                        token = self.tokenizer.peek()?;
                        if token.kind != TokenKind::SqClose {
                            node.rlist.push(Box::new(Node::new(NodeKind::Dummy)));
                            node.rlist.push(Box::new(self.parse_expr1()?));
                        }
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                    } else {
                        let mut node = Node::new(NodeKind::Subscript);
                        node.pos = npos;
                        node.left = Some(Box::new(left.clone()));
                        node.right = Some(Box::new(right));
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                        left = node;
                    }
                }
            } else if token.kind == TokenKind::POpen {
                let mut node = Node::new(NodeKind::Call);
                node.pos = token.pos;
                node.left = Some(Box::new(left.clone()));
                if self.tokenizer.peek()?.kind == TokenKind::PClose {
                    self.tokenizer.get()?;
                } else {
                    loop {
                        node.rlist.push(Box::new(self.parse_expr1()?));
                        token = self.tokenizer.get()?;
                        if token.kind == TokenKind::Comma {
                            if self.tokenizer.peek()?.kind == TokenKind::PClose {
                                self.tokenizer.get()?;
                                break;
                            }
                        } else if token.kind == TokenKind::PClose {
                            break;
                        } else {
                            return self.token_err(token);
                        }
                    }
                }
                if node.rlist.len() > MAX_FUNC_ARGS {
                    return Err(ParseError {
                        msg: "E740: Too many arguments for function".to_string(),
                        pos: node.pos,
                    });
                }
                left = node;
            } else if !iswhite(&c) && token.kind == TokenKind::Dot {
                match self.parse_dot(token, left.clone()) {
                    Some(node) => {
                        left = node;
                    }
                    None => {
                        self.reader.borrow_mut().seek_set(pos);
                        break;
                    }
                }
            } else {
                self.reader.borrow_mut().seek_set(pos);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr9(&mut self) -> Result<Node, ParseError> {
        let pos = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let mut node = Node::new(NodeKind::Dummy);
        node.pos = token.pos;
        match token.kind {
            TokenKind::Number => {
                node.kind = NodeKind::Number;
                node.value = token.value;
            }
            TokenKind::DQuote => {
                self.reader.borrow_mut().seek_set(pos);
                node.kind = NodeKind::String;
                node.value = format!("\"{}\"", self.tokenizer.get_dstring()?);
            }
            TokenKind::SQuote => {
                self.reader.borrow_mut().seek_set(pos);
                node.kind = NodeKind::String;
                node.value = format!("'{}'", self.tokenizer.get_sstring()?);
            }
            TokenKind::SqOpen => {
                node.kind = NodeKind::List;
                let token = self.tokenizer.peek()?;
                if token.kind == TokenKind::SqClose {
                    self.tokenizer.get()?;
                } else {
                    loop {
                        node.body.push(Box::new(self.parse_expr1()?));
                        let token = self.tokenizer.peek()?;
                        match token.kind {
                            TokenKind::Comma => {
                                self.tokenizer.get()?;
                                if self.tokenizer.peek()?.kind == TokenKind::SqClose {
                                    self.tokenizer.get()?;
                                    break;
                                }
                            }
                            TokenKind::SqClose => {
                                self.tokenizer.get()?;
                                break;
                            }
                            _ => {
                                return self.token_err(token);
                            }
                        }
                    }
                }
            }
            TokenKind::COpen => {
                let savepos = self.reader.borrow().tell();
                let nodepos = token.pos;
                let mut token = self.tokenizer.get()?;
                let mut is_lambda = token.kind == TokenKind::Arrow;
                if !is_lambda && token.kind != TokenKind::SQuote && token.kind != TokenKind::DQuote
                {
                    let token2 = self.tokenizer.peek()?;
                    is_lambda = token2.kind == TokenKind::Arrow || token2.kind == TokenKind::Comma;
                }
                let mut fallback = false;
                if is_lambda {
                    node.kind = NodeKind::Lambda;
                    node.pos = nodepos;
                    let mut named: Vec<String> = vec![];
                    loop {
                        match token.kind {
                            TokenKind::Arrow => {
                                break;
                            }
                            TokenKind::Identifier => {
                                if !isargname(&token.value) {
                                    return Err(ParseError {
                                        msg: format!("E125: Illegal argument: {}", token.value),
                                        pos: token.pos,
                                    });
                                } else if named.contains(&token.value.clone()) {
                                    return Err(ParseError {
                                        msg: format!(
                                            "E853: Duplicate argument name: {}",
                                            token.value
                                        ),
                                        pos: token.pos,
                                    });
                                }
                                named.push(token.value.clone());
                                let mut varnode = Node::new(NodeKind::Identifier);
                                varnode.pos = token.pos;
                                varnode.value = token.value;
                                if iswhite(&self.reader.borrow().peek())
                                    && self.tokenizer.peek()?.kind == TokenKind::Comma
                                {
                                    return Err(ParseError {
                                        msg: String::from(
                                            "E475: invalid argument: White space is not allowed before comma"
                                        ),
                                        pos: self.reader.borrow_mut().getpos()
                                    });
                                }
                                token = self.tokenizer.get()?;
                                node.rlist.push(Box::new(varnode));
                                if token.kind == TokenKind::Comma {
                                    token = self.tokenizer.peek()?;
                                    if token.kind == TokenKind::Arrow {
                                        self.tokenizer.get()?;
                                        break;
                                    }
                                } else if token.kind == TokenKind::Arrow {
                                    break;
                                } else {
                                    return Err(ParseError {
                                        msg: format!(
                                            "unexpected token: {}, type: {:#?}",
                                            token.value, token.kind
                                        ),
                                        pos: token.pos,
                                    });
                                }
                            }
                            TokenKind::DotDotDot => {
                                let mut varnode = Node::new(NodeKind::Identifier);
                                varnode.pos = token.pos;
                                varnode.value = token.value;
                                node.rlist.push(Box::new(varnode));
                                token = self.tokenizer.peek()?;
                                if token.kind == TokenKind::Arrow {
                                    self.tokenizer.get()?;
                                    break;
                                } else {
                                    return self.token_err(token);
                                }
                            }
                            _ => {
                                fallback = true;
                                break;
                            }
                        }
                        token = self.tokenizer.get()?;
                    }
                    if !fallback {
                        node.left = Some(Box::new(self.parse_expr1()?));
                        token = self.tokenizer.get()?;
                        if token.kind == TokenKind::CClose {
                            return self.token_err(token);
                        }
                        return Ok(node);
                    }
                }
                node = Node::new(NodeKind::Dict);
                node.pos = nodepos;
                self.reader.borrow_mut().seek_set(savepos);
                token = self.tokenizer.peek()?;
                if token.kind == TokenKind::CClose {
                    self.tokenizer.get()?;
                    return Ok(node);
                }
                loop {
                    let key = self.parse_expr1()?;
                    token = self.tokenizer.get()?;
                    if token.kind == TokenKind::CClose {
                        if node.body.len() > 0 {
                            return self.token_err(token);
                        }
                        self.reader.borrow_mut().seek_set(pos);
                        node = self.parse_identifier()?;
                        break;
                    }
                    if token.kind != TokenKind::Colon {
                        return self.token_err(token);
                    }
                    let val = self.parse_expr1()?;
                    node.dict.push((Box::new(key), Box::new(val)));
                    token = self.tokenizer.get()?;
                    if token.kind == TokenKind::Comma {
                        if self.tokenizer.peek()?.kind == TokenKind::CClose {
                            self.tokenizer.get()?;
                            break;
                        } else if token.kind == TokenKind::CClose {
                            break;
                        } else {
                            return self.token_err(token);
                        }
                    }
                }
            }
            TokenKind::POpen => {
                node = self.parse_expr1()?;
                let token = self.tokenizer.get()?;
                if token.kind != TokenKind::PClose {
                    return self.token_err(token);
                }
            }
            TokenKind::Option => {
                node.kind = NodeKind::Option;
                node.value = token.value;
            }
            TokenKind::Identifier => {
                self.reader.borrow_mut().seek_set(pos);
                node = self.parse_identifier()?;
            }
            _ if token.kind == TokenKind::LT
                && self.reader.borrow().peekn(4).eq_ignore_ascii_case("SID>") =>
            {
                self.reader.borrow_mut().seek_set(pos);
                node = self.parse_identifier()?;
            }
            TokenKind::Is | TokenKind::IsCS | TokenKind::IsNot | TokenKind::IsNotCS => {
                self.reader.borrow_mut().seek_set(pos);
                node = self.parse_identifier()?;
            }
            TokenKind::Env => {
                node.kind = NodeKind::Env;
                node.value = token.value;
            }
            TokenKind::Reg => {
                node.kind = NodeKind::Reg;
                node.value = token.value;
            }
            _ => {
                return self.token_err(token);
            }
        };
        Ok(node)
    }

    fn parse_identifier(&mut self) -> Result<Node, ParseError> {
        self.reader.borrow_mut().skip_white();
        let mut node = Node::new(NodeKind::Dummy);
        node.pos = self.reader.borrow_mut().getpos();
        let curly_parts = self.parse_curly_parts()?;
        if curly_parts.len() == 1 && curly_parts[0].kind == NodeKind::CurlyNamePart {
            node.kind = NodeKind::Identifier;
            node.value = curly_parts.first().unwrap().value.clone();
        } else {
            node.kind = NodeKind::CurlyName;
            node.body = curly_parts
                .into_iter()
                .map(|n| Box::new(n))
                .collect::<Vec<Box<Node>>>();
        }
        Ok(node)
    }

    fn parse_curly_parts(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut curly_parts = vec![];
        let c = self.reader.borrow().peek();
        let pos = self.reader.borrow_mut().getpos();
        if c == "<" && self.reader.borrow().peekn(5).eq_ignore_ascii_case("<SID>") {
            let name = self.reader.borrow_mut().getn(5);
            let mut node = Node::new(NodeKind::CurlyNamePart);
            node.pos = pos;
            node.value = name;
            curly_parts.push(node);
        }
        loop {
            let c = self.reader.borrow().peek();
            if isnamec(&c) {
                let pos = self.reader.borrow_mut().getpos();
                let name = self.reader.borrow_mut().read_name();
                let mut node = Node::new(NodeKind::CurlyNamePart);
                node.pos = pos;
                node.value = name;
                curly_parts.push(node);
            } else if c == "{" {
                self.reader.borrow_mut().get();
                let pos = self.reader.borrow_mut().getpos();
                let mut node = self.parse_expr1()?;
                node.kind = NodeKind::CurlyNameExpr;
                node.pos = pos;
                curly_parts.push(node);
                self.reader.borrow_mut().skip_white();
                let c = self.reader.borrow().peek();
                if c != "}" {
                    return Err(ParseError {
                        msg: format!("unexpected token: {}", c),
                        pos: self.reader.borrow_mut().getpos(),
                    });
                }
                self.reader.borrow_mut().seek_cur(1);
            } else {
                break;
            }
        }
        Ok(curly_parts)
    }

    fn parse_dot(&mut self, token: Token, left: Node) -> Option<Node> {
        if ![
            NodeKind::Identifier,
            NodeKind::CurlyName,
            NodeKind::Dict,
            NodeKind::Subscript,
            NodeKind::Call,
            NodeKind::Dot,
        ]
            .contains(&left.kind)
        {
            return None;
        }
        if !iswordc(&self.reader.borrow().peek()) {
            return None;
        }
        let pos = self.reader.borrow_mut().getpos();
        let name = self.reader.borrow_mut().read_word();
        if isnamec(&self.reader.borrow().peek()) {
            return None;
        }
        let mut right = Node::new(NodeKind::Identifier);
        right.pos = pos;
        right.value = name;
        let mut node = Node::new(NodeKind::Dot);
        node.pos = token.pos;
        node.left = Some(Box::new(left));
        node.right = Some(Box::new(right));
        Some(node)
    }

    pub fn parse_lv(&mut self) -> Result<Node, ParseError> {
        self.parse_lv8()
    }

    fn parse_lv8(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_lv9()?;
        loop {
            let pos = self.reader.borrow().tell();
            let c = self.reader.borrow().peek();
            let mut token = self.tokenizer.get()?;
            if !iswhite(&c) && token.kind == TokenKind::SqOpen {
                let npos = token.pos;
                let mut node = Node::new(NodeKind::Dummy);
                node.pos = npos;
                if self.tokenizer.peek()?.kind == TokenKind::Colon {
                    self.tokenizer.get()?;
                    node = Node::new(NodeKind::Slice);
                    node.pos = npos;
                    node.left = Some(Box::new(left));
                    node.rlist = vec![Box::new(Node::new(NodeKind::Dummy))];
                    token = self.tokenizer.peek()?;
                    if token.kind == TokenKind::SqClose {
                        node.rlist.push(Box::new(self.parse_expr1()?));
                    } else {
                        node.rlist.push(Box::new(Node::new(NodeKind::Dummy)));
                    }
                    token = self.tokenizer.get()?;
                    if token.kind != TokenKind::SqClose {
                        return self.token_err(token);
                    }
                } else {
                    let right = self.parse_expr1()?;
                    if self.tokenizer.peek()?.kind == TokenKind::Colon {
                        self.tokenizer.get()?;
                        node = Node::new(NodeKind::Slice);
                        node.pos = npos;
                        node.left = Some(Box::new(left));
                        token = self.tokenizer.peek()?;
                        node.rlist = vec![Box::new(right)];
                        if token.kind == TokenKind::SqClose {
                            node.rlist.push(Box::new(self.parse_expr1()?));
                        } else {
                            node.rlist.push(Box::new(Node::new(NodeKind::Dummy)));
                        }
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                    } else {
                        node = Node::new(NodeKind::Subscript);
                        node.pos = npos;
                        node.left = Some(Box::new(left));
                        node.right = Some(Box::new(right));
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                    }
                }
                left = node;
            } else if !iswhite(&c) && token.kind == TokenKind::Dot {
                match self.parse_dot(token, left.clone()) {
                    Some(n) => {
                        left = n;
                    }
                    None => {
                        self.reader.borrow_mut().seek_set(pos);
                        break;
                    }
                }
            } else {
                self.reader.borrow_mut().seek_set(pos);
                break;
            }
        }
        Ok(left)
    }

    fn parse_lv9(&mut self) -> Result<Node, ParseError> {
        let pos = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let mut node = Node::new(NodeKind::Dummy);
        node.pos = token.pos;
        match token.kind {
            TokenKind::COpen | TokenKind::Identifier => {
                self.reader.borrow_mut().seek_set(pos);
                node = self.parse_identifier()?;
                node.value = token.value;
            }
            _ if self.reader.borrow().peekn(5).eq_ignore_ascii_case("<SID>") => {
                self.reader.borrow_mut().seek_set(pos);
                node = self.parse_identifier()?;
                node.value = token.value;
            }
            TokenKind::Option => {
                node.kind = NodeKind::Option;
                node.value = token.value;
            }
            TokenKind::Env => {
                node.kind = NodeKind::Env;
                node.value = token.value;
            }
            TokenKind::Reg => {
                node.kind = NodeKind::Reg;
                node.value = token.value;
            }
            _ => {
                return self.token_err(token);
            }
        };
        Ok(node)
    }
}
