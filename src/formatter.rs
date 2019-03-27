use std::io::{Error, ErrorKind};
use viml_parser::{Modifier, Node};

const INDENT: &str = "  ";
const CONTINUATION: usize = 3;
const MAX_LEN: usize = 80;

fn node_is_atom(node: &Node) -> bool {
    // not building this into the Node struct because this only has meaning in the context of the
    // formatter. in this case "atom" means a node that is a singular, i.e. whose value can only be
    // used as part of a more complex expression and is basically meaningless on its own.
    match node {
        Node::CurlyName { .. }
        | Node::CurlyNameExpr { .. }
        | Node::CurlyNamePart { .. }
        | Node::Env { .. }
        | Node::Identifier { .. }
        | Node::Number { .. }
        | Node::Option { .. }
        | Node::Reg { .. }
        | Node::String { .. } => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct Formatter {
    output: Vec<String>,
    current_indent: usize,
    line: String,
    last_line_was_blank: bool,
    current_continuation_indent: usize, // indent beyond the next line backslash
}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {
            output: vec![],
            current_indent: 0,
            line: String::new(),
            last_line_was_blank: false,
            current_continuation_indent: 0,
        }
    }

    pub fn format(&mut self, ast: &Node) -> Result<String, Error> {
        self.current_indent = 0;
        self.output.clear();
        self.line.clear();
        self.last_line_was_blank = false;
        if let Node::TopLevel { body, .. } = ast {
            for node in body {
                self.f(node);
                self.next_line();
            }
            if self.output.len() > 0 {
                while self.output[0].trim() == "" {
                    self.output.remove(0);
                }
                let mut last = self.output.len() - 1;
                while last > 0 && self.output[last].trim() == "" {
                    self.output.remove(last);
                    last = self.output.len() - 1;
                }
            }
            Ok(self.output.join("\n"))
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "provided node is not a TopLevel node",
            ))
        }
    }

    fn indent(&self) -> String {
        INDENT.repeat(self.current_indent)
    }

    fn will_fit(&self, item: &str) -> bool {
        self.line.len() + item.len() <= MAX_LEN
    }

    fn next_line(&mut self) {
        let current_line = self.line.split_off(0).trim_end().to_string();
        if current_line == "" {
            if self.last_line_was_blank {
                // don't allow more than one blank line
                return;
            }
            self.last_line_was_blank = true;
        } else {
            self.last_line_was_blank = false;
        }
        self.output.push(current_line);
        self.line.push_str(&self.indent());
    }

    fn continue_line(&mut self) {
        self.output
            .push(self.line.split_off(0).trim_end().to_string());
        self.line.push_str(&self.indent());
        self.line.push_str(&INDENT.repeat(CONTINUATION));
        self.line.push_str("\\ ");
        if self.current_continuation_indent > 1 {
            self.line
                .push_str(&INDENT.repeat(self.current_continuation_indent - 1))
        }
    }

    fn add(&mut self, s: &str) {
        self.line.push_str(s);
    }

    fn fit(&mut self, s: &str) {
        if !self.will_fit(s) {
            self.continue_line();
        }
        self.add(s);
    }

    fn f(&mut self, node: &Node) {
        if node_is_atom(node) {
            self.f_atom_node(node);
        } else if Node::has_body(node) {
            self.f_body_node(node);
        } else {
            self.f_node(node);
        }
    }

    fn f_atom_node(&mut self, node: &Node) {
        // this method assumes there is some value in self.line already, and just adds the
        // formatted node to that value, or continues it on the next line. for these nodes the
        // Display output is what we want.
        self.fit(&format!("{}", node));
    }

    fn f_letlhs(&mut self, node: &Node) {
        match node {
            Node::Let {
                var, list, rest, ..
            }
            | Node::For {
                var, list, rest, ..
            } => {
                if let Some(v) = var {
                    self.f(v);
                } else {
                    self.add("[");
                    let last = list.len() - 1;
                    for (i, node) in list.iter().enumerate() {
                        self.f(node);
                        if i != last {
                            self.add(", ");
                        }
                    }
                    if let Some(r) = rest {
                        self.add("; ");
                        self.f(r);
                    }
                    self.add("]");
                }
            }
            _ => (),
        }
    }

    fn f_list(&mut self, items: &[Box<Node>]) {
        if items.len() == 0 {
            self.fit("[]");
        } else {
            // try to fit this on one line first
            let saved_line = self.line.clone();
            let marker = self.output.len();
            self.fit("[");
            let last = items.len();
            for (i, item) in items.iter().enumerate() {
                self.f(item);
                if i != last - 1 {
                    self.add(", ");
                }
            }
            self.fit("]");
            // did it fit?
            if self.output.len() != marker {
                // if we had to add lines to the output, it did not. delete the lines we added.
                self.output.truncate(marker);
                self.line = saved_line;
                // now add a single item per line ("block" style)
                self.fit("[");
                self.current_continuation_indent += 1;
                for item in items.iter() {
                    self.continue_line();
                    self.f(item);
                    self.add(",");
                }
                self.current_continuation_indent -= 1;
                self.continue_line();
                self.add("]");
            }
        }
    }

    fn f_dict(&mut self, items: &[(Box<Node>, Box<Node>)]) {
        if items.len() == 0 {
            self.fit("{}");
        } else {
            // try to fit on one line first
            let saved_line = self.line.clone();
            let marker = self.output.len();
            self.fit("{");
            let last = items.len();
            for (i, (k, v)) in items.iter().enumerate() {
                self.f(k);
                self.add(": ");
                self.f(v);
                if i != last - 1 {
                    self.add(", ");
                }
            }
            self.fit("}");
            // did it fit?
            if self.output.len() != marker {
                // if we had to add lines to the output, it did not. delete the lines we added.
                self.output.truncate(marker);
                self.line = saved_line;
                // now add a single item per line ("block" style)
                self.fit("{");
                self.current_continuation_indent += 1;
                for (k, v) in items.iter() {
                    self.continue_line();
                    self.f(k);
                    self.add(": ");
                    self.f(v);
                    self.add(",");
                }
                self.current_continuation_indent -= 1;
                self.continue_line();
                self.add("}");
            }
        }
    }

    fn f_mods(&mut self, mods: &[Modifier]) {
        for modifier in mods {
            if let Some(count) = modifier.count {
                self.add(&count.to_string());
            }
            self.add(&modifier.name);
            if modifier.bang {
                self.add("!");
            }
            self.add(" ");
        }
    }

    fn f_node(&mut self, node: &Node) {
        // this method assumes there is not a value (besides the current indent) in self.line
        // already. it will always put at least something onto the end of the current line before
        // it checks length and possibly continues onto the next line.
        match node {
            Node::Augroup { name, .. } => {
                if name.len() > 0 {
                    if name.to_lowercase() == "end" && self.current_indent > 0 {
                        self.current_indent -= 1;
                        self.line = format!("{}augroup ", self.indent());
                        self.fit("END"); // do not allow lowercase "end"
                    } else {
                        self.add("augroup ");
                        self.fit(&name.replace("|", "\\|").replace("\"", "\\\""));
                        self.current_indent += 1;
                    }
                } else {
                    self.add("augroup");
                }
            }
            Node::Autocmd {
                mods,
                bang,
                group,
                events,
                patterns,
                nested,
                body,
                ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("autocmd");
                if *bang {
                    self.add("!");
                }
                if group.len() > 0 {
                    self.add(" ");
                    self.fit(group);
                }
                if events.len() > 0 {
                    let mut events = events.clone();
                    events.sort_unstable();
                    self.fit(&format!(" {}", events.join(",")));
                }
                if patterns.len() > 0 {
                    let mut patterns = patterns.clone();
                    patterns.sort_unstable();
                    self.fit(&format!(" {}", patterns.join(",")));
                }
                if *nested {
                    self.fit(" nested");
                }
                if body.len() > 0 {
                    let saved_output = self.output.split_off(0);
                    let saved_line = self.line.split_off(0);
                    let mut trimmed = vec![];
                    let mut raw = vec![];
                    for node in body {
                        self.output.clear();
                        self.line.clear();
                        self.f(node);
                        self.next_line();
                        trimmed.push(
                            self.output
                                .iter()
                                .map(|line| line.trim())
                                .collect::<Vec<&str>>()
                                .join(" | "),
                        );
                        raw.push(self.output.split_off(0));
                    }
                    self.output = saved_output;
                    self.line = saved_line;
                    self.add(" ");
                    let last_raw = raw.len() - 1;
                    for i in 0..raw.len() {
                        if self.will_fit(&trimmed[i]) {
                            self.add(&trimmed[i]);
                        } else {
                            let pieces = raw[i].clone();
                            let last_piece = pieces.len() - 1;
                            let indent = self.indent().len();
                            for (j, piece) in pieces.iter().enumerate() {
                                self.continue_line();
                                if j == 0 {
                                    self.add(&piece);
                                } else {
                                    self.add(piece.get(indent..).unwrap());
                                }
                                if j != last_piece {
                                    self.add(" | ");
                                }
                            }
                        }
                        if i != last_raw {
                            self.add(" | ");
                        }
                    }
                }
            }
            Node::BinaryOp {
                left, right, op, ..
            } => {
                self.f(left);
                self.fit(&format!(" {} ", op));
                self.f(right);
            }
            Node::Call { name, args, .. } => {
                self.f(name);
                self.add("(");
                let last = args.len();
                for (i, arg) in args.iter().enumerate() {
                    self.f(arg);
                    if i != last - 1 {
                        self.add(", ");
                    }
                }
                self.add(")");
            }
            Node::Colorscheme { name, .. } => {
                self.add("colorscheme");
                if let Some(n) = name {
                    self.fit(&format!(" {}", n));
                }
            }
            Node::Comment {
                value, trailing, ..
            } => {
                let comment = if value.starts_with(char::is_whitespace) {
                    value.to_string()
                } else {
                    format!(" {}", value)
                };
                if *trailing {
                    let last = self.output.len() - 1;
                    self.line = self.output.remove(last);
                    self.add(&format!(" \"{}", comment));
                } else {
                    self.add(&format!("\"{}", comment));
                }
            }
            Node::DelFunction {
                mods, bang, left, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("delfunction");
                if *bang {
                    self.add("!");
                }
                self.add(" ");
                self.f(left);
            }
            Node::Dict { items, .. } => self.f_dict(items),
            Node::Dot { left, right, .. } => {
                self.f(left);
                self.add(".");
                self.f(right);
            }
            Node::Echo {
                mods, cmd, list, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add(cmd);
                self.add(" ");
                for item in list.iter() {
                    self.f(item);
                }
            }
            Node::EchoHl { mods, value, .. } => {
                self.f_mods(mods.as_slice());
                self.add("echohl");
                self.fit(&value);
            }
            Node::ExCall { mods, left, .. } => {
                self.f_mods(mods.as_slice());
                self.add("call ");
                self.f(left);
            }
            Node::ExCmd { value, .. } => {
                // should call f_mods() here but `value` is just the whole line verbatim so not
                // currently necessary
                self.add(&value);
            }
            Node::Execute { mods, list, .. } => {
                self.f_mods(mods.as_slice());
                self.add("execute ");
                for item in list.iter() {
                    self.f(item);
                    self.add(" ");
                }
            }
            Node::Lambda { args, expr, .. } => {
                self.add("{");
                for (i, arg) in args.iter().enumerate() {
                    self.f(arg);
                    if i != args.len() - 1 {
                        self.add(",");
                    }
                    self.add(" ");
                }
                self.add("-> ");
                self.f(expr);
                self.fit("}");
            }
            Node::Let {
                mods, right, op, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("let ");
                self.f_letlhs(node);
                self.fit(&format!(" {} ", op));
                self.f(right);
            }
            Node::List { items, .. } => self.f_list(items),
            Node::LockVar {
                mods,
                cmd,
                bang,
                depth,
                list,
                ..
            } => {
                self.f_mods(mods.as_slice());
                self.add(&cmd);
                if *bang {
                    self.add("!");
                }
                self.add(" ");
                if let Some(d) = depth {
                    self.add(&d.to_string());
                    self.add(" ");
                }
                for item in list.iter() {
                    self.f(item);
                    self.add(" ");
                }
            }
            Node::Mapping {
                mods,
                command,
                attrs,
                left,
                right,
                right_expr,
                ..
            } => {
                self.f_mods(mods.as_slice());
                self.add(&command);
                if attrs.len() > 0 {
                    let mut attrs = attrs.clone();
                    attrs.sort_unstable();
                    for attr in attrs {
                        self.fit(&format!(" <{}>", attr));
                    }
                }
                if left.len() > 0 {
                    self.add(" ");
                    self.fit(&left);
                    if let Some(re) = right_expr {
                        self.add(" ");
                        self.f(re);
                    } else if right.len() > 0 {
                        self.add(" ");
                        self.fit(&right.replace("|", "\\|"));
                    }
                }
            }
            Node::ParenExpr { expr, .. } => {
                self.add("(");
                self.f(expr);
                self.fit(")");
            }
            Node::Return { mods, left, .. } => {
                self.f_mods(mods.as_slice());
                self.add("return");
                if let Some(l) = left {
                    self.add(" ");
                    self.f(l);
                }
            }
            Node::Shebang { value, .. } => self.add(&format!("#!{}", value)),
            Node::Slice {
                name, left, right, ..
            } => {
                self.f(name);
                self.add("[");
                if let Some(l) = left {
                    self.f(l);
                }
                self.add(":");
                if let Some(r) = right {
                    self.f(r);
                }
                self.add("]");
            }
            Node::Subscript { name, index, .. } => {
                self.f(name);
                self.add("[");
                self.f(index);
                self.add("]");
            }
            Node::Ternary {
                cond, left, right, ..
            } => {
                self.f(cond);
                self.add(" ? ");
                self.f(left);
                self.add(" : ");
                self.f(right);
            }
            Node::Throw { mods, err, .. } => {
                self.f_mods(mods.as_slice());
                self.add("throw ");
                self.f(err);
            }
            Node::UnaryOp { op, right, .. } => {
                self.add(&format!("{}", op));
                self.f(right);
            }
            Node::Unlet {
                mods, bang, list, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("unlet");
                if *bang {
                    self.add("!");
                }
                self.add(" ");
                for item in list.iter() {
                    self.f(item);
                    self.add(" ");
                }
            }
            _ => (),
        };
    }

    fn f_body(&mut self, body: &Vec<Box<Node>>) {
        self.current_indent += 1;
        for node in body.iter() {
            self.next_line();
            self.f(node);
        }
        self.current_indent -= 1;
        self.next_line();
    }

    fn f_body_node(&mut self, node: &Node) {
        match node {
            Node::Catch {
                mods,
                pattern,
                body,
                ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("catch");
                if let Some(p) = pattern {
                    self.add(" ");
                    self.fit(&p);
                }
                self.f_body(&body);
            }
            Node::Else { mods, body, .. } => {
                self.f_mods(mods.as_slice());
                self.add("else");
                self.f_body(body);
            }
            Node::ElseIf {
                mods, cond, body, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("elseif ");
                self.f(cond);
                self.f_body(body);
            }
            Node::Finally { mods, body, .. } => {
                self.f_mods(mods.as_slice());
                self.add("finally");
                self.f_body(body);
            }
            Node::For {
                mods, right, body, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("for ");
                self.f_letlhs(node);
                self.add(" in ");
                self.f(right);
                self.f_body(body);
                self.add("endfor");
            }
            Node::Function {
                mods,
                name,
                bang,
                args,
                attrs,
                body,
                ..
            } => {
                if self.output.len() > 0 {
                    // a function must be preceded by a blank line or a comment
                    let last_line = self.output[self.output.len() - 1].trim().to_string();
                    if last_line != "" && !last_line.starts_with('"') {
                        self.next_line(); // blank lines between functions
                    }
                }
                self.f_mods(mods.as_slice());
                self.add("function");
                if *bang {
                    self.add("!");
                }
                self.add(" ");
                self.f(name);
                self.add("(");
                let last = args.len();
                for (i, arg) in args.iter().enumerate() {
                    self.f(arg);
                    if i != last - 1 {
                        self.add(", ");
                    }
                }
                self.add(")");
                if attrs.len() > 0 {
                    self.add(&format!(" {}", attrs.join(" ")));
                }
                self.f_body(body);
                self.add("endfunction");
                self.next_line(); // blank lines between functions
            }
            Node::If {
                mods,
                cond,
                elseifs,
                else_,
                body,
                ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("if ");
                self.f(cond);
                self.f_body(body);
                for elseif in elseifs.iter() {
                    self.f_body_node(elseif);
                }
                if let Some(e) = else_ {
                    self.f_body_node(e);
                }
                self.add("endif");
            }
            Node::Try {
                mods,
                body,
                catches,
                finally,
                ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("try");
                self.f_body(body);
                for catch in catches.iter() {
                    self.f_body_node(catch);
                }
                if let Some(f) = finally {
                    self.f_body_node(f);
                }
                self.add("endtry");
            }
            Node::While {
                mods, cond, body, ..
            } => {
                self.f_mods(mods.as_slice());
                self.add("while ");
                self.f(cond);
                self.f_body(body);
                self.add("endwhile");
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse_lines;
    use super::*;

    #[test]
    fn test_list_formatting() {
        // "line formatting" - entire list fits on a single line
        let node =
            parse_lines(&["let foo = ['this list will fit', 'this list will fit']"]).unwrap();
        let mut formatter = Formatter::new();
        let result = formatter.format(&node).unwrap();
        let expected = "let foo = ['this list will fit', 'this list will fit']";
        assert_eq!(expected, &result);
        // "block formatting" - list won't fit on a single line, so format it as a block
        let node = parse_lines(
            &[r#"let foo = ['list is too long', 'list is too long', 'list is too long', 'list is too long']"#]
        ).unwrap();
        let mut formatter = Formatter::new();
        let result = formatter.format(&node).unwrap();
        let expected = r#"let foo = [
      \ 'list is too long',
      \ 'list is too long',
      \ 'list is too long',
      \ 'list is too long',
      \ ]"#;
        assert_eq!(expected, &result);
    }

    #[test]
    fn test_dict_formatting() {
        // "line formatting" - entire dict fits on a single line
        let node =
            parse_lines(&["let foo = {'this': 'dict will fit', 'this dict': 'will fit'}"]).unwrap();
        let mut formatter = Formatter::new();
        let result = formatter.format(&node).unwrap();
        let expected = "let foo = {'this': 'dict will fit', 'this dict': 'will fit'}";
        assert_eq!(expected, &result);
        // "block formatting" - dict won't fit on a single line, so format it as a block
        let node = parse_lines(
            &[r#"let foo = {'this': 'dict will not fit', 'this dict': 'will not fit', 'this dict will': 'not fit'}"#]
        ).unwrap();
        let mut formatter = Formatter::new();
        let result = formatter.format(&node).unwrap();
        let expected = r#"let foo = {
      \ 'this': 'dict will not fit',
      \ 'this dict': 'will not fit',
      \ 'this dict will': 'not fit',
      \ }"#;
        assert_eq!(expected, &result);
    }
}
