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

#[derive(Debug)]
pub struct Formatter<'a> {
    ast: &'a Node,
    output: Vec<String>,
    indent_style: &'a str,
    current_indent: usize,
    max_len: usize,
    continuation: usize,
    line: String,
}

impl<'a> Formatter<'a> {
    pub fn new(ast: &Node) -> Formatter {
        Formatter {
            ast,
            output: vec![],
            indent_style: "  ",
            current_indent: 0,
            max_len: 80,
            continuation: 3,
            line: String::new(),
        }
    }

    fn indent(&self) -> String {
        self.indent_style.repeat(self.current_indent)
    }

    fn will_fit(&self, item: &str) -> bool {
        self.line.len() + item.len() <= self.max_len
    }

    fn next_line(&mut self) {
        self.output
            .push(self.line.split_off(0).trim_end().to_string());
        let indent = self.indent();
        self.line.push_str(&indent);
    }

    fn continue_line(&mut self) {
        self.next_line();
        let indent = self.indent();
        self.line.push_str(&format!(
            "{}{}\\ ",
            indent,
            &self.indent_style.repeat(self.continuation)
        ));
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

    pub fn format(&mut self) -> String {
        if let Node::TopLevel { body, .. } = self.ast {
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

    fn f_lr(&mut self, op: &str, left: &Node, right: &Node) {
        self.f(left);
        self.fit(&format!(" {} ", op));
        self.f(right);
    }

    fn f_node(&mut self, node: &Node) {
        // this method assumes there is not a value (besides the current indent) in self.line
        // already. it will always put at least something onto the end of the current line before
        // it checks length and possibly continues onto the next line.
        match node {
            Node::Add { left, right, .. } => self.f_lr("+", left, right),
            Node::And { left, right, .. } => self.f_lr("&&", left, right),
            Node::BinOp {
                left, right, op, ..
            } => self.f_lr(op, left, right),
            Node::Break { .. } => self.add("break"),
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
            Node::Comment { value, .. } => self.add(&format!("\" {}", value)),
            Node::Concat { left, right, .. } => self.f_lr(".", left, right),
            Node::Continue { .. } => self.add("continue"),
            Node::DelFunction { left, .. } => {
                self.add("delfunction ");
                self.f(left);
            }
            Node::Dict { items, .. } => {
                if items.len() == 0 {
                    self.add("{}");
                } else {
                    self.add("{");
                    let last = items.len();
                    for (i, (k, v)) in items.iter().enumerate() {
                        self.f(k);
                        self.add(": ");
                        self.f(v);
                        if i != last - 1 {
                            self.add(", ");
                        }
                    }
                    self.add("}");
                }
            }
            Node::Divide { left, right, .. } => self.f_lr("/", left, right),
            Node::Dot { left, right, .. } => {
                self.f(left);
                self.add(".");
                self.f(right);
            }
            Node::Echo { cmd, list, .. } => {
                self.add(cmd);
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
                // super hack; need to add augroup nodes to parser
                if value == "augroup END" && self.current_indent > 0 {
                    self.current_indent -= 1;
                    self.line.clear();
                    let indent = self.indent();
                    self.line.push_str(&format!("{}{}", indent, value));
                } else if value.starts_with("augroup") && !value.ends_with("END") {
                    self.add(&value);
                    self.current_indent += 1;
                } else {
                    self.add(&value);
                }
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
                for arg in args.iter() {
                    self.f(arg);
                    self.add(", ");
                }
                self.add("-> ");
                self.f(expr);
            }
            Node::Let { right, op, .. } => {
                let var = letlhs_to_string(node);
                self.add("let ");
                self.fit(&var);
                self.fit(&format!(" {} ", op));
                self.f(right);
            }
            Node::List { items, .. } => {
                if items.len() == 0 {
                    self.add("[]");
                } else {
                    self.add("[");
                    let last = items.len();
                    for (i, item) in items.iter().enumerate() {
                        self.f(item);
                        if i != last - 1 {
                            self.add(", ");
                        }
                    }
                    self.add("]");
                }
            }
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
            Node::Minus { left, .. } => {
                self.add("-");
                self.f(left);
            }
            Node::Multiply { left, right, .. } => self.f_lr("*", left, right),
            Node::Not { left, .. } => {
                self.add("!");
                self.f(left);
            }
            Node::Or { left, right, .. } => self.f_lr("||", left, right),
            Node::Plus { left, .. } => {
                self.add("+");
                self.f(left);
            }
            Node::Remainder { left, right, .. } => self.f_lr("%", left, right),
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
            Node::Subtract { left, right, .. } => self.f_lr("-", left, right),
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
            Node::Unlet { list, .. } => {
                self.add("unlet ");
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
                if self.output.len() > 0 && self.output[self.output.len() - 1].trim() != "" {
                    self.next_line(); // blank lines between functions
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
                    self.f_body_node(catch)
                }
                if let Some(f) = finally {
                    self.f_body_node(f);
                }
                self.add("endtry");
            }
            Node::While { cond, body, .. } => {
                self.add("while ");
                self.f(cond);
                for node in body.iter() {
                    self.next_line();
                    self.f(node);
                }
                self.next_line();
                self.add("endwhile");
            }
            _ => (),
        }
    }
}
