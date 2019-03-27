use super::Position;
use crate::modifier::Modifier;
use std::fmt;

const INDENT: &str = "  ";

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\r', "\\r")
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

/// The operation kind in a Node::BinaryOp node.
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOpKind {
    /// Addition (`+`)
    Add,
    /// And (`&&`)
    And,
    /// Concatentation (`.` - Note that this parser cannot 100% distinguish between dictionary
    /// access and concatenation as VimL uses the dot token for both.)
    Concat,
    /// Division (`/`)
    Divide,
    /// Equals (`==`)
    EqEq,
    /// Case-insensitive equals (`==?`)
    EqEqCI,
    /// Case-sensitive equals (`==#`)
    EqEqCS,
    /// Greater-than (`>`)
    GT,
    /// Case-insensitive greater-than (`>?`)
    GTCI,
    /// Case-sensitive greater-than (`>#`)
    GTCS,
    /// Greater-than or equals (`>=`)
    GTEq,
    /// Case-insensitive greater-than or equals (`>=?`)
    GTEqCI,
    /// Case-sensitive greater-than or equals (`>=#`)
    GTEqCS,
    /// Is (same instance) (`is`)
    Is,
    /// Case-instance is (same instance) (`is?`)
    IsCI,
    /// Case-sensitive is (same instance) (`is#`)
    IsCS,
    /// Is not (same instance) (`isnot`)
    IsNot,
    /// Case-insensitive is not (same instance) (`isnot?`)
    IsNotCI,
    /// Case-sensitive is not (same instance) (`isnot#`)
    IsNotCS,
    /// Less-than (`<`)
    LT,
    /// Case-insensitive less-than (`<?`)
    LTCI,
    /// Case-sensitive less-than (`<#`)
    LTCS,
    /// Less-than or equals (`<=`)
    LTEq,
    /// Case-insensitive less-than or equals (`<=?`)
    LTEqCI,
    /// Case-sensitive less-than or equals (`<=#`)
    LTEqCS,
    /// Regexp matches (`=~`)
    Match,
    /// Case-insensitive regexp matches (`=~?`)
    MatchCI,
    /// Case-sensitive regexp matches (`=~#`)
    MatchCS,
    /// Multiplication (`*`)
    Multiply,
    /// Regexp does not match (`!~`)
    NoMatch,
    /// Case-insensitive regexp does not match (`!~?`)
    NoMatchCI,
    /// Case-sensitive regexp does not match (`!~#`)
    NoMatchCS,
    /// Does not equal (`!=`)
    NotEq,
    /// Case-insensitive does not equal (`!=?`)
    NotEqCI,
    /// Case-sensitive does not equal (`!=#`)
    NotEqCS,
    /// Or (`||`)
    Or,
    /// Modulo (`%`)
    Remainder,
    /// Subtraction (`-`)
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

/// The operation kind in a Node::UnaryOp node.
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOpKind {
    /// Minus (`-`)
    Minus,
    /// Bang (`!`)
    Not,
    /// Plus (`+`)
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

/// A single AST node. All variants have an inner struct containing data specific to the node.
/// Every variant has a `pos` member (a [Position](struct.Position.html) struct) that represents
/// the position of the node in the original source. Many variants have a `mods` vector which
/// contains zero or more [Modifier](struct.Modifier.html)s.
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    /// An autocommand group
    Augroup {
        pos: Position,
        /// The name of the group. Vim allows almost anything in this (including spaces!).
        name: String,
    },
    /// An autocommand
    Autocmd {
        pos: Position,
        mods: Vec<Modifier>,
        /// Whether this command was invoked with a bang (`!`).
        bang: bool,
        /// The group of this autocommand, if it is specified in the command. If it is not
        /// specified in the command, this is an empty string (`""`). For example:
        ///
        /// In
        /// ```text
        /// autocmd my-group BufEnter * echo "foo"
        /// ```
        /// this is `"my-group"`.
        ///
        /// In
        /// ```text
        /// augroup my-group
        ///   autocmd BufEnter * echo "foo"
        /// augroup END
        /// ```
        /// this is `""`.
        group: String,
        /// A vector of the events that will cause this command to be executed. Only valid events
        /// will be included (invalid events cause [ParseError](struct.ParseError.html)s). In
        /// ```text
        /// autocmd BufNewFile,BufReadPost * echo "foo"
        /// ```
        /// this is `["BufNewFile", "BufReadPost"]`.
        events: Vec<String>,
        /// A vector of patterns that must match for this command to be executed. In
        /// ```text
        /// autocmd BufReadPost *.foo,*.bar echo "foo"
        /// ```
        /// this is `["*.foo", "*.bar"]`.
        patterns: Vec<String>,
        /// Whether the command to be executed should cause other autocommands to be fired. In
        /// ```text
        /// autocmd FileChangedShell *.c nested e!
        /// ```
        /// this is `true`.
        nested: bool,
        /// The commands that will be executed when one of the events occurs and one of the
        /// patterns is matched.
        body: Vec<Box<Node>>,
    },
    /// An operation with two atoms
    BinaryOp {
        pos: Position,
        /// The kind of operation (see [BinaryOpKind](enum.BinaryOpKind.html)).
        op: BinaryOpKind,
        /// The node on the left side of the operation.
        left: Box<Node>,
        /// The node on the right side of the operation.
        right: Box<Node>,
    },
    /// An empty line. This kind of node can be ignored - it only exists for the VimL formatter
    /// which is the parent project of this parser.
    BlankLine { pos: Position },
    /// A function call. Not to be confused with [ExCall](#variant.ExCall).
    Call {
        pos: Position,
        /// The name of the function being called. This is _probably_ a single atom node (like an
        /// [Identifier](#variant.Identifier)), but doesn't have to be.
        name: Box<Node>,
        /// The arguments passed to the function.
        args: Vec<Box<Node>>,
    },
    /// A catch clause - will only show up in the `catches` member of a [Try](#variant.Try) node.
    Catch {
        pos: Position,
        mods: Vec<Modifier>,
        /// A pattern, if one exists - e.g. `/^Vim\%((\a\+)\)\=:E123/`.
        pattern: Option<String>,
        /// The commands in the body of the clause.
        body: Vec<Box<Node>>,
    },
    /// A colorscheme command
    Colorscheme {
        pos: Position,
        /// The name of the colorscheme, if one was provided.
        name: Option<String>,
    },
    /// A comment
    Comment {
        pos: Position,
        /// The content of the comment. Includes a leading space, so in this case:
        /// ```text
        /// " this is a comment
        /// ```
        /// it is `" this is a comment"`.
        value: String,
        /// If `true`, this comment was at the end of a line with another command on it.
        /// ```text
        /// " this is not a trailing comment
        /// let foo = 1 " this is a trailing comment
        /// ```
        trailing: bool,
    },
    /// An overall container for a "curly braces name" variable.
    CurlyName {
        pos: Position,
        /// The pieces that form the variable. These will be either
        /// [CurlyNameExpr](#variant.CurlyNameExpr) nodes or
        /// [CurlyNamePart](#variant.CurlyNamePart) nodes.
        pieces: Vec<Box<Node>>,
    },
    /// An expression in curly braces in a "curly braces name" variable.
    CurlyNameExpr {
        pos: Position,
        /// The expression within the braces. In `foo_{bar}_baz` this is `baz`.
        expr: Box<Node>,
    },
    /// A string piece of a "curly brances name" variable.
    CurlyNamePart {
        pos: Position,
        /// The string. In `foo_{bar}_baz`, `foo_` is one CurlyNamePart, `_baz` is another.
        value: String,
    },
    /// A delfunction command
    DelFunction {
        pos: Position,
        mods: Vec<Modifier>,
        /// Whether this command was invoked with a bang (`!`).
        bang: bool,
        /// The argument to the delfunction command. This is probably an
        /// [Identifier](#variant.Identifier), but doesn't have to be.
        left: Box<Node>,
    },
    /// A dictionary
    Dict {
        pos: Position,
        /// The items in the dictionary, as `(key, value)` tuples. The keys have to be either
        /// [String](#variant.String)s or [Number](#variant.Number)s. (Vim allows either, though
        /// numbers will be coerced into strings.)
        items: Vec<(Box<Node>, Box<Node>)>,
    },
    /// A dot operation - usually accessing an item in a dictionary. (Note that this parser cannot
    /// 100% distinguish between dictionary access and concatenation as VimL uses the dot token for
    /// both.)
    Dot {
        pos: Position,
        /// The node on the left side of the dot.
        left: Box<Node>,
        /// The node on the right side of the dot.
        right: Box<Node>,
    },
    /// An echo command
    Echo {
        pos: Position,
        mods: Vec<Modifier>,
        /// The particular command - either `echo`, `echoerr`, `echomsg`, or `echon`.
        cmd: String,
        /// The arguments passed to the echo command.
        list: Vec<Box<Node>>,
    },
    /// An echohl command
    EchoHl {
        pos: Position,
        mods: Vec<Modifier>,
        /// The name of the highlight group passed to the echohl command.
        value: String,
    },
    /// An else clause - will only show up in the `else_` member of an [If](#variant.If) node.
    Else {
        pos: Position,
        mods: Vec<Modifier>,
        /// The commands in the body of the clause.
        body: Vec<Box<Node>>,
    },
    /// An elseif clause - will only show up in the `elseifs` member of an [If](#variant.If) node.
    ElseIf {
        pos: Position,
        mods: Vec<Modifier>,
        /// The condition of the elseif.
        cond: Box<Node>,
        /// The commands in the body of the clause.
        body: Vec<Box<Node>>,
    },
    /// The end of a clause that requires and end statement. This will either be an `endif`,
    /// `endfor`, `endfunction`, `endtry`, or `endwhile`. This will only exist in the `end` member
    /// of an associated [If](#variant.If), [For](#variant.For), [Function](#variant.Function),
    /// [Try](#variant.Try), or [While](#variant.While) node.
    End { pos: Position, mods: Vec<Modifier> },
    /// An environment variable e.g. `$FOO`
    Env {
        pos: Position,
        /// The variable. The `$` is included.
        value: String,
    },
    /// The `call` command. Not to be confused with [Call](#variant.Call).
    ExCall {
        pos: Position,
        mods: Vec<Modifier>,
        /// The argument passed to the call command (probably a [Call](#variant.Call)).
        left: Box<Node>,
    },
    /// A general command which does not have a specific variant associated with it. This variant
    /// is kind of a "catch-all" for any commands that are not parsed specifically.
    ExCmd {
        pos: Position,
        mods: Vec<Modifier>,
        /// Whether this command was invoked with a bang (`!`).
        bang: bool,
        /// The literal text of the command - just the entire line from the original source.
        value: String,
    },
    /// An execute command
    Execute {
        pos: Position,
        mods: Vec<Modifier>,
        /// The arguments passed to the execute command.
        list: Vec<Box<Node>>,
    },
    /// A finally clause - will only show up in the `finally` member of a [Try](#variant.Try) node.
    Finally {
        pos: Position,
        mods: Vec<Modifier>,
        /// The commands in the body of the clause.
        body: Vec<Box<Node>>,
    },
    /// A for loop
    For {
        pos: Position,
        mods: Vec<Modifier>,
        /// The variable in the for statement, e.g. in `for x in something`, this is `x`.
        var: Option<Box<Node>>,
        /// If there are multiple variables in the for statement, this is a list of those
        /// variables, e.g. in `for [a, b] in something`, this list contains `a` and `b`.
        list: Vec<Box<Node>>,
        /// If there is a `{lastname}` variable in the for statement, this is that variable, e.g.
        /// in `for [a, b; z] in something`, this is `z`.
        rest: Option<Box<Node>>,
        /// The collection being iterated, e.g. in `for x in something`, this is `something`.
        right: Box<Node>,
        /// The commands in the body of the loop.
        body: Vec<Box<Node>>,
        /// The `endfor` - an [End](#variant.End) Node. Note that while this is an Option, it is a
        /// parse error for there not to be one - it's only an Option so the parser can parse the
        /// body of the clause before the `endfor` is found.
        end: Option<Box<Node>>,
    },
    /// A function definition
    Function {
        pos: Position,
        mods: Vec<Modifier>,
        /// Whether this command was invoked with a bang (`!`).
        bang: bool,
        /// The name of the function - probably an [Identifier](#variant.Identifier).
        name: Box<Node>,
        /// The parameters of the function.
        args: Vec<Box<Node>>,
        /// The commands in the body of the function.
        body: Vec<Box<Node>>,
        /// A list of attributes of the function - can contain any of "range", "abort", "dict", or
        /// "closure".
        attrs: Vec<String>,
        /// The `endfunction` - an [End](#variant.End) Node. Note that while this is an Option, it
        /// is a parse error for there not to be one - it's only an Option so the parser can parse
        /// the body of the function before the `endfunction` is found.
        end: Option<Box<Node>>,
    },
    /// An identifier (a variable, function name, etc)
    Identifier {
        pos: Position,
        /// The identifier
        value: String,
    },
    /// An if statement
    If {
        pos: Position,
        mods: Vec<Modifier>,
        /// The condition of the if.
        cond: Box<Node>,
        /// The elseif causes of the if.
        elseifs: Vec<Box<Node>>,
        /// The else clause of the if.
        else_: Option<Box<Node>>,
        /// The commands in the body of the if.
        body: Vec<Box<Node>>,
        /// The `endif` - an [End](#variant.End) Node. Note that while this is an Option, it is a
        /// parse error for there not to be one - it's only an Option so the parser can parse the
        /// body of the if before the `endif` is found.
        end: Option<Box<Node>>,
    },
    // A lambda function
    Lambda {
        pos: Position,
        /// The arguments of the lambda.
        args: Vec<Box<Node>>,
        /// The expression that is evaluated (equivalent to the body of a regular function).
        expr: Box<Node>,
    },
    /// A variable declaration
    Let {
        pos: Position,
        mods: Vec<Modifier>,
        /// The variable being defined, e.g. in `let x = something`, this is `x`.
        var: Option<Box<Node>>,
        /// If there are multiple variables in the let statement, this is a list of those
        /// variables, e.g. in `let [a, b] = something`, this list contains `a` and `b`.
        list: Vec<Box<Node>>,
        /// If there is a `{lastname}` variable in the let statement, this is that variable, e.g.
        /// in `let [a, b; z] = something`, this is `z`.
        rest: Option<Box<Node>>,
        /// The expression being assigned to the variables, e.g. in `let x = something`, this is
        /// `something`.
        right: Box<Node>,
        /// The operation of the let statement, e.g. in `let x += 1`, this is `+=`.
        op: String,
    },
    /// A list
    List {
        pos: Position,
        /// The items in the list.
        items: Vec<Box<Node>>,
    },
    /// A lockvar or unlockvar command
    LockVar {
        pos: Position,
        mods: Vec<Modifier>,
        /// Whether this command was invoked with a bang (`!`).
        bang: bool,
        /// The specific command - either `lockvar` or `unlockvar`
        cmd: String,
        /// The depth argument of the command, if there is one.
        depth: Option<usize>,
        /// The variables to lock or unlock.
        list: Vec<Box<Node>>,
    },
    /// A key mapping command
    Mapping {
        pos: Position,
        mods: Vec<Modifier>,
        /// The specific mapping command used, e.g. `nnoremap` or `xmap`.
        command: String,
        /// The left-hand side of the mapping (i.e. the key(s) to be mapped).
        left: String,
        /// The right-hand side of the mapping, if it is not an expression mapping.
        right: String,
        /// The right-hand side of the mapping, if it is an expression mapping.
        right_expr: Option<Box<Node>>,
        /// Any attributes of the mapping - could include "buffer", "nowait", "silent", "script",
        /// "unique" and/or "expr". (If it contains "expr", `right_expr` should be `Some`.
        attrs: Vec<String>,
    },
    /// A number
    Number {
        pos: Position,
        /// The number in its originally-parsed representation (which is why it's a string), e.g.
        /// if it started as `1e3`, this will be "1e3", not "1000".
        value: String,
    },
    /// An option variable, e.g. `&foo`
    Option {
        pos: Position,
        /// The variable. The `&` is included.
        value: String,
    },
    /// A parenthesized expression
    ParenExpr {
        pos: Position,
        /// The expression
        expr: Box<Node>,
    },
    /// A register variable, e.g. `@x`
    Reg {
        pos: Position,
        /// The register. The `@` is included.
        value: String,
    },
    /// A return statement
    Return {
        pos: Position,
        mods: Vec<Modifier>,
        /// The value to return, if there is one.
        left: Option<Box<Node>>,
    },
    /// A shebang (`#!`). Not common in VimL (it's a holdover from the Python library from which
    /// this parser was translated).
    Shebang {
        pos: Position,
        /// The literal text of the shebang. Does not include the `#!`, e.g. in `#!/bin/sh`, this
        /// is `"/bin/sh"`.
        value: String,
    },
    /// A slice
    Slice {
        pos: Position,
        /// The expression being sliced - generally an [Identifier](#variant.Identifier), but
        /// it doesn't have to be.
        name: Box<Node>,
        /// The left part of the slice, if it has one.
        left: Option<Box<Node>>,
        /// The right part of the slice, if it has one.
        right: Option<Box<Node>>,
    },
    /// A string - either single- or double-quoted
    String {
        pos: Position,
        /// The string. It includes the surrounding quotes.
        value: String,
    },
    /// A subscripted expression (e.g. `foo[1]`)
    Subscript {
        pos: Position,
        /// The expression being subscripted - generally an [Identifier](#variant.Identifier), but
        /// it doesn't have to be.
        name: Box<Node>,
        /// The subscript expression - generally a [Number](#variant.Number), but it doesn't have
        /// to be.
        index: Box<Node>,
    },
    /// A ternary expression (e.g. `condition ? foo : bar`)
    Ternary {
        pos: Position,
        /// The condition
        cond: Box<Node>,
        /// The expression evaluated if the condition is true.
        left: Box<Node>,
        /// The expression evaluated if the condition is false.
        right: Box<Node>,
    },
    /// A throw statement
    Throw {
        pos: Position,
        mods: Vec<Modifier>,
        /// The argument provided to the throw statement - generally a [String](#variant.String),
        /// but it doesn't have to be.
        err: Box<Node>,
    },
    /// The top level node returned from the [parse_file](fn.parse_file.html) and
    /// [parse_lines](fn.parse_lines.html) functions. There will only be one of these and its only
    /// purpose is to serve as a container for all of the statements in the VimL input.
    TopLevel {
        pos: Position,
        /// The statements of the input.
        body: Vec<Box<Node>>,
    },
    /// A try statement
    Try {
        pos: Position,
        mods: Vec<Modifier>,
        /// The commands in the body of the try.
        body: Vec<Box<Node>>,
        /// Any catch statements within the try. These will be [Catch](#variant.Catch)es.
        catches: Vec<Box<Node>>,
        /// A finally statement, if there is one. This will be a [FInally](#variant.Finally).
        finally: Option<Box<Node>>,
        /// The `endtry` - an [End](#variant.End) Node. Note that while this is an Option, it is a
        /// parse error for there not to be one - it's only an Option so the parser can parse the
        /// body of the try before the `endtry` is found.
        end: Option<Box<Node>>,
    },
    /// A unary operation
    UnaryOp {
        pos: Position,
        /// The operation kind
        op: UnaryOpKind,
        /// The expression being operated upon.
        right: Box<Node>,
    },
    /// An unlet statement
    Unlet {
        pos: Position,
        mods: Vec<Modifier>,
        /// Whether this command was invoked with a bang (`!`).
        bang: bool,
        /// The variables to be unlet.
        list: Vec<Box<Node>>,
    },
    /// A while loop
    While {
        pos: Position,
        mods: Vec<Modifier>,
        /// The commands in the body of the loop.
        body: Vec<Box<Node>>,
        /// The condition of the loop.
        cond: Box<Node>,
        /// The `endwhile` - an [End](#variant.End) Node. Note that while this is an Option, it is
        /// a parse error for there not to be one - it's only an Option so the parser can parse the
        /// body of the loop before the `endwhile` is found.
        end: Option<Box<Node>>,
    },
}

impl Node {
    /// The position of a node. Also accessible directly through the `pos` member of each node's
    /// inner struct (every node variant has a `pos` member), but this method is provided for
    /// convenience to avoid having to destructure a variant just to get the position.
    pub fn pos(&self) -> Position {
        match self {
            Node::Augroup { pos, .. }
            | Node::Autocmd { pos, .. }
            | Node::BinaryOp { pos, .. }
            | Node::BlankLine { pos, .. }
            | Node::Call { pos, .. }
            | Node::Catch { pos, .. }
            | Node::Colorscheme { pos, .. }
            | Node::Comment { pos, .. }
            | Node::CurlyName { pos, .. }
            | Node::CurlyNameExpr { pos, .. }
            | Node::CurlyNamePart { pos, .. }
            | Node::DelFunction { pos, .. }
            | Node::Dict { pos, .. }
            | Node::Dot { pos, .. }
            | Node::Echo { pos, .. }
            | Node::EchoHl { pos, .. }
            | Node::Else { pos, .. }
            | Node::ElseIf { pos, .. }
            | Node::End { pos, .. }
            | Node::Env { pos, .. }
            | Node::ExCall { pos, .. }
            | Node::ExCmd { pos, .. }
            | Node::Execute { pos, .. }
            | Node::Finally { pos, .. }
            | Node::For { pos, .. }
            | Node::Function { pos, .. }
            | Node::Identifier { pos, .. }
            | Node::If { pos, .. }
            | Node::Lambda { pos, .. }
            | Node::Let { pos, .. }
            | Node::List { pos, .. }
            | Node::LockVar { pos, .. }
            | Node::Mapping { pos, .. }
            | Node::Number { pos, .. }
            | Node::Option { pos, .. }
            | Node::ParenExpr { pos, .. }
            | Node::Reg { pos, .. }
            | Node::Return { pos, .. }
            | Node::Shebang { pos, .. }
            | Node::Slice { pos, .. }
            | Node::String { pos, .. }
            | Node::Subscript { pos, .. }
            | Node::Ternary { pos, .. }
            | Node::Throw { pos, .. }
            | Node::TopLevel { pos, .. }
            | Node::Try { pos, .. }
            | Node::UnaryOp { pos, .. }
            | Node::Unlet { pos, .. }
            | Node::While { pos, .. } => *pos,
        }
    }

    /// Whether a given node is a [For](#variant.For) node.
    pub fn is_for(node: &Node) -> bool {
        match node {
            Node::For { .. } => true,
            _ => false,
        }
    }

    /// Whether a given node is a [Function](#variant.Function) node.
    pub fn is_function(node: &Node) -> bool {
        match node {
            Node::Function { .. } => true,
            _ => false,
        }
    }

    /// Whether a given node is a [While](#variant.While) node.
    pub fn is_while(node: &Node) -> bool {
        match node {
            Node::While { .. } => true,
            _ => false,
        }
    }

    /// Whether a given node has a `body` member.
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
                Node::Colorscheme { name, .. } => {
                    if let Some(n) = name {
                        format!("(colorscheme {})", n)
                    } else {
                        "(colorscheme)".to_string()
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
                Node::LockVar {
                    cmd, depth, list, ..
                } => {
                    if let Some(d) = depth {
                        display_with_list(&format!("{} {}", cmd, d), &list)
                    } else {
                        display_with_list(&cmd, &list)
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
            pos: Position::empty(),
            mods: vec![],
            var: None,
            list: vec![],
            rest: None,
            right: Box::new(Node::ExCmd {
                pos: Position::empty(),
                mods: vec![],
                bang: false,
                value: "break".to_string(),
            }),
            body: vec![],
            end: None,
        };
        let not_for_node = Node::ExCmd {
            pos: Position::empty(),
            mods: vec![],
            bang: false,
            value: "break".to_string(),
        };
        assert!(Node::is_for(&for_node));
        assert!(!Node::is_for(&not_for_node));
    }

    #[test]
    fn test_node_is_function() {
        let function_node = Node::Function {
            pos: Position::empty(),
            mods: vec![],
            bang: true,
            name: Box::new(Node::ExCmd {
                pos: Position::empty(),
                mods: vec![],
                bang: false,
                value: "break".to_string(),
            }),
            args: vec![],
            body: vec![],
            attrs: vec![],
            end: None,
        };
        let not_function_node = Node::ExCmd {
            pos: Position::empty(),
            mods: vec![],
            bang: false,
            value: "break".to_string(),
        };
        assert!(Node::is_function(&function_node));
        assert!(!Node::is_function(&not_function_node));
    }

    #[test]
    fn test_node_is_while() {
        let while_node = Node::While {
            pos: Position::empty(),
            mods: vec![],
            body: vec![],
            cond: Box::new(Node::ExCmd {
                pos: Position::empty(),
                mods: vec![],
                bang: false,
                value: "break".to_string(),
            }),
            end: None,
        };
        let not_while_node = Node::ExCmd {
            pos: Position::empty(),
            mods: vec![],
            bang: false,
            value: "break".to_string(),
        };
        assert!(Node::is_while(&while_node));
        assert!(!Node::is_while(&not_while_node));
    }

    #[test]
    fn test_has_body() {
        let while_node = Node::While {
            pos: Position::empty(),
            mods: vec![],
            body: vec![],
            cond: Box::new(Node::ExCmd {
                pos: Position::empty(),
                mods: vec![],
                bang: false,
                value: "break".to_string(),
            }),
            end: None,
        };
        let break_node = Node::ExCmd {
            pos: Position::empty(),
            mods: vec![],
            bang: false,
            value: "break".to_string(),
        };
        assert!(Node::has_body(&while_node));
        assert!(!Node::has_body(&break_node));
    }
}
