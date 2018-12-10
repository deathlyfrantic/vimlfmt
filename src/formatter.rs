use viml_parser::Node;

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

fn letlhs_to_string(node: &Node) -> String {
    let mut rv = String::new();
    match node {
        Node::Let {
            var, list, rest, ..
        }
        | Node::For {
            var, list, rest, ..
        } => {
            // we're making an assumption that var, list, and rest are all atomic nodes. any other
            // kind doesn't make sense, but might be allowed by vim anyway.
            if let Some(v) = var {
                // this is the "x" in "let x = something"
                rv.push_str(&format!("{}", v))
            } else {
                // this is the "[a, b]" in "let [a, b] = something" with an optional "rest" param,
                // e.g. "let [a, b; z] = something". see :h let-unpack
                rv.push_str(&format!(
                    "[{}",
                    list.iter()
                        .map(|n| format!("{}", n))
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
                if let Some(r) = rest {
                    rv.push_str(&format!("; {}", r));
                }
                rv.push(']');
            }
        }
        _ => (),
    }
    rv
}

fn str_length_with_tabs(s: &str) -> usize {
    // assume every tab == 8 spaces which isn't necessarily true for mid-line tabs. we just care
    // about leading tabs here, for the heathens that use tabs for indentation.
    let num_tabs = s.split('\t').collect::<Vec<&str>>().len() - 1;
    s.len() + (num_tabs * 7) // 1 space of each tab is already included in s.len()
}

#[derive(Debug)]
pub struct Formatter<'a> {
    output: Vec<String>,
    indent_style: &'a str,
    current_indent: usize,
    max_len: usize,
    continuation: usize,
    line: String,
    last_line_was_blank: bool,
    current_continuation_indent: usize, // indent beyond the next line backslash
}

impl<'a> Formatter<'a> {
    pub fn new(indent_style: &'a str, continuation: usize, max_len: usize) -> Formatter<'a> {
        Formatter {
            output: vec![],
            indent_style,
            current_indent: 0,
            max_len,
            continuation,
            line: String::new(),
            last_line_was_blank: false,
            current_continuation_indent: 0,
        }
    }

    fn indent(&self) -> String {
        self.indent_style.repeat(self.current_indent)
    }

    fn will_fit(&self, item: &str) -> bool {
        str_length_with_tabs(&self.line) + str_length_with_tabs(item) <= self.max_len
    }

    fn next_line(&mut self) {
        let current_line = self.line.split_off(0).trim_end().to_string();
        if current_line == "" {
            if self.last_line_was_blank {
                return;
            }
            self.last_line_was_blank = true;
        } else {
            self.last_line_was_blank = false;
        }
        self.output.push(current_line);
        let indent = self.indent();
        self.line.push_str(&indent);
    }

    fn continue_line(&mut self) {
        self.output
            .push(self.line.split_off(0).trim_end().to_string());
        let indent = self.indent();
        self.line.push_str(&indent);
        self.line
            .push_str(&self.indent_style.repeat(self.continuation));
        self.line.push_str("\\ ");
        if self.current_continuation_indent > 1 {
            self.line.push_str(
                &self
                    .indent_style
                    .repeat(self.current_continuation_indent - 1),
            )
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

    pub fn format(&mut self, ast: &Node) -> String {
        self.current_indent = 0;
        self.output.clear();
        self.line.clear();
        self.last_line_was_blank = false;
        if let Node::TopLevel { body, .. } = ast {
            for node in body {
                self.f(node);
                self.next_line()
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
            self.output.join("\n")
        } else {
            "provided node is not a TopLevel node".to_string()
        }
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

    fn f_node(&mut self, node: &Node) {
        // this method assumes there is not a value (besides the current indent) in self.line
        // already. it will always put at least something onto the end of the current line before
        // it checks length and possibly continues onto the next line.
        match node {
            Node::Augroup { name, .. } => {
                if name.len() > 0 {
                    if name.to_lowercase() == "end" && self.current_indent > 0 {
                        self.current_indent -= 1;
                        let indent = self.indent();
                        self.line = format!("{}augroup ", indent);
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
                ea,
                group,
                events,
                patterns,
                nested,
                body,
                ..
            } => {
                self.add("autocmd");
                if ea.bang {
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
                            let indent = str_length_with_tabs(&self.indent());
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
            Node::Comment {
                value, trailing, ..
            } => {
                if *trailing {
                    let last = self.output.len() - 1;
                    self.line = self.output.remove(last);
                    self.add(&format!(" \"{}", value));
                } else {
                    self.add(&format!("\"{}", value));
                }
            }
            Node::DelFunction { left, .. } => {
                self.add("delfunction ");
                self.f(left);
            }
            Node::Dict { items, .. } => self.f_dict(items),
            Node::Dot { left, right, .. } => {
                self.f(left);
                self.add(".");
                self.f(right);
            }
            Node::Echo { cmd, list, .. } => {
                self.add(cmd);
                self.add(" ");
                for item in list.iter() {
                    self.f(item);
                }
            }
            Node::EchoHl { value, .. } => {
                self.add("echohl");
                self.fit(&value);
            }
            Node::ExCall { left, .. } => {
                self.add("call ");
                self.f(left);
            }
            Node::ExCmd { value, .. } => {
                self.add(&value);
            }
            Node::Execute { list, .. } => {
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
            Node::Let { right, op, .. } => {
                let var = letlhs_to_string(node);
                self.add("let ");
                self.fit(&var);
                self.fit(&format!(" {} ", op));
                self.f(right);
            }
            Node::List { items, .. } => self.f_list(items),
            Node::LockVar { depth, list, .. } => {
                self.add("lockvar ");
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
                command,
                attrs,
                left,
                right,
                ..
            } => {
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
                    if right.len() > 0 {
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
            Node::Return { left, .. } => {
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
            Node::Throw { err, .. } => {
                self.add("throw ");
                self.f(err);
            }
            Node::UnaryOp { op, right, .. } => {
                self.add(&format!("{}", op));
                self.f(right);
            }
            Node::Unlet { list, ea, .. } => {
                self.add("unlet");
                if ea.bang {
                    self.add("!");
                }
                self.add(" ");
                for item in list.iter() {
                    self.f(item);
                    self.add(" ");
                }
            }
            Node::UnlockVar { depth, list, .. } => {
                self.add("unlockvar ");
                if let Some(d) = depth {
                    self.add(&d.to_string());
                    self.add(" ");
                }
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
            Node::Catch { pattern, body, .. } => {
                self.add("catch");
                if let Some(p) = pattern {
                    self.add(" ");
                    self.fit(&p);
                }
                self.f_body(&body);
            }
            Node::Else { body, .. } => {
                self.add("else");
                self.f_body(body);
            }
            Node::ElseIf { cond, body, .. } => {
                self.add("elseif ");
                self.f(cond);
                self.f_body(body);
            }
            Node::Finally { body, .. } => {
                self.add("finally");
                self.f_body(body);
            }
            Node::For { right, body, .. } => {
                let var = letlhs_to_string(node);
                self.add("for ");
                self.fit(&var);
                self.add(" in ");
                self.f(right);
                self.f_body(body);
                self.add("endfor");
            }
            Node::Function {
                ea,
                name,
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
                self.add("function");
                if ea.bang {
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
                cond,
                elseifs,
                else_,
                body,
                ..
            } => {
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
                body,
                catches,
                finally,
                ..
            } => {
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
            Node::While { cond, body, .. } => {
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
    use super::super::viml_parser::parse_lines;
    use super::*;

    fn create_node(s: &str) -> Node {
        if let Node::TopLevel { body, .. } = parse_lines(&[s]).unwrap() {
            return *body[0].clone();
        }
        panic!("can't create node from '{}'", s);
    }

    #[test]
    fn test_letlhs_to_string() {
        let node = create_node("for var in something | echo 'foo' | endfor");
        assert_eq!(&letlhs_to_string(&node), "var");
        let node = create_node("for [a,b] in something | echo 'foo' | endfor");
        assert_eq!(&letlhs_to_string(&node), "[a, b]");
        let node = create_node("for [a,b;z] in something | echo 'foo' | endfor");
        assert_eq!(&letlhs_to_string(&node), "[a, b; z]");
    }

    #[test]
    fn test_str_length_with_tabs() {
        assert_eq!(str_length_with_tabs("foobar"), 6);
        assert_eq!(str_length_with_tabs("foo\tbar"), 14);
    }
}
