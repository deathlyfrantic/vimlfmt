use super::Position;
use exarg::ExArg;
use std::fmt;

const INDENT: &str = "  ";

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

fn display_left<T: fmt::Display>(name: &str, left: T) -> String {
    format!("({} {})", name, left)
}

fn display_lr<T: fmt::Display>(name: &str, left: T, right: T) -> String {
    format!("({} {} {})", name, left, right)
}

fn display_with_list<T: fmt::Display>(name: &str, list: &[T]) -> String {
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
pub enum BinaryOpKind {
    Add,
    And,
    Concat,
    Divide,
    EqEq,
    EqEqCI,
    EqEqCS,
    GT,
    GTCI,
    GTCS,
    GTEq,
    GTEqCI,
    GTEqCS,
    Is,
    IsCI,
    IsCS,
    IsNot,
    IsNotCI,
    IsNotCS,
    LT,
    LTCI,
    LTCS,
    LTEq,
    LTEqCI,
    LTEqCS,
    Match,
    MatchCI,
    MatchCS,
    Multiply,
    NoMatch,
    NoMatchCI,
    NoMatchCS,
    NotEq,
    NotEqCI,
    NotEqCS,
    Or,
    Remainder,
    Subtract,
}

impl fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                BinaryOpKind::Add => "+",
                BinaryOpKind::And => "&&",
                BinaryOpKind::Concat => ".",
                BinaryOpKind::Divide => "/",
                BinaryOpKind::EqEq => "==",
                BinaryOpKind::EqEqCI => "==?",
                BinaryOpKind::EqEqCS => "==#",
                BinaryOpKind::GT => ">",
                BinaryOpKind::GTCI => ">?",
                BinaryOpKind::GTCS => ">#",
                BinaryOpKind::GTEq => ">=",
                BinaryOpKind::GTEqCI => ">=?",
                BinaryOpKind::GTEqCS => ">=#",
                BinaryOpKind::Is => "is",
                BinaryOpKind::IsCI => "is?",
                BinaryOpKind::IsCS => "is#",
                BinaryOpKind::IsNot => "isnot",
                BinaryOpKind::IsNotCI => "isnot?",
                BinaryOpKind::IsNotCS => "isnot#",
                BinaryOpKind::LT => "<",
                BinaryOpKind::LTCI => "<?",
                BinaryOpKind::LTCS => "<#",
                BinaryOpKind::LTEq => "<=",
                BinaryOpKind::LTEqCI => "<=?",
                BinaryOpKind::LTEqCS => "<=#",
                BinaryOpKind::Match => "=~",
                BinaryOpKind::MatchCI => "=~?",
                BinaryOpKind::MatchCS => "=~#",
                BinaryOpKind::Multiply => "*",
                BinaryOpKind::NoMatch => "!~",
                BinaryOpKind::NoMatchCI => "!~?",
                BinaryOpKind::NoMatchCS => "!~#",
                BinaryOpKind::NotEq => "!=",
                BinaryOpKind::NotEqCI => "!=?",
                BinaryOpKind::NotEqCS => "!=#",
                BinaryOpKind::Or => "||",
                BinaryOpKind::Remainder => "%",
                BinaryOpKind::Subtract => "-",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOpKind {
    Minus,
    Not,
    Plus,
}

impl fmt::Display for UnaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                UnaryOpKind::Minus => "-",
                UnaryOpKind::Not => "!",
                UnaryOpKind::Plus => "+",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Augroup {
        pos: Position,
        name: String,
    },
    Autocmd {
        pos: Position,
        ea: ExArg,
        group: String,
        events: Vec<String>,
        patterns: Vec<String>,
        nested: bool,
        body: Vec<Box<Node>>,
    },
    BinaryOp {
        pos: Position,
        op: BinaryOpKind,
        left: Box<Node>,
        right: Box<Node>,
    },
    BlankLine {
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
        body: Vec<Box<Node>>,
    },
    Comment {
        pos: Position,
        value: String,
        trailing: bool,
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
    Dot {
        pos: Position,
        left: Box<Node>,
        right: Box<Node>,
    },
    Echo {
        ea: ExArg,
        pos: Position,
        cmd: String, // echo, echoerr, echomsg, echon
        list: Vec<Box<Node>>,
    },
    EchoHl {
        ea: ExArg,
        pos: Position,
        value: String,
    },
    Else {
        ea: ExArg,
        pos: Position,
        body: Vec<Box<Node>>,
    },
    ElseIf {
        ea: ExArg,
        pos: Position,
        cond: Box<Node>,
        body: Vec<Box<Node>>,
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
        body: Vec<Box<Node>>,
    },
    For {
        ea: ExArg,
        pos: Position,
        var: Option<Box<Node>>,  // this is the x in "for x in something"
        list: Vec<Box<Node>>,    // this is the a, b in "for [a, b] in something"
        rest: Option<Box<Node>>, // this is the z in "for [a, b; z] in something" <- REAL SYNTAX :(
        right: Box<Node>,        // this is the something in "for x in something"
        body: Vec<Box<Node>>,
        end: Option<Box<Node>>,
    },
    Function {
        ea: ExArg,
        pos: Position,
        name: Box<Node>,
        args: Vec<Box<Node>>,
        body: Vec<Box<Node>>,
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
        elseifs: Vec<Box<Node>>,
        else_: Option<Box<Node>>,
        body: Vec<Box<Node>>,
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
    Mapping {
        pos: Position,
        ea: ExArg,
        command: String,
        left: String,
        right: String,
        right_expr: Option<Box<Node>>,
        attrs: Vec<String>,
    },
    Number {
        pos: Position,
        value: String,
    },
    Option {
        pos: Position,
        value: String,
    },
    ParenExpr {
        pos: Position,
        expr: Box<Node>,
    },
    Reg {
        pos: Position,
        value: String,
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
        body: Vec<Box<Node>>,
    },
    Try {
        ea: ExArg,
        pos: Position,
        body: Vec<Box<Node>>,
        catches: Vec<Box<Node>>,
        finally: Option<Box<Node>>,
        end: Option<Box<Node>>,
    },
    UnaryOp {
        pos: Position,
        op: UnaryOpKind,
        right: Box<Node>,
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
        body: Vec<Box<Node>>,
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

    pub fn has_body(node: &Node) -> bool {
        match node {
            Node::Catch { .. }
            | Node::Else { .. }
            | Node::ElseIf { .. }
            | Node::Finally { .. }
            | Node::For { .. }
            | Node::Function { .. }
            | Node::If { .. }
            | Node::TopLevel { .. }
            | Node::Try { .. }
            | Node::While { .. } => true,
            _ => false,
        }
    }
}

fn format_body(body: &[Box<Node>]) -> String {
    let mut rv = String::new();
    for node in body {
        if let Node::BlankLine { .. } = node.as_ref() {
            continue;
        }
        for line in format!("{}", node).split("\n") {
            rv.push_str(&format!("\n{}{}", INDENT, line));
        }
    }
    rv
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Node::Augroup { name, .. } => {
                    if name.len() == 0 {
                        "(augroup)".to_string()
                    } else {
                        format!(
                            "(augroup {})",
                            name.replace("|", "\\|").replace("\"", "\\\"")
                        )
                    }
                }
                Node::Autocmd {
                    group,
                    events,
                    patterns,
                    nested,
                    body,
                    ..
                } => {
                    let mut rv = String::from("(autocmd");
                    if group.len() > 0 {
                        rv.push_str(&format!(" {}", group));
                    }
                    if events.len() > 0 {
                        let mut events = events.clone();
                        events.sort();
                        rv.push_str(&format!(" {}", events.join(",")));
                    }
                    if patterns.len() > 0 {
                        let mut patterns = patterns.clone();
                        patterns.sort();
                        rv.push_str(&format!(" {}", patterns.join(",")));
                    }
                    if *nested {
                        rv.push_str(" nested");
                    }
                    if body.len() > 0 {
                        rv.push_str(&format!(
                            " {}",
                            body.iter()
                                .map(|n| format!("{}", n))
                                .collect::<Vec<String>>()
                                .join(" ")
                        ));
                    }
                    rv.push(')');
                    rv
                }
                Node::BinaryOp {
                    op, left, right, ..
                } => {
                    let op = match op {
                        BinaryOpKind::Concat => "concat".to_string(),
                        _ => format!("{}", op),
                    };
                    format!("({} {} {})", op, left, right)
                }
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
                Node::Dot { left, right, .. } => display_lr("dot", left, right),
                Node::Echo { cmd, list, .. } => display_with_list(&cmd, &list),
                Node::EchoHl { value, .. } => format!("(echohl \"{}\")", escape(value)),
                Node::ExCall { left, .. } => display_left("call", left),
                Node::ExCmd { value, .. } => match value.as_str() {
                    "break" | "continue" => format!("({})", value),
                    _ => format!("(excmd \"{}\")", escape(value)),
                },
                Node::Execute { list, .. } => display_with_list("execute", &list),
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
                    rv.push_str(&format_body(body.as_slice()));
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
                    rv.push_str(&format_body(body.as_slice()));
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
                    rv.push_str(&format_body(body.as_slice()));
                    for elseif in elseifs {
                        let mut elseif = *elseif.clone();
                        if let Node::ElseIf {
                            ref mut cond,
                            ref mut body,
                            ..
                        } = elseif
                        {
                            rv.push_str(&format!("\n elseif {}", cond));
                            rv.push_str(&format_body(body.as_slice()));
                        }
                    }
                    if let Some(e) = else_ {
                        let mut e = *e.clone();
                        if let Node::Else { ref mut body, .. } = e {
                            rv.push_str("\n else");
                            rv.push_str(&format_body(body.as_slice()));
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
                        display_with_list("list", &items)
                    }
                }
                Node::LockVar { depth, list, .. } => {
                    if let Some(d) = depth {
                        display_with_list(&format!("lockvar {}", d), &list)
                    } else {
                        display_with_list("lockvar", &list)
                    }
                }
                Node::Mapping {
                    command,
                    left,
                    right,
                    right_expr,
                    ..
                } => {
                    let mut rv = format!("({}", command);
                    if left.len() > 0 {
                        rv.push_str(&format!(" {}", left));
                        if let Some(re) = right_expr {
                            rv.push_str(&format!(" {}", re));
                        } else if right.len() > 0 {
                            rv.push_str(&format!(" {}", right));
                        }
                    }
                    rv.push(')');
                    rv
                }
                Node::ParenExpr { expr, .. } => format!("{}", expr),
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
                Node::Ternary {
                    cond, left, right, ..
                } => display_lr(&format!("?: {}", cond), left, right),
                Node::Throw { err, .. } => display_left("throw", err),
                Node::TopLevel { body, .. } => format!(
                    "{}",
                    body.iter()
                        .filter_map(|n| if let Node::BlankLine { .. } = n.as_ref() {
                            None
                        } else {
                            Some(format!("{}", n))
                        })
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
                    rv.push_str(&format_body(body.as_slice()));
                    for catch in catches {
                        let mut catch = *catch.clone();
                        if let Node::Catch {
                            ref mut pattern,
                            ref mut body,
                            ..
                        } = catch
                        {
                            if let Some(p) = pattern {
                                rv.push_str(&format!("\n catch /{}/", p));
                            } else {
                                rv.push_str("\n catch");
                            }
                            rv.push_str(&format_body(body.as_slice()));
                        }
                    }
                    if let Some(f) = finally {
                        let mut f = *f.clone();
                        if let Node::Finally { ref mut body, .. } = f {
                            rv.push_str("\n finally");
                            rv.push_str(&format_body(body.as_slice()));
                        }
                    }
                    rv.push_str(")");
                    rv
                }
                Node::UnaryOp { op, right, .. } => display_left(&format!("{}", op), right),
                Node::Unlet { list, .. } => display_with_list("unlet", &list),
                Node::UnlockVar { depth, list, .. } => {
                    if let Some(d) = depth {
                        display_with_list(&format!("unlockvar {}", d), &list)
                    } else {
                        display_with_list("unlockvar", &list)
                    }
                }
                Node::While { cond, body, .. } => {
                    let mut rv = format!("(while {}", cond);
                    rv.push_str(&format_body(body.as_slice()));
                    rv.push_str(")");
                    rv
                }
                _ => format!("{:?}", self),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(&escape("foo"), "foo");
        assert_eq!(&escape(r#"backslash \"#), r#"backslash \\"#);
        assert_eq!(&escape(r#"quote ""#), r#"quote \""#);
        assert_eq!(&escape("\r"), "\\r");
    }

    #[test]
    fn test_display_left() {
        assert_eq!(&display_left("foo", "bar"), "(foo bar)");
    }

    #[test]
    fn test_display_lr() {
        assert_eq!(&display_lr("foo", "bar", "baz"), "(foo bar baz)");
    }

    #[test]
    fn test_display_with_list() {
        assert_eq!(
            &display_with_list("foo", &["bar", "baz", "quux"]),
            "(foo bar baz quux)"
        );
    }

    #[test]
    fn test_node_is_for() {
        let for_node = Node::For {
            ea: ExArg::new(),
            pos: Position::empty(),
            var: None,
            list: vec![],
            rest: None,
            right: Box::new(Node::ExCmd {
                pos: Position::empty(),
                ea: ExArg::new(),
                value: "break".to_string(),
            }),
            body: vec![],
            end: None,
        };
        let not_for_node = Node::ExCmd {
            pos: Position::empty(),
            ea: ExArg::new(),
            value: "break".to_string(),
        };
        assert!(Node::is_for(&for_node));
        assert!(!Node::is_for(&not_for_node));
    }

    #[test]
    fn test_node_is_function() {
        let function_node = Node::Function {
            ea: ExArg::new(),
            pos: Position::empty(),
            name: Box::new(Node::ExCmd {
                pos: Position::empty(),
                ea: ExArg::new(),
                value: "break".to_string(),
            }),
            args: vec![],
            body: vec![],
            attrs: vec![],
            end: None,
        };
        let not_function_node = Node::ExCmd {
            pos: Position::empty(),
            ea: ExArg::new(),
            value: "break".to_string(),
        };
        assert!(Node::is_function(&function_node));
        assert!(!Node::is_function(&not_function_node));
    }

    #[test]
    fn test_node_is_while() {
        let while_node = Node::While {
            ea: ExArg::new(),
            pos: Position::empty(),
            body: vec![],
            cond: Box::new(Node::ExCmd {
                pos: Position::empty(),
                ea: ExArg::new(),
                value: "break".to_string(),
            }),
            end: None,
        };
        let not_while_node = Node::ExCmd {
            pos: Position::empty(),
            ea: ExArg::new(),
            value: "break".to_string(),
        };
        assert!(Node::is_while(&while_node));
        assert!(!Node::is_while(&not_while_node));
    }

    #[test]
    fn test_has_body() {
        let while_node = Node::While {
            ea: ExArg::new(),
            pos: Position::empty(),
            body: vec![],
            cond: Box::new(Node::ExCmd {
                pos: Position::empty(),
                ea: ExArg::new(),
                value: "break".to_string(),
            }),
            end: None,
        };
        let break_node = Node::ExCmd {
            pos: Position::empty(),
            ea: ExArg::new(),
            value: "break".to_string(),
        };
        assert!(Node::has_body(&while_node));
        assert!(!Node::has_body(&break_node));
    }
}
