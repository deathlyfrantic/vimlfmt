use super::{isargname, isnamec, iswhite, iswordc, ParseError, Position};
use exarg::ExArg;
use reader::Reader;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use token::{Token, TokenKind, Tokenizer};

const MAX_FUNC_ARGS: usize = 20;

fn indent(n: usize) -> String {
    "  ".repeat(n)
}

fn escape(s: &str) -> String {
    let mut rv = String::new();
    for c in s.chars() {
        if c == '\r' {
            rv.push_str("\\r");
        } else {
            if c == '\\' || c == '"' {
                rv.push('\\');
            }
            rv.push(c);
        }
    }
    rv
}

fn display_left<T>(name: &str, left: T) -> String
where
    T: fmt::Display,
{
    format!("({} {})", name, left)
}

fn display_lr<T>(name: &str, left: T, right: T) -> String
where
    T: fmt::Display,
{
    format!("({} {} {})", name, left, right)
}

fn display_with_list<T>(name: &str, list: &Vec<T>) -> String
where
    T: fmt::Display,
{
    format!(
        "({} {})",
        name,
        list.iter()
            .map(|n| format!("{}", *n))
            .collect::<Vec<String>>()
            .join(" ")
    )
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Add {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    And {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    BinOp {
        pos: Position,
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },
    Break {
        ea: ExArg,
        pos: Position,
    },
    Call {
        pos: Position,
        name: Box<Node>,
        args: Vec<Box<Node>>,
    },
    Catch {
        ea: ExArg,
        pos: Position,
        pattern: Option<String>,
        body: Vec<Rc<RefCell<Node>>>,
    },
    Comment {
        pos: Position,
        value: String,
    },
    Concat {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Continue {
        ea: ExArg,
        pos: Position,
    },
    CurlyName {
        pos: Position,
        pieces: Vec<Box<Node>>,
    },
    CurlyNameExpr {
        pos: Position,
        expr: Box<Node>,
    },
    CurlyNamePart {
        pos: Position,
        value: String,
    },
    DelFunction {
        ea: ExArg,
        pos: Position,
        left: Box<Node>,
    },
    Dict {
        pos: Position,
        items: Vec<(Box<Node>, Box<Node>)>,
    },
    Divide {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Dot {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Echo {
        ea: ExArg,
        pos: Position,
        list: Vec<Box<Node>>,
    },
    EchoErr {
        ea: ExArg,
        pos: Position,
        list: Vec<Box<Node>>,
    },
    EchoHl {
        ea: ExArg,
        pos: Position,
        value: String,
    },
    EchoMsg {
        ea: ExArg,
        pos: Position,
        list: Vec<Box<Node>>,
    },
    EchoN {
        ea: ExArg,
        pos: Position,
        list: Vec<Box<Node>>,
    },
    Else {
        ea: ExArg,
        pos: Position,
        body: Vec<Rc<RefCell<Node>>>,
    },
    ElseIf {
        ea: ExArg,
        pos: Position,
        cond: Box<Node>,
        body: Vec<Rc<RefCell<Node>>>,
    },
    End {
        ea: ExArg,
        pos: Position,
    },
    Env {
        pos: Position,
        value: String,
    },
    ExCall {
        ea: ExArg,
        pos: Position,
        left: Box<Node>,
    },
    ExCmd {
        ea: ExArg,
        pos: Position,
        value: String,
    },
    Execute {
        ea: ExArg,
        pos: Position,
        list: Vec<Box<Node>>,
    },
    Finally {
        ea: ExArg,
        pos: Position,
        body: Vec<Rc<RefCell<Node>>>,
    },
    For {
        ea: ExArg,
        pos: Position,
        var: Option<Box<Node>>,  // this is the x in "for x in something"
        list: Vec<Box<Node>>,    // this is the a, b in "for [a, b] in something"
        rest: Option<Box<Node>>, // this is the z in "for [a, b; z] in something" <- REAL SYNTAX :(
        right: Box<Node>,        // this is the something in "for x in something"
        body: Vec<Rc<RefCell<Node>>>,
        end: Option<Box<Node>>,
    },
    Function {
        ea: ExArg,
        pos: Position,
        name: Box<Node>,
        args: Vec<Box<Node>>,
        body: Vec<Rc<RefCell<Node>>>,
        attrs: Vec<String>,
        end: Option<Box<Node>>,
    },
    Identifier {
        pos: Position,
        value: String,
    },
    If {
        ea: ExArg,
        pos: Position,
        cond: Box<Node>,
        elseifs: Vec<Rc<RefCell<Node>>>,
        else_: Option<Rc<RefCell<Node>>>,
        body: Vec<Rc<RefCell<Node>>>,
        end: Option<Box<Node>>,
    },
    Lambda {
        pos: Position,
        args: Vec<Box<Node>>,
        expr: Box<Node>,
    },
    Let {
        ea: ExArg,
        pos: Position,
        var: Option<Box<Node>>,  // this is the x in "let x = something"
        list: Vec<Box<Node>>,    // this is the a, b in "let [a, b] = something"
        rest: Option<Box<Node>>, // this is the z in "let [a, b; z] = something" <- REAL SYNTAX :(
        right: Box<Node>,        // this is the something in "let x = something"
        op: String,
    },
    List {
        pos: Position,
        items: Vec<Box<Node>>,
    },
    LockVar {
        ea: ExArg,
        pos: Position,
        depth: Option<usize>,
        list: Vec<Box<Node>>,
    },
    Minus {
        pos: Position,
        left: Box<Node>,
    },
    Multiply {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Not {
        pos: Position,
        left: Box<Node>,
    },
    Number {
        pos: Position,
        value: String,
    },
    Option {
        pos: Position,
        value: String,
    },
    Or {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Plus {
        pos: Position,
        left: Box<Node>,
    },
    Reg {
        pos: Position,
        value: String,
    },
    Remainder {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Return {
        ea: ExArg,
        pos: Position,
        left: Option<Box<Node>>,
    },
    Shebang {
        pos: Position,
        value: String,
    },
    Slice {
        pos: Position,
        name: Box<Node>,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    String {
        pos: Position,
        value: String,
    },
    Subscript {
        pos: Position,
        name: Box<Node>,
        index: Box<Node>,
    },
    Subtract {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Ternary {
        pos: Position,
        cond: Box<Node>,
        left: Box<Node>,
        right: Box<Node>,
    },
    Throw {
        ea: ExArg,
        pos: Position,
        err: Box<Node>,
    },
    TopLevel {
        pos: Position,
        body: Vec<Rc<RefCell<Node>>>,
    },
    Try {
        ea: ExArg,
        pos: Position,
        body: Vec<Rc<RefCell<Node>>>,
        catches: Vec<Rc<RefCell<Node>>>,
        finally: Option<Rc<RefCell<Node>>>,
        end: Option<Box<Node>>,
    },
    Unlet {
        ea: ExArg,
        pos: Position,
        list: Vec<Box<Node>>,
    },
    UnlockVar {
        ea: ExArg,
        pos: Position,
        depth: Option<usize>,
        list: Vec<Box<Node>>,
    },
    While {
        ea: ExArg,
        pos: Position,
        body: Vec<Rc<RefCell<Node>>>,
        cond: Box<Node>,
        end: Option<Box<Node>>,
    },
}

impl Node {
    pub fn is_for(node: &Node) -> bool {
        match node {
            Node::For { .. } => true,
            _ => false,
        }
    }

    pub fn is_function(node: &Node) -> bool {
        match node {
            Node::Function { .. } => true,
            _ => false,
        }
    }

    pub fn is_while(node: &Node) -> bool {
        match node {
            Node::While { .. } => true,
            _ => false,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Node::Add { left, right, .. } => display_lr("+", left, right),
                Node::And { left, right, .. } => display_lr("&&", left, right),
                Node::BinOp {
                    op, left, right, ..
                } => format!("({} {} {})", op, left, right),
                Node::Break { .. } => "(break)".to_string(),
                Node::Call { name, args, .. } => {
                    if args.len() > 0 {
                        format!(
                            "({} {})",
                            name,
                            args.iter()
                                .map(|n| format!("{}", n))
                                .collect::<Vec<String>>()
                                .join(" ")
                        )
                    } else {
                        format!("({})", name)
                    }
                }
                Node::Comment { value, .. } => format!(";{}", value),
                Node::Concat { left, right, .. } => display_lr("concat", left, right),
                Node::Continue { .. } => "(continue)".to_string(),
                Node::CurlyName { pieces, .. } => pieces
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join(""),
                Node::CurlyNameExpr { expr, .. } => format!("{{{}}}", expr),
                Node::CurlyNamePart { value, .. }
                | Node::Env { value, .. }
                | Node::Identifier { value, .. }
                | Node::Number { value, .. }
                | Node::Option { value, .. }
                | Node::Reg { value, .. }
                | Node::String { value, .. } => value.clone(),
                Node::DelFunction { left, .. } => display_left("delfunction", left),
                Node::Dict { items, .. } => {
                    if items.len() > 0 {
                        format!(
                            "(dict {})",
                            items
                                .iter()
                                .map(|(k, v)| format!("({} {})", k, v))
                                .collect::<Vec<String>>()
                                .join(" ")
                        )
                    } else {
                        "(dict)".to_string()
                    }
                }
                Node::Divide { left, right, .. } => display_lr("/", left, right),
                Node::Dot { left, right, .. } => display_lr("dot", left, right),
                Node::Echo { list, .. } => display_with_list("echo", list),
                Node::EchoErr { list, .. } => display_with_list("echoerr", list),
                Node::EchoHl { value, .. } => format!("(echohl \"{}\")", escape(value)),
                Node::EchoMsg { list, .. } => display_with_list("echomsg", list),
                Node::EchoN { list, .. } => display_with_list("echon", list),
                Node::ExCall { left, .. } => display_left("call", left),
                Node::ExCmd { value, .. } => format!("(excmd \"{}\")", escape(value)),
                Node::Execute { list, .. } => display_with_list("execute", list),
                Node::For {
                    var,
                    list,
                    rest,
                    right,
                    body,
                    ..
                } => {
                    let left = if let Some(v) = var {
                        format!("{}", v)
                    } else {
                        let mut l = format!(
                            "({}",
                            list.iter()
                                .map(|n| format!("{}", n))
                                .collect::<Vec<String>>()
                                .join(" ")
                        );
                        if let Some(r) = rest {
                            l.push_str(&format!(" . {}", r));
                        }
                        l.push_str(")");
                        l
                    };
                    let mut rv = format!("(for {} {}", left, right);
                    for node in body {
                        for line in format!("{}", node.borrow()).split("\n") {
                            rv.push_str(&format!("\n{}{}", indent(1), line));
                        }
                    }
                    rv.push_str(")");
                    rv
                }
                Node::Function {
                    name, args, body, ..
                } => {
                    let mut rv = format!("(function ({}", name);
                    if args.len() > 0 {
                        let mut args = args
                            .iter()
                            .map(|n| format!("{}", n))
                            .collect::<Vec<String>>();
                        let last = args.len() - 1;
                        if args[last] == "..." {
                            args[last] = ". ...".to_string();
                        }
                        rv.push_str(&format!(" {}", args.join(" ")));
                    }
                    rv.push_str(")");
                    for node in body {
                        for line in format!("{}", node.borrow()).split("\n") {
                            rv.push_str(&format!("\n{}{}", indent(1), line));
                        }
                    }
                    rv.push_str(")");
                    rv
                }
                Node::If {
                    cond,
                    body,
                    elseifs,
                    else_,
                    ..
                } => {
                    let mut rv = format!("(if {}", cond);
                    for node in body {
                        for line in format!("{}", node.borrow()).split("\n") {
                            rv.push_str(&format!("\n{}{}", indent(1), line));
                        }
                    }
                    for elseif in elseifs {
                        if let Node::ElseIf {
                            ref mut cond,
                            ref mut body,
                            ..
                        } = *elseif.borrow_mut()
                        {
                            rv.push_str(&format!("\n elseif {}", cond));
                            for node in body {
                                for line in format!("{}", node.borrow()).split("\n") {
                                    rv.push_str(&format!("\n{}{}", indent(1), line));
                                }
                            }
                        }
                    }
                    if let Some(e) = else_ {
                        if let Node::Else { ref mut body, .. } = *e.borrow_mut() {
                            rv.push_str("\n else");
                            for node in body {
                                for line in format!("{}", node.borrow()).split("\n") {
                                    rv.push_str(&format!("\n{}{}", indent(1), line));
                                }
                            }
                        }
                    }
                    rv.push_str(")");
                    rv
                }
                Node::Lambda { args, expr, .. } => format!(
                    "(lambda ({}) {})",
                    args.iter()
                        .map(|n| format!("{}", n))
                        .collect::<Vec<String>>()
                        .join(" "),
                    expr
                ),
                Node::Let {
                    var,
                    list,
                    rest,
                    right,
                    op,
                    ..
                } => {
                    let left = if let Some(v) = var {
                        format!("{}", v)
                    } else {
                        let mut l = format!(
                            "({}",
                            list.iter()
                                .map(|n| format!("{}", n))
                                .collect::<Vec<String>>()
                                .join(" ")
                        );
                        if let Some(r) = rest {
                            l.push_str(&format!(" . {}", r));
                        }
                        l.push_str(")");
                        l
                    };
                    format!("(let {} {} {})", op, left, right)
                }
                Node::List { items, .. } => {
                    if items.len() == 0 {
                        "(list)".to_string()
                    } else {
                        display_with_list("list", items)
                    }
                }
                Node::LockVar { depth, list, .. } => {
                    if let Some(d) = depth {
                        display_with_list(&format!("lockvar {}", d), list)
                    } else {
                        display_with_list("lockvar", list)
                    }
                }
                Node::Minus { left, .. } => display_left("-", left),
                Node::Multiply { left, right, .. } => display_lr("*", left, right),
                Node::Not { left, .. } => display_left("!", left),
                Node::Or { left, right, .. } => display_lr("||", left, right),
                Node::Plus { left, .. } => display_left("+", left),
                Node::Remainder { left, right, .. } => display_lr("%", left, right),
                Node::Return { left, .. } => {
                    if let Some(ref l) = left {
                        display_left("return", l)
                    } else {
                        "(return)".to_string()
                    }
                }
                Node::Shebang { value, .. } => format!("(#! \"{}\")", escape(value)),
                Node::Slice {
                    name, left, right, ..
                } => {
                    let r0 = match left {
                        Some(l) => format!("{}", l),
                        None => "nil".to_string(),
                    };
                    let r1 = match right {
                        Some(r) => format!("{}", r),
                        None => "nil".to_string(),
                    };
                    format!("(slice {} {} {})", name, r0, r1)
                }
                Node::Subscript { name, index, .. } => display_lr("subscript", name, index),
                Node::Subtract { left, right, .. } => display_lr("-", left, right),
                Node::Ternary {
                    cond, left, right, ..
                } => display_lr(&format!("?: {}", cond), left, right),
                Node::Throw { err, .. } => display_left("throw", err),
                Node::TopLevel { body, .. } => format!(
                    "{}",
                    body.iter()
                        .map(|n| format!("{}", n.borrow()))
                        .collect::<Vec<String>>()
                        .join("\n")
                ),
                Node::Try {
                    body,
                    catches,
                    finally,
                    ..
                } => {
                    let mut rv = String::from("(try");
                    for node in body {
                        for line in format!("{}", node.borrow()).split("\n") {
                            rv.push_str(&format!("\n{}{}", indent(1), line));
                        }
                    }
                    for catch in catches {
                        if let Node::Catch {
                            ref mut pattern,
                            ref mut body,
                            ..
                        } = *catch.borrow_mut()
                        {
                            if let Some(p) = pattern {
                                rv.push_str(&format!("\n catch /{}/", p));
                            } else {
                                rv.push_str("\n catch");
                            }
                            for node in body {
                                for line in format!("{}", node.borrow()).split("\n") {
                                    rv.push_str(&format!("\n{}{}", indent(1), line));
                                }
                            }
                        }
                    }
                    if let Some(f) = finally {
                        if let Node::Finally { ref mut body, .. } = *f.borrow_mut() {
                            rv.push_str("\n finally");
                            for node in body {
                                for line in format!("{}", node.borrow()).split("\n") {
                                    rv.push_str(&format!("\n{}{}", indent(1), line));
                                }
                            }
                        }
                    }
                    rv.push_str(")");
                    rv
                }
                Node::Unlet { list, .. } => display_with_list("unlet", list),
                Node::UnlockVar { depth, list, .. } => {
                    if let Some(d) = depth {
                        display_with_list(&format!("unlockvar {}", d), list)
                    } else {
                        display_with_list("unlockvar", list)
                    }
                }
                Node::While { cond, body, .. } => {
                    let mut rv = format!("(while {}", cond);
                    for node in body {
                        for line in format!("{}", node.borrow()).split("\n") {
                            rv.push_str(&format!("\n{}{}", indent(1), line));
                        }
                    }
                    rv.push_str(")");
                    rv
                }
                _ => format!("{:?}", self),
            }
        )
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
            let pos = token.pos;
            let cond = Box::new(left);
            let left_side = Box::new(self.parse_expr1()?);
            token = self.tokenizer.get()?;
            if token.kind != TokenKind::Colon {
                return self.token_err(token);
            }
            let right = Box::new(self.parse_expr1()?);
            let node = Node::Ternary {
                pos,
                cond,
                left: left_side,
                right,
            };
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
                let node = Node::Or {
                    pos: token.pos,
                    left: Box::new(left),
                    right: Box::new(self.parse_expr3()?),
                };
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
                let node = Node::And {
                    pos: token.pos,
                    left: Box::new(left),
                    right: Box::new(self.parse_expr4()?),
                };
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
        let cursor = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        let left_side = Box::new(left.clone());
        let op = match token.kind {
            TokenKind::EqEq => "==",
            TokenKind::EqEqCI => "==?",
            TokenKind::EqEqCS => "==#",
            TokenKind::NotEq => "!=",
            TokenKind::NotEqCI => "!=?",
            TokenKind::NotEqCS => "!=#",
            TokenKind::GT => ">",
            TokenKind::GTCI => ">?",
            TokenKind::GTCS => ">#",
            TokenKind::GTEq => ">=",
            TokenKind::GTEqCI => ">=?",
            TokenKind::GTEqCS => ">=#",
            TokenKind::LT => "<",
            TokenKind::LTCI => "<?",
            TokenKind::LTCS => "<#",
            TokenKind::LTEq => "<=",
            TokenKind::LTEqCI => "<=?",
            TokenKind::LTEqCS => "<=#",
            TokenKind::Match => "=~",
            TokenKind::MatchCI => "=~?",
            TokenKind::MatchCS => "=~#",
            TokenKind::NoMatch => "!~",
            TokenKind::NoMatchCI => "!~?",
            TokenKind::NoMatchCS => "!~#",
            TokenKind::Is => "is",
            TokenKind::IsCI => "is?",
            TokenKind::IsCS => "is#",
            TokenKind::IsNot => "isnot",
            TokenKind::IsNotCI => "isnot?",
            TokenKind::IsNotCS => "isnot#",
            _ => {
                self.reader.borrow_mut().seek_set(cursor);
                return Ok(left);
            }
        }.to_string();
        let node = Node::BinOp {
            pos,
            op,
            left: left_side,
            right: Box::new(self.parse_expr5()?),
        };
        left = node;
        Ok(left)
    }

    fn parse_expr5(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr6()?;
        loop {
            let cursor = self.reader.borrow().tell();
            let token = self.tokenizer.get()?;
            let pos = token.pos;
            let left_side = Box::new(left.clone());
            let node = match token.kind {
                TokenKind::Plus => Node::Add {
                    pos,
                    left: left_side,
                    right: Box::new(self.parse_expr6()?),
                },
                TokenKind::Minus => Node::Subtract {
                    pos,
                    left: left_side,
                    right: Box::new(self.parse_expr6()?),
                },
                TokenKind::Dot => Node::Concat {
                    pos,
                    left: left_side,
                    right: Box::new(self.parse_expr6()?),
                },
                _ => {
                    self.reader.borrow_mut().seek_set(cursor);
                    break;
                }
            };
            left = node;
        }
        Ok(left)
    }

    fn parse_expr6(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr7()?;
        loop {
            let cursor = self.reader.borrow().tell();
            let token = self.tokenizer.get()?;
            let pos = token.pos;
            let left_side = Box::new(left.clone());
            let node = match token.kind {
                TokenKind::Star => Node::Multiply {
                    pos,
                    left: left_side,
                    right: Box::new(self.parse_expr7()?),
                },
                TokenKind::Slash => Node::Divide {
                    pos,
                    left: left_side,
                    right: Box::new(self.parse_expr7()?),
                },
                TokenKind::Percent => Node::Remainder {
                    pos,
                    left: left_side,
                    right: Box::new(self.parse_expr7()?),
                },
                _ => {
                    self.reader.borrow_mut().seek_set(cursor);
                    break;
                }
            };
            left = node;
        }
        Ok(left)
    }

    fn parse_expr7(&mut self) -> Result<Node, ParseError> {
        let cursor = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        let node = match token.kind {
            TokenKind::Not => Node::Not {
                pos,
                left: Box::new(self.parse_expr7()?),
            },
            TokenKind::Minus => Node::Minus {
                pos,
                left: Box::new(self.parse_expr7()?),
            },
            TokenKind::Plus => Node::Plus {
                pos,
                left: Box::new(self.parse_expr7()?),
            },
            _ => {
                self.reader.borrow_mut().seek_set(cursor);
                return self.parse_expr8();
            }
        };
        Ok(node)
    }

    fn parse_expr8(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr9()?;
        loop {
            let cursor = self.reader.borrow().tell();
            let c = self.reader.borrow().peek();
            let mut token = self.tokenizer.get()?;
            if !iswhite(&c) && token.kind == TokenKind::SqOpen {
                let npos = token.pos;
                if self.tokenizer.peek()?.kind == TokenKind::Colon {
                    self.tokenizer.get()?;
                    let pos = npos;
                    let name = Box::new(left.clone());
                    let left_side = None;
                    token = self.tokenizer.peek()?;
                    let right = if token.kind != TokenKind::SqClose {
                        Some(Box::new(self.parse_expr1()?))
                    } else {
                        None
                    };
                    let node = Node::Slice {
                        pos,
                        name,
                        left: left_side,
                        right,
                    };
                    token = self.tokenizer.get()?;
                    if token.kind != TokenKind::SqClose {
                        return self.token_err(token);
                    }
                    left = node;
                } else {
                    let expr = self.parse_expr1()?;
                    if self.tokenizer.peek()?.kind == TokenKind::Colon {
                        self.tokenizer.get()?;
                        let pos = npos;
                        let name = Box::new(left.clone());
                        let left_side = Some(Box::new(expr));
                        token = self.tokenizer.peek()?;
                        let right = if token.kind != TokenKind::SqClose {
                            Some(Box::new(self.parse_expr1()?))
                        } else {
                            None
                        };
                        let node = Node::Slice {
                            pos,
                            name,
                            left: left_side,
                            right,
                        };
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                        left = node;
                    } else {
                        let node = Node::Subscript {
                            pos: npos,
                            name: Box::new(left.clone()),
                            index: Box::new(expr),
                        };
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                        left = node;
                    }
                }
            } else if token.kind == TokenKind::POpen {
                let pos = token.pos;
                let name = Box::new(left.clone());
                let mut args = vec![];
                if self.tokenizer.peek()?.kind == TokenKind::PClose {
                    self.tokenizer.get()?;
                } else {
                    loop {
                        args.push(Box::new(self.parse_expr1()?));
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
                if args.len() > MAX_FUNC_ARGS {
                    return Err(ParseError {
                        msg: "E740: Too many arguments for function".to_string(),
                        pos,
                    });
                }
                let node = Node::Call { pos, name, args };
                left = node;
            } else if !iswhite(&c) && token.kind == TokenKind::Dot {
                match self.parse_dot(token, left.clone()) {
                    Some(node) => {
                        left = node;
                    }
                    None => {
                        self.reader.borrow_mut().seek_set(cursor);
                        break;
                    }
                }
            } else {
                self.reader.borrow_mut().seek_set(cursor);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr9(&mut self) -> Result<Node, ParseError> {
        let cursor = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::Number => Node::Number {
                pos,
                value: token.value,
            },
            TokenKind::DQuote => {
                self.reader.borrow_mut().seek_set(cursor);
                Node::String {
                    pos,
                    value: format!("\"{}\"", self.tokenizer.get_dstring()?),
                }
            }
            TokenKind::SQuote => {
                self.reader.borrow_mut().seek_set(cursor);
                Node::String {
                    pos,
                    value: format!("\'{}\'", self.tokenizer.get_sstring()?),
                }
            }
            TokenKind::SqOpen => {
                let token = self.tokenizer.peek()?;
                let mut items = vec![];
                if token.kind == TokenKind::SqClose {
                    self.tokenizer.get()?;
                } else {
                    loop {
                        items.push(Box::new(self.parse_expr1()?));
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
                Node::List { pos, items }
            }
            TokenKind::COpen => {
                let savepos = self.reader.borrow().tell();
                let mut token = self.tokenizer.get()?;
                let mut is_lambda = token.kind == TokenKind::Arrow;
                if !is_lambda && token.kind != TokenKind::SQuote && token.kind != TokenKind::DQuote
                {
                    let token2 = self.tokenizer.peek()?;
                    is_lambda = token2.kind == TokenKind::Arrow || token2.kind == TokenKind::Comma;
                }
                let mut fallback = false;
                if is_lambda {
                    let mut args = vec![];
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
                                let varnode = Node::Identifier {
                                    pos: token.pos,
                                    value: token.value,
                                };
                                let maybe_comma = self.tokenizer.peek()?.kind;
                                if iswhite(&self.reader.borrow().peek())
                                    && maybe_comma == TokenKind::Comma
                                {
                                    return Err(ParseError {
                                        msg: String::from(
                                            "E475: invalid argument: White space is not allowed before comma"
                                        ),
                                        pos: self.reader.borrow().getpos()
                                    });
                                }
                                token = self.tokenizer.get()?;
                                args.push(Box::new(varnode));
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
                                let varnode = Node::Identifier {
                                    pos: token.pos,
                                    value: token.value,
                                };
                                args.push(Box::new(varnode));
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
                        let expr = Box::new(self.parse_expr1()?);
                        let node = Node::Lambda { pos, args, expr };
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::CClose {
                            return self.token_err(token);
                        }
                        return Ok(node);
                    }
                }
                let mut items = vec![];
                self.reader.borrow_mut().seek_set(savepos);
                token = self.tokenizer.peek()?;
                if token.kind == TokenKind::CClose {
                    self.tokenizer.get()?;
                    return Ok(Node::Dict { pos, items });
                }
                loop {
                    let key = self.parse_expr1()?;
                    token = self.tokenizer.get()?;
                    if token.kind == TokenKind::CClose {
                        // premature closing of dict, e.g. "let d = { 'foo': }"
                        if items.len() > 0 {
                            return self.token_err(token);
                        }
                        self.reader.borrow_mut().seek_set(cursor);
                        return self.parse_identifier();
                    }
                    if token.kind != TokenKind::Colon {
                        return self.token_err(token);
                    }
                    let val = self.parse_expr1()?;
                    items.push((Box::new(key), Box::new(val)));
                    token = self.tokenizer.get()?;
                    if token.kind == TokenKind::Comma {
                        if self.tokenizer.peek()?.kind == TokenKind::CClose {
                            self.tokenizer.get()?;
                            break;
                        }
                    } else if token.kind == TokenKind::CClose {
                        break;
                    } else {
                        return self.token_err(token);
                    }
                }
                Node::Dict { pos, items }
            }
            TokenKind::POpen => {
                let node = self.parse_expr1()?;
                let token = self.tokenizer.get()?;
                if token.kind != TokenKind::PClose {
                    return self.token_err(token);
                }
                node
            }
            TokenKind::Option => Node::Option {
                pos,
                value: token.value,
            },
            _ if token.kind == TokenKind::LT
                && self.reader.borrow().peekn(4).eq_ignore_ascii_case("SID>") =>
            {
                self.reader.borrow_mut().seek_set(cursor);
                self.parse_identifier()?
            }
            TokenKind::Identifier
            | TokenKind::Is
            | TokenKind::IsCS
            | TokenKind::IsNot
            | TokenKind::IsNotCS => {
                self.reader.borrow_mut().seek_set(cursor);
                self.parse_identifier()?
            }
            TokenKind::Env => Node::Env {
                pos,
                value: token.value,
            },
            TokenKind::Reg => Node::Reg {
                pos,
                value: token.value,
            },
            _ => {
                return self.token_err(token);
            }
        })
    }

    fn parse_identifier(&mut self) -> Result<Node, ParseError> {
        self.reader.borrow_mut().skip_white();
        let pos = self.reader.borrow().getpos();
        let mut curly_parts = self.parse_curly_parts()?;
        let mut node = None;
        if curly_parts.len() == 1 {
            if let Node::CurlyNamePart { ref mut value, .. } = curly_parts[0] {
                node = Some(Node::Identifier {
                    pos,
                    value: value.to_string(),
                });
            }
        }
        if node.is_none() {
            node = Some(Node::CurlyName {
                pos,
                pieces: curly_parts
                    .into_iter()
                    .map(|n| Box::new(n))
                    .collect::<Vec<Box<Node>>>(),
            });
        }
        Ok(node.unwrap())
    }

    fn parse_curly_parts(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut curly_parts = vec![];
        let c = self.reader.borrow().peek();
        let pos = self.reader.borrow().getpos();
        if c == "<" && self.reader.borrow().peekn(5).eq_ignore_ascii_case("<SID>") {
            let name = self.reader.borrow_mut().getn(5);
            curly_parts.push(Node::CurlyNamePart { pos, value: name });
        }
        loop {
            let c = self.reader.borrow().peek();
            if isnamec(&c) {
                let pos = self.reader.borrow().getpos();
                let name = self.reader.borrow_mut().read_name();
                curly_parts.push(Node::CurlyNamePart { pos, value: name });
            } else if c == "{" {
                self.reader.borrow_mut().get();
                let pos = self.reader.borrow().getpos();
                curly_parts.push(Node::CurlyNameExpr {
                    pos,
                    expr: Box::new(self.parse_expr1()?),
                });
                self.reader.borrow_mut().skip_white();
                let c = self.reader.borrow().peek();
                if c != "}" {
                    return Err(ParseError {
                        msg: format!("unexpected token: {}", c),
                        pos: self.reader.borrow().getpos(),
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
        match &left {
            Node::Identifier { .. }
            | Node::CurlyName { .. }
            | Node::Dict { .. }
            | Node::Subscript { .. }
            | Node::Call { .. }
            | Node::Dot { .. } => (),
            _ => return None,
        }
        if !iswordc(&self.reader.borrow().peek()) {
            return None;
        }
        let pos = self.reader.borrow().getpos();
        let name = self.reader.borrow_mut().read_word();
        if isnamec(&self.reader.borrow().peek()) {
            return None;
        }
        let right = Box::new(Node::Identifier { pos, value: name });
        Some(Node::Dot {
            pos: token.pos,
            left: Box::new(left),
            right,
        })
    }

    pub fn parse_lv(&mut self) -> Result<Node, ParseError> {
        self.parse_lv8()
    }

    fn parse_lv8(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_lv9()?;
        loop {
            let cursor = self.reader.borrow().tell();
            let c = self.reader.borrow().peek();
            let mut token = self.tokenizer.get()?;
            let node;
            if !iswhite(&c) && token.kind == TokenKind::SqOpen {
                let pos = token.pos;
                if self.tokenizer.peek()?.kind == TokenKind::Colon {
                    self.tokenizer.get()?;
                    let name = Box::new(left);
                    let left_side = None;
                    token = self.tokenizer.peek()?;
                    let right = if token.kind != TokenKind::SqClose {
                        Some(Box::new(self.parse_expr1()?))
                    } else {
                        None
                    };
                    node = Node::Slice {
                        pos,
                        name,
                        left: left_side,
                        right,
                    };
                    token = self.tokenizer.get()?;
                    if token.kind != TokenKind::SqClose {
                        return self.token_err(token);
                    }
                } else {
                    let right = self.parse_expr1()?;
                    if self.tokenizer.peek()?.kind == TokenKind::Colon {
                        self.tokenizer.get()?;
                        let name = Box::new(left);
                        token = self.tokenizer.peek()?;
                        let left_side = Some(Box::new(right));
                        let right_side = if token.kind != TokenKind::SqClose {
                            Some(Box::new(self.parse_expr1()?))
                        } else {
                            None
                        };
                        node = Node::Slice {
                            pos,
                            name,
                            left: left_side,
                            right: right_side,
                        };
                        token = self.tokenizer.get()?;
                        if token.kind != TokenKind::SqClose {
                            return self.token_err(token);
                        }
                    } else {
                        node = Node::Subscript {
                            pos,
                            name: Box::new(left),
                            index: Box::new(right),
                        };
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
                        self.reader.borrow_mut().seek_set(cursor);
                        break;
                    }
                }
            } else {
                self.reader.borrow_mut().seek_set(cursor);
                break;
            }
        }
        Ok(left)
    }

    fn parse_lv9(&mut self) -> Result<Node, ParseError> {
        let cursor = self.reader.borrow().tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::COpen | TokenKind::Identifier => {
                self.reader.borrow_mut().seek_set(cursor);
                let mut node = self.parse_identifier()?;
                if let Node::Identifier { ref mut value, .. } = node {
                    *value = token.value;
                };
                node
            }
            _ if token.kind == TokenKind::LT
                && self.reader.borrow().peekn(4).eq_ignore_ascii_case("SID>") =>
            {
                self.reader.borrow_mut().seek_set(cursor);
                let mut node = self.parse_identifier()?;
                if let Node::Identifier { ref mut value, .. } = node {
                    *value = token.value;
                };
                node
            }
            TokenKind::Option => Node::Option {
                pos,
                value: token.value,
            },
            TokenKind::Env => Node::Env {
                pos,
                value: token.value,
            },
            TokenKind::Reg => Node::Reg {
                pos,
                value: token.value,
            },
            _ => {
                return self.token_err(token);
            }
        })
    }
}
