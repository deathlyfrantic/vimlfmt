use super::{isargname, isdigit, isvarname, iswhite, iswordc, ParseError, Position};
use command::{neovim_commands, vim_commands, Command, Flag, ParserKind};
use exarg::ExArg;
use modifier::Modifier;
use node::{Node, NodeKind, NodeParser};
use reader::Reader;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;
use token::{TokenKind, Tokenizer};

fn ends_excmds(s: &str) -> bool {
    ["", "|", "\"", "<EOF>", "\n"].contains(&s)
}

#[derive(Debug)]
pub struct Parser {
    reader: Rc<RefCell<Reader>>,
    context: Vec<Rc<RefCell<Node>>>,
    commands: Vec<Command>,
}

impl Parser {
    pub fn new(reader: Reader, neovim: bool) -> Parser {
        Parser {
            reader: Rc::new(RefCell::new(reader)),
            context: vec![],
            commands: if neovim {
                neovim_commands()
            } else {
                vim_commands()
            },
        }
    }

    fn push_context(&mut self, node: Rc<RefCell<Node>>) {
        self.context.insert(0, node)
    }

    fn pop_context(&mut self) -> Rc<RefCell<Node>> {
        self.context.remove(0)
    }

    fn find_context(&self, needle: NodeKind) -> bool {
        for node in self.context.iter() {
            if node.borrow().kind == needle {
                return true;
            }
        }
        false
    }

    fn add_node(&mut self, node: Rc<RefCell<Node>>) {
        self.context[0].borrow_mut().body.push(node);
    }

    fn check_missing_endfunction(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if self.context[0].borrow().kind == NodeKind::Function {
            return Err(ParseError {
                msg: format!("E126: Missing :endfunction:    {}", end),
                pos: pos,
            });
        }
        Ok(())
    }

    fn check_missing_endif(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if [NodeKind::If, NodeKind::ElseIf, NodeKind::Else].contains(&self.context[0].borrow().kind)
        {
            return Err(ParseError {
                msg: format!("E126: Missing :endif:    {}", end),
                pos: pos,
            });
        }
        Ok(())
    }

    fn check_missing_endtry(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if [NodeKind::Try, NodeKind::Catch, NodeKind::Finally]
            .contains(&self.context[0].borrow().kind)
        {
            return Err(ParseError {
                msg: format!("E126: Missing :endtry:    {}", end),
                pos: pos,
            });
        }
        Ok(())
    }

    fn check_missing_endwhile(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if self.context[0].borrow().kind == NodeKind::While {
            return Err(ParseError {
                msg: format!("E126: Missing :endwhile:    {}", end),
                pos: pos,
            });
        }
        Ok(())
    }

    fn check_missing_endfor(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if self.context[0].borrow().kind == NodeKind::For {
            return Err(ParseError {
                msg: format!("E126: Missing :endfor:    {}", end),
                pos: pos,
            });
        }
        Ok(())
    }

    fn err<T>(&self, msg: &str) -> Result<T, ParseError> {
        Err(ParseError {
            msg: msg.to_string(),
            pos: self.reader.borrow().getpos(),
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let mut toplevel = Node::new(NodeKind::TopLevel);
        toplevel.pos = self.reader.borrow().getpos();
        self.push_context(Rc::new(RefCell::new(toplevel)));
        while self.reader.borrow().peek() != "<EOF>" {
            self.parse_one_cmd()?;
        }
        self.check_missing_endfunction("TOPLEVEL", self.reader.borrow().getpos())?;
        self.check_missing_endif("TOPLEVEL", self.reader.borrow().getpos())?;
        self.check_missing_endtry("TOPLEVEL", self.reader.borrow().getpos())?;
        self.check_missing_endwhile("TOPLEVEL", self.reader.borrow().getpos())?;
        self.check_missing_endfor("TOPLEVEL", self.reader.borrow().getpos())?;
        match Rc::try_unwrap(self.pop_context()) {
            Ok(node) => Ok(node.into_inner()),
            Err(_) => Err(ParseError {
                msg: "unable to remove node from context vector".to_string(),
                pos: Position::empty(),
            }),
        }
    }

    fn parse_expr(&mut self) -> Result<Node, ParseError> {
        NodeParser::new(Rc::clone(&self.reader)).parse()
    }

    fn parse_one_cmd(&mut self) -> Result<(), ParseError> {
        let mut ea = ExArg::new();
        if self.reader.borrow().peekn(2) == "#!" {
            self.parse_shebang()?;
            return Ok(());
        }
        self.reader.borrow_mut().skip_white_and_colon();
        if self.reader.borrow().peek() == "\n" {
            self.reader.borrow_mut().get();
            return Ok(());
        }
        if self.reader.borrow().peek() == "\"" {
            self.parse_comment()?;
            self.reader.borrow_mut().get();
            return Ok(());
        }
        ea.linepos = self.reader.borrow().getpos();
        ea.modifiers = self.parse_command_modifiers()?;
        ea.range = self.parse_range()?;
        self.parse_command(ea)?;
        self.parse_trail()?;
        Ok(())
    }

    fn parse_shebang(&mut self) -> Result<(), ParseError> {
        let sb = self.reader.borrow_mut().getn(2);
        if sb != "#!" {
            return self.err(&format!("unexpected characters: {}", sb));
        }
        let mut node = Node::new(NodeKind::Shebang);
        node.pos = self.reader.borrow().getpos();
        node.string = self.reader.borrow_mut().get_line();
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), ParseError> {
        let npos = self.reader.borrow().getpos();
        let c = self.reader.borrow_mut().get();
        if c != "\"" {
            return Err(ParseError {
                msg: format!("unexpected character: {}", c),
                pos: npos,
            });
        }
        let mut node = Node::new(NodeKind::Comment);
        node.pos = npos;
        node.string = self.reader.borrow_mut().get_line();
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_command_modifiers(&mut self) -> Result<Vec<Modifier>, ParseError> {
        let mut modifiers: Vec<Modifier> = vec![];
        loop {
            let pos = self.reader.borrow().tell();
            let mut d = "".to_string();
            let peeked = self.reader.borrow().peek();
            if isdigit(&peeked) {
                d = self.reader.borrow_mut().read_digit();
                self.reader.borrow_mut().skip_white();
            }
            let k = self.reader.borrow_mut().read_alpha();
            let c = self.reader.borrow().peek();
            self.reader.borrow_mut().skip_white();
            match k {
                _ if "aboveleft".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("aboveleft"))
                }
                _ if "belowright".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("belowright"))
                }
                _ if "browse".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("browse"))
                }
                _ if "botright".starts_with(&k) && k.len() >= 2 => {
                    modifiers.push(Modifier::new("botright"))
                }
                _ if "confirm".starts_with(&k) && k.len() >= 4 => {
                    modifiers.push(Modifier::new("confirm"))
                }
                _ if "keepmarks".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("keepmarks"))
                }
                _ if "keepalt".starts_with(&k) && k.len() >= 5 => {
                    modifiers.push(Modifier::new("keepalt"))
                }
                _ if "keepjumps".starts_with(&k) && k.len() >= 5 => {
                    modifiers.push(Modifier::new("keepjumps"))
                }
                _ if "keeppatterns".starts_with(&k) && k.len() >= 5 => {
                    modifiers.push(Modifier::new("keeppatterns"))
                }
                _ if "hide".starts_with(&k) && k.len() >= 3 => {
                    if ends_excmds(&c) {
                        break;
                    }
                    modifiers.push(Modifier::new("hide"))
                }
                _ if "lockmarks".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("lockmarks"))
                }
                _ if "leftabove".starts_with(&k) && k.len() >= 5 => {
                    modifiers.push(Modifier::new("leftabove"))
                }
                _ if "noautocmd".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("noautocmd"))
                }
                _ if "noswapfile".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("noswapfile"))
                }
                _ if "rightbelow".starts_with(&k) && k.len() >= 6 => {
                    modifiers.push(Modifier::new("rightbelow"))
                }
                _ if "sandbox".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("sandbox"))
                }
                _ if "silent".starts_with(&k) && k.len() >= 3 => {
                    let mut mods = Modifier::new("silent");
                    if c == "!" {
                        mods.bang = true;
                        self.reader.borrow_mut().get();
                    }
                    modifiers.push(mods)
                }
                _ if &k == "tab" => {
                    let mut mods = Modifier::new("tab");
                    if let Ok(n) = d.parse::<usize>() {
                        mods.count = n;
                    }
                    modifiers.push(mods)
                }
                _ if "topleft".starts_with(&k) && k.len() >= 2 => {
                    modifiers.push(Modifier::new("topleft"))
                }
                _ if "unsilent".starts_with(&k) && k.len() >= 3 => {
                    modifiers.push(Modifier::new("unsilent"))
                }
                _ if "vertical".starts_with(&k) && k.len() >= 4 => {
                    modifiers.push(Modifier::new("vertical"))
                }
                _ if "verbose".starts_with(&k) && k.len() >= 4 => {
                    let mut mods = Modifier::new("verbose");
                    mods.count = match d.parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => 1,
                    };
                    modifiers.push(mods)
                }
                _ => {
                    self.reader.borrow_mut().seek_set(pos);
                    break;
                }
            }
        }
        Ok(modifiers)
    }

    fn parse_range(&mut self) -> Result<Vec<String>, ParseError> {
        let mut tokens: Vec<String> = vec![];
        loop {
            loop {
                self.reader.borrow_mut().skip_white();
                let c = self.reader.borrow().peek();
                match c.as_str() {
                    "" => break,
                    "." | "$" => tokens.push(self.reader.borrow_mut().get()),
                    "'" => {
                        if self.reader.borrow().peek_ahead(1) == "\n" {
                            break;
                        }
                        tokens.push(self.reader.borrow_mut().getn(2));
                    }
                    "/" | "?" => {
                        self.reader.borrow_mut().get();
                        let (pattern, _) = self.parse_pattern(&c)?;
                        tokens.push(pattern);
                    }
                    "\\" => {
                        let m = self.reader.borrow().peek_ahead(1);
                        if m == "&" || m == "?" || m == "/" {
                            tokens.push(self.reader.borrow_mut().getn(2));
                        } else {
                            return self.err("E10: \\\\ should be followed by /, ? or &");
                        }
                    }
                    _ if isdigit(&c) => {
                        tokens.push(self.reader.borrow_mut().read_digit());
                    }
                    _ => (),
                }
                loop {
                    self.reader.borrow_mut().skip_white();
                    if self.reader.borrow().peek() == "\n" {
                        break;
                    }
                    let n = self.reader.borrow_mut().read_integer();
                    if n == "" {
                        break;
                    }
                    tokens.push(n);
                }
                if self.reader.borrow().peek() != "/" && self.reader.borrow().peek() != "?" {
                    break;
                }
            }
            let p = self.reader.borrow().peek();
            if p == "%" || p == "*" {
                tokens.push(self.reader.borrow_mut().get());
            }
            let p = self.reader.borrow().peek();
            if p == ";" || p == "," {
                tokens.push(self.reader.borrow_mut().get());
                continue;
            }
            break;
        }
        Ok(tokens)
    }

    fn parse_pattern(&mut self, delimiter: &str) -> Result<(String, String), ParseError> {
        let mut pattern = String::new();
        let mut endc = String::new();
        let mut in_bracket = 0;
        loop {
            let c = self.reader.borrow_mut().getn(1);
            if c == "" {
                break;
            }
            if c == delimiter && in_bracket == 0 {
                endc = c;
                break;
            }
            pattern.push_str(&c);
            if c == "\\" {
                let c = self.reader.borrow().peek();
                if c == "\n" {
                    return self.err("E682: Invalid search pattern or delimiter");
                }
                self.reader.borrow_mut().getn(1);
                pattern.push_str(&c);
            } else if c == "[" {
                in_bracket += 1;
            } else if c == "]" {
                in_bracket -= 1;
            }
        }
        Ok((pattern, endc))
    }

    fn parse_command(&mut self, mut ea: ExArg) -> Result<(), ParseError> {
        self.reader.borrow_mut().skip_white_and_colon();
        ea.cmdpos = self.reader.borrow().getpos();
        let peeked = self.reader.borrow().peek();
        if ["\n", "\"", "<EOF>", ""].contains(&peeked.as_str()) {
            if ea.modifiers.len() > 0 || ea.range.len() > 0 {
                self.parse_cmd_modifier_range(ea);
            }
            return Ok(());
        }
        match self.find_command() {
            Some(c) => {
                ea.cmd = c;
            }
            None => {
                return self.err(&format!(
                    "E492: Not an editor command: {}",
                    self.reader.borrow().peek_line()
                ));
            }
        }
        if self.reader.borrow().peek() == "!"
            && !["substitute", "smagic", "snomagic"].contains(&ea.cmd.name.as_str())
        {
            self.reader.borrow_mut().get();
            ea.force_it = true;
        }
        if !ea.cmd.flags.contains(&Flag::Bang)
            && ea.force_it
            && !ea.cmd.flags.contains(&Flag::Usercmd)
        {
            return Err(ParseError {
                msg: "E477: No ! allowed".to_string(),
                pos: ea.cmdpos,
            });
        }
        if ea.cmd.name != "!" {
            self.reader.borrow_mut().skip_white();
        }
        ea.argpos = self.reader.borrow().getpos();
        if ea.cmd.flags.contains(&Flag::Argopt) {
            self.parse_argopt()?;
        }
        if ea.cmd.name == "write" || ea.cmd.name == "update" {
            if self.reader.borrow().peek() == ">" {
                if self.reader.borrow().peek_ahead(1) == ">" {
                    return self.err("E494: Use w or w>>");
                }
                self.reader.borrow_mut().seek_cur(2);
                self.reader.borrow_mut().skip_white();
            } else if self.reader.borrow().peek() == "!" && ea.cmd.name == "write" {
                self.reader.borrow_mut().get();
                ea.use_filter = true;
            }
        }
        if ea.cmd.name == "read" {
            if ea.force_it {
                ea.use_filter = true;
                ea.force_it = false;
            } else if self.reader.borrow().peek() == "!" {
                self.reader.borrow_mut().get();
                ea.use_filter = true;
            }
        }
        if ea.cmd.name == "<" || ea.cmd.name == ">" {
            while self.reader.borrow().peek() == ea.cmd.name {
                self.reader.borrow_mut().get();
            }
            self.reader.borrow_mut().skip_white();
        }
        if ea.cmd.flags.contains(&Flag::Editcmd) && !ea.use_filter {
            self.parse_argcmd();
        }
        self._parse_command(ea)
    }

    fn _parse_command(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match ea.cmd.parser {
            ParserKind::Append | ParserKind::Insert => Ok(self.parse_cmd_append(ea)),
            ParserKind::Break => self.parse_cmd_break(ea),
            ParserKind::Call => self.parse_cmd_call(ea),
            ParserKind::Catch => self.parse_cmd_catch(ea),
            ParserKind::Common | ParserKind::Usercmd => self.parse_cmd_common(ea),
            ParserKind::Continue => self.parse_cmd_continue(ea),
            ParserKind::Delfunction => self.parse_cmd_delfunction(ea),
            ParserKind::Echo => self.parse_cmd_with_exprlist(ea, NodeKind::Echo),
            ParserKind::Echoerr => self.parse_cmd_with_exprlist(ea, NodeKind::EchoErr),
            ParserKind::Echohl => self.parse_cmd_echohl(ea),
            ParserKind::Echomsg => self.parse_cmd_with_exprlist(ea, NodeKind::EchoMsg),
            ParserKind::Echon => self.parse_cmd_with_exprlist(ea, NodeKind::EchoN),
            ParserKind::Else => self.parse_cmd_else(ea),
            ParserKind::Elseif => self.parse_cmd_elseif(ea),
            ParserKind::Endfor => self.parse_cmd_endfor(ea),
            ParserKind::Endfunction => self.parse_cmd_endfunction(ea),
            ParserKind::Endif => self.parse_cmd_endif(ea),
            ParserKind::Endtry => self.parse_cmd_endtry(ea),
            ParserKind::Endwhile => self.parse_cmd_endwhile(ea),
            ParserKind::Execute => self.parse_cmd_with_exprlist(ea, NodeKind::Execute),
            ParserKind::Finally => self.parse_cmd_finally(ea),
            ParserKind::Finish => self.parse_cmd_finish(ea),
            ParserKind::For => self.parse_cmd_for(ea),
            ParserKind::Function => self.parse_cmd_function(ea),
            ParserKind::If => self.parse_cmd_if(ea),
            ParserKind::Let => self.parse_cmd_let(ea),
            ParserKind::Loadkeymap => self.parse_cmd_loadkeymap(ea),
            ParserKind::Lockvar => self.parse_cmd_lockvar(ea),
            ParserKind::Lang => self.parse_cmd_lang(ea),
            ParserKind::Return => self.parse_cmd_return(ea),
            ParserKind::Syntax => self.parse_cmd_syntax(ea),
            ParserKind::Throw => self.parse_cmd_throw(ea),
            ParserKind::Try => self.parse_cmd_try(ea),
            ParserKind::Unlet => self.parse_cmd_unlet(ea),
            ParserKind::Unlockvar => self.parse_cmd_unlockvar(ea),
            ParserKind::While => self.parse_cmd_while(ea),
            ParserKind::Wincmd => self.parse_cmd_wincmd(ea),
        }
    }

    fn parse_cmd_append(&mut self, ea: ExArg) {
        self.reader.borrow_mut().setpos(ea.linepos);
        let cmdline = self.reader.borrow_mut().get_line();
        let mut lines = vec![cmdline];
        loop {
            if self.reader.borrow().peek() == "<EOF>" {
                break;
            }
            let line = self.reader.borrow_mut().get_line();
            lines.push(line);
            if lines.last().unwrap() == "." {
                break;
            }
        }
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.string = lines.join("\n");
        self.add_node(Rc::new(RefCell::new(node)));
    }

    fn parse_cmd_break(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if !self.find_context(NodeKind::While) && !self.find_context(NodeKind::For) {
            return self.err("E587: :break without :while or :for");
        }
        let mut node = Node::new(NodeKind::Break);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_call(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::ExCall);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.reader.borrow_mut().skip_white();
        if ends_excmds(&self.reader.borrow().peek()) {
            return self.err("E471: Argument required");
        }
        let left = self.parse_expr()?;
        if left.kind != NodeKind::Call {
            return Err(ParseError {
                msg: "Not a function call".to_string(),
                pos: left.pos,
            });
        }
        node.left = Some(Box::new(left));
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_catch(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if self.context[0].borrow().kind == NodeKind::Finally {
            return Err(ParseError {
                msg: "E604: :catch after :finally".to_string(),
                pos: ea.cmdpos,
            });
        } else if self.context[0].borrow().kind != NodeKind::Try
            && self.context[0].borrow().kind != NodeKind::Catch
        {
            return Err(ParseError {
                msg: "E604: :catch without :try".to_string(),
                pos: ea.cmdpos,
            });
        }
        if self.context[0].borrow().kind != NodeKind::Try {
            self.pop_context();
        }
        let mut node = Node::new(NodeKind::Catch);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        if !ends_excmds(&self.reader.borrow().peek()) {
            let p = self.reader.borrow_mut().get();
            let (pattern, _) = self.parse_pattern(&p)?;
            node.pattern = pattern;
        }
        let rc_node = Rc::new(RefCell::new(node));
        self.context[0].borrow_mut().catch.push(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_common(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut end;
        if ea.cmd.flags.contains(&Flag::Trlbar) && !ea.use_filter {
            end = self.separate_nextcmd(&ea)?;
        } else {
            loop {
                end = self.reader.borrow().getpos();
                if self.reader.borrow_mut().getn(1) == "" {
                    break;
                }
            }
        }
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        node.string = self.reader.borrow_mut().getstr(ea.linepos, end);
        node.ea = Some(ea);
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_continue(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if !self.find_context(NodeKind::While) && !self.find_context(NodeKind::For) {
            return Err(ParseError {
                msg: "E586: :continue without :while or :for".to_string(),
                pos: ea.cmdpos,
            });
        }
        let mut node = Node::new(NodeKind::Continue);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_delfunction(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::DelFunction);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.left = Some(Box::new(self.parse_lvalue_func()?));
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_with_exprlist(&mut self, ea: ExArg, kind: NodeKind) -> Result<(), ParseError> {
        let mut node = Node::new(kind);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.list = self
            .parse_exprlist()?
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_echohl(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::EchoHl);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.string = String::new();
        while !ends_excmds(&self.reader.borrow().peek()) {
            node.string.push_str(&self.reader.borrow_mut().get());
        }
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_else(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if self.context[0].borrow().kind != NodeKind::If
            && self.context[0].borrow().kind != NodeKind::ElseIf
        {
            return Err(ParseError {
                msg: "E581: :else without :if".to_string(),
                pos: ea.cmdpos,
            });
        }
        if self.context[0].borrow().kind != NodeKind::If {
            self.pop_context();
        }
        let mut node = Node::new(NodeKind::Else);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        let rc_node = Rc::new(RefCell::new(node));
        self.context[0].borrow_mut().else_ = Some(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_elseif(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if self.context[0].borrow().kind != NodeKind::If
            && self.context[0].borrow().kind != NodeKind::ElseIf
        {
            return Err(ParseError {
                msg: "E581: :elseif without :if".to_string(),
                pos: ea.cmdpos,
            });
        }
        if self.context[0].borrow().kind != NodeKind::If {
            self.pop_context();
        }
        let mut node = Node::new(NodeKind::ElseIf);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.cond = Some(Box::new(self.parse_expr()?));
        let rc_node = Rc::new(RefCell::new(node));
        self.context[0]
            .borrow_mut()
            .elseif
            .push(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_endfor(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if self.context[0].borrow().kind != NodeKind::For {
            return Err(ParseError {
                msg: "E588: :endfor without :for".to_string(),
                pos: ea.cmdpos,
            });
        }
        let mut node = Node::new(NodeKind::EndFor);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.context[0].borrow_mut().end = Some(Box::new(node));
        self.pop_context();
        Ok(())
    }

    fn parse_cmd_endfunction(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.check_missing_endif("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endtry("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endwhile("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endfor("ENDFUNCTION", ea.cmdpos)?;
        if self.context[0].borrow().kind != NodeKind::Function {
            return Err(ParseError {
                msg: "E193: :endfunction not inside a function".to_string(),
                pos: ea.cmdpos,
            });
        }
        self.reader.borrow_mut().get_line();
        let mut node = Node::new(NodeKind::EndFunction);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.context[0].borrow_mut().end = Some(Box::new(node));
        self.pop_context();
        Ok(())
    }

    fn parse_cmd_endif(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if ![NodeKind::If, NodeKind::ElseIf, NodeKind::Else]
            .contains(&self.context[0].borrow().kind)
        {
            return Err(ParseError {
                msg: "E580: :endif without :if".to_string(),
                pos: ea.cmdpos,
            });
        }
        if self.context[0].borrow().kind != NodeKind::If {
            self.pop_context();
        }
        let mut node = Node::new(NodeKind::EndIf);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.context[0].borrow_mut().end = Some(Box::new(node));
        self.pop_context();
        Ok(())
    }

    fn parse_cmd_endtry(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if ![NodeKind::Try, NodeKind::Catch, NodeKind::Finally]
            .contains(&self.context[0].borrow().kind)
        {
            return Err(ParseError {
                msg: "E580: :endtry without :try".to_string(),
                pos: ea.cmdpos,
            });
        }
        if self.context[0].borrow().kind != NodeKind::Try {
            self.pop_context();
        }
        let mut node = Node::new(NodeKind::EndTry);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.context[0].borrow_mut().end = Some(Box::new(node));
        self.pop_context();
        Ok(())
    }

    fn parse_cmd_endwhile(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if self.context[0].borrow().kind != NodeKind::While {
            return Err(ParseError {
                msg: "E588: :endwhile without :while".to_string(),
                pos: ea.cmdpos,
            });
        }
        let mut node = Node::new(NodeKind::EndWhile);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.context[0].borrow_mut().end = Some(Box::new(node));
        self.pop_context();
        Ok(())
    }

    fn parse_cmd_finally(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if ![NodeKind::Try, NodeKind::Catch].contains(&self.context[0].borrow().kind) {
            return Err(ParseError {
                msg: "E606: :finally without :try".to_string(),
                pos: ea.cmdpos,
            });
        }
        if self.context[0].borrow().kind != NodeKind::Try {
            self.pop_context();
        }
        let mut node = Node::new(NodeKind::Finally);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        let rc_node = Rc::new(RefCell::new(node));
        self.context[0].borrow_mut().finally = Some(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_finish(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let rv = self.parse_cmd_common(ea);
        if self.context[0].borrow().kind == NodeKind::TopLevel {
            self.reader.borrow_mut().seek_end();
        }
        rv
    }

    fn parse_cmd_for(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::For);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        let lhs = self.parse_letlhs()?;
        if let Some(left) = lhs.0 {
            node.left = Some(Box::new(left));
        }
        node.list = lhs
            .1
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        node.rest = lhs
            .2
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        self.reader.borrow_mut().skip_white();
        let epos = self.reader.borrow().getpos();
        if self.reader.borrow_mut().read_alpha() != "in" {
            return Err(ParseError {
                msg: "Missing \"in\" after :for".to_string(),
                pos: epos,
            });
        }
        node.right = Some(Box::new(self.parse_expr()?));
        let rc_node = Rc::new(RefCell::new(node));
        self.add_node(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_if(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::If);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.cond = Some(Box::new(self.parse_expr()?));
        let rc_node = Rc::new(RefCell::new(node));
        self.add_node(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_let(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let pos = self.reader.borrow().tell();
        self.reader.borrow_mut().skip_white();
        if ends_excmds(&self.reader.borrow().peek()) {
            self.reader.borrow_mut().seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let lhs = self.parse_letlhs()?;
        self.reader.borrow_mut().skip_white();
        let s1 = self.reader.borrow().peek();
        let s2 = self.reader.borrow().peekn(2);
        if ends_excmds(&s1) || s2 != "+=" && s2 != "-=" && s2 != ".=" && s1 != "=" {
            self.reader.borrow_mut().seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let mut node = Node::new(NodeKind::Let);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        if let Some(left) = lhs.0 {
            node.left = Some(Box::new(left));
        }
        node.list = lhs
            .1
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        node.rest = lhs
            .2
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        if s2 == "+=" || s2 == "-=" || s2 == ".=" {
            self.reader.borrow_mut().getn(2);
            node.op = s2;
        } else if s1 == "=" {
            self.reader.borrow_mut().get();
            node.op = s1;
        } else {
            return self.err("NOT REACHED");
        }
        node.right = Some(Box::new(self.parse_expr()?));
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_loadkeymap(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.reader.borrow_mut().setpos(ea.linepos);
        let mut lines = vec![self.reader.borrow_mut().get_line()];
        loop {
            if self.reader.borrow().peek() == "<EOF>" {
                break;
            }
            lines.push(self.reader.borrow_mut().get_line());
        }
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.string = lines.join("\n");
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_lockvar(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::LockVar);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.reader.borrow_mut().skip_white();
        if isdigit(&self.reader.borrow().peek()) {
            node.depth = Some(
                self.reader
                    .borrow_mut()
                    .read_digit()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
        node.list = self
            .parse_lvaluelist()?
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_lang(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut lines = vec![];
        self.reader.borrow_mut().skip_white();
        if self.reader.borrow().peekn(2) == "<<" {
            self.reader.borrow_mut().getn(2);
            self.reader.borrow_mut().skip_white();
            let mut m = self.reader.borrow_mut().get_line();
            if m == "" {
                m = ".".to_string();
            }
            self.reader.borrow_mut().setpos(ea.linepos);
            lines.push(self.reader.borrow_mut().get_line());
            self.reader.borrow_mut().get();
            loop {
                if self.reader.borrow().peek() == "<EOF>" {
                    break;
                }
                lines.push(self.reader.borrow_mut().get_line());
                if lines.last().unwrap() == &m {
                    break;
                }
                self.reader.borrow_mut().get();
            }
        } else {
            self.reader.borrow_mut().setpos(ea.linepos);
            lines.push(self.reader.borrow_mut().get_line());
        }
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.string = lines.join("\n");
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_return(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if !self.find_context(NodeKind::Function) {
            return Err(ParseError {
                msg: "E133: :return not inside a function".to_string(),
                pos: ea.cmdpos,
            });
        }
        let mut node = Node::new(NodeKind::Return);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.reader.borrow_mut().skip_white();
        let c = self.reader.borrow().peek();
        if c == "\"" || !ends_excmds(&c) {
            node.left = Some(Box::new(self.parse_expr()?));
        }
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_syntax(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut end;
        loop {
            end = self.reader.borrow().getpos();
            let c = self.reader.borrow().peek();
            if c == "/" || c == "'" || c == "\"" {
                self.reader.borrow_mut().get();
                self.parse_pattern(&c)?;
            } else if c == "=" {
                self.reader.borrow_mut().get();
                self.parse_pattern(" ")?;
            } else if ends_excmds(&c) {
                break;
            }
            let peeked = self.reader.borrow().peek();
            if !["/", "'", "\"", "="].contains(&peeked.as_str()) {
                self.reader.borrow_mut().getn(1);
            }
        }
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        node.string = self.reader.borrow_mut().getstr(ea.linepos, end);
        node.ea = Some(ea);
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_throw(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::Throw);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.left = Some(Box::new(self.parse_expr()?));
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_try(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::Try);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        let rc_node = Rc::new(RefCell::new(node));
        self.add_node(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_unlet(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::Unlet);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.list = self
            .parse_lvaluelist()?
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_unlockvar(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::UnlockVar);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        self.reader.borrow_mut().skip_white();
        if isdigit(&self.reader.borrow().peek()) {
            node.depth = Some(
                self.reader
                    .borrow_mut()
                    .read_digit()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
        node.list = self
            .parse_exprlist()?
            .into_iter()
            .map(|n| Box::new(n))
            .collect::<Vec<Box<Node>>>();
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_cmd_while(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut node = Node::new(NodeKind::While);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.cond = Some(Box::new(self.parse_expr()?));
        let rc_node = Rc::new(RefCell::new(node));
        self.add_node(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_cmd_wincmd(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let c = self.reader.borrow_mut().getn(1);
        if c == "" {
            return self.err("E471: Argument required");
        } else if c == "g" || c == "\x07" {
            let c2 = self.reader.borrow_mut().getn(1);
            if c2 == "" || iswhite(&c2) {
                return self.err("E474: Invalid argument");
            }
        }
        let end = self.reader.borrow().getpos();
        self.reader.borrow_mut().skip_white();
        if !ends_excmds(&self.reader.borrow().peek()) {
            return self.err("E474: Invalid argument");
        }
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        node.string = self.reader.borrow_mut().getstr(ea.linepos, end);
        node.ea = Some(ea);
        self.add_node(Rc::new(RefCell::new(node)));
        Ok(())
    }

    fn parse_letlhs(&mut self) -> Result<(Option<Node>, Vec<Node>, Option<Node>), ParseError> {
        let mut tokenizer = Tokenizer::new(Rc::clone(&self.reader));
        let mut nodes = vec![];
        let mut left = None;
        let mut rest = None;
        if tokenizer.peek()?.kind == TokenKind::SqOpen {
            tokenizer.get()?;
            loop {
                nodes.push(self.parse_lvalue()?);
                let mut token = tokenizer.get()?;
                match token.kind {
                    TokenKind::SqClose => {
                        break;
                    }
                    TokenKind::Comma => {
                        continue;
                    }
                    TokenKind::Semicolon => {
                        rest = Some(self.parse_lvalue()?);
                        token = tokenizer.get()?;
                        if token.kind == TokenKind::SqClose {
                            break;
                        } else {
                            return Err(ParseError {
                                msg: format!("E475: Invalid argument: {}", token.value),
                                pos: token.pos,
                            });
                        }
                    }
                    _ => {
                        return Err(ParseError {
                            msg: format!("E475: Invalid argument: {}", token.value),
                            pos: token.pos,
                        });
                    }
                }
            }
        } else {
            left = Some(self.parse_lvalue()?);
        }
        Ok((left, nodes, rest))
    }

    fn parse_cmd_function(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let pos = self.reader.borrow().tell();
        self.reader.borrow_mut().skip_white();
        if ends_excmds(&self.reader.borrow().peek()) || self.reader.borrow().peek() == "/" {
            self.reader.borrow_mut().seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let left = self.parse_lvalue_func()?;
        self.reader.borrow_mut().skip_white();
        if left.kind == NodeKind::Identifier {
            if !left.value.starts_with("<")
                && !left.value.starts_with(|c: char| c.is_uppercase())
                && !left.value.contains(":")
                && !left.value.contains("#")
            {
                return Err(ParseError {
                    msg: format!(
                        "E128: Function name must start with a capital or contain a colon: {}",
                        left.value
                    ),
                    pos: left.pos,
                });
            }
        }
        if self.reader.borrow().peek() != "(" {
            self.reader.borrow_mut().seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let mut node = Node::new(NodeKind::Function);
        node.pos = ea.cmdpos;
        node.ea = Some(ea);
        node.left = Some(Box::new(left));
        self.reader.borrow_mut().getn(1);
        let mut tokenizer = Tokenizer::new(Rc::clone(&self.reader));
        if tokenizer.peek()?.kind == TokenKind::PClose {
            tokenizer.get()?;
        } else {
            let mut named: Vec<String> = vec![];
            loop {
                let mut varnode = Node::new(NodeKind::Identifier);
                let mut token = tokenizer.get()?;
                if token.kind == TokenKind::Identifier {
                    if !isargname(&token.value)
                        || token.value == "firstline"
                        || token.value == "lastline"
                    {
                        return Err(ParseError {
                            msg: format!("E125: Illegal argument: {}", token.value),
                            pos: token.pos,
                        });
                    } else if named.contains(&token.value) {
                        return Err(ParseError {
                            msg: format!("E853: Duplicate argument name: {}", token.value),
                            pos: token.pos,
                        });
                    }
                    named.push(token.value.clone());
                    varnode.pos = token.pos;
                    varnode.value = token.value;
                    node.rlist.push(Box::new(varnode));
                    if iswhite(&self.reader.borrow().peek())
                        && tokenizer.peek()?.kind == TokenKind::Comma
                    {
                        return self
                            .err("E475: Invalid argument: White space is not allowed before comma");
                    }
                    token = tokenizer.get()?;
                    if token.kind == TokenKind::Comma {
                        if tokenizer.peek()?.kind == TokenKind::PClose {
                            tokenizer.get()?;
                            break;
                        }
                    } else if token.kind == TokenKind::PClose {
                        break;
                    } else {
                        return Err(ParseError {
                            msg: format!("unexpected token: {}", token.value),
                            pos: token.pos,
                        });
                    }
                } else if token.kind == TokenKind::DotDotDot {
                    varnode.pos = token.pos;
                    varnode.value = token.value;
                    node.rlist.push(Box::new(varnode));
                    token = tokenizer.get()?;
                    if token.kind == TokenKind::PClose {
                        break;
                    } else {
                        return Err(ParseError {
                            msg: format!("unexpected token: {}", token.value),
                            pos: token.pos,
                        });
                    }
                } else {
                    return Err(ParseError {
                        msg: format!("unexpected token: {}", token.value),
                        pos: token.pos,
                    });
                }
            }
        }
        loop {
            self.reader.borrow_mut().skip_white();
            let epos = self.reader.borrow().getpos();
            let key = self.reader.borrow_mut().read_alpha();
            match key.as_str() {
                "" => {
                    break;
                }
                "range" => node.attrs.push("range".to_string()),
                "abort" => node.attrs.push("abort".to_string()),
                "dict" => node.attrs.push("dict".to_string()),
                "closure" => node.attrs.push("closure".to_string()),
                _ => {
                    return Err(ParseError {
                        msg: format!("unexpected token: {}", key),
                        pos: epos,
                    });
                }
            }
        }
        let rc_node = Rc::new(RefCell::new(node));
        self.add_node(Rc::clone(&rc_node));
        self.push_context(rc_node);
        Ok(())
    }

    fn parse_exprlist(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes = vec![];
        loop {
            self.reader.borrow_mut().skip_white();
            let c = self.reader.borrow().peek();
            if c != "\"" && ends_excmds(&c) {
                break;
            }
            let node = self.parse_expr()?;
            nodes.push(node);
        }
        Ok(nodes)
    }

    fn parse_lvalue(&mut self) -> Result<Node, ParseError> {
        let mut parser = NodeParser::new(Rc::clone(&self.reader));
        let node = parser.parse_lv()?;
        if node.kind == NodeKind::Identifier {
            if !isvarname(&node.value) {
                return Err(ParseError {
                    msg: format!("E461: Illegal variable name: {}", node.value),
                    pos: node.pos,
                });
            }
        }
        match node.kind {
            NodeKind::Identifier
            | NodeKind::CurlyName
            | NodeKind::Subscript
            | NodeKind::Slice
            | NodeKind::Dot
            | NodeKind::Option
            | NodeKind::Env
            | NodeKind::Reg => Ok(node),
            _ => Err(ParseError {
                msg: "Invalid expression".to_string(),
                pos: node.pos,
            }),
        }
    }

    fn parse_lvaluelist(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes = vec![];
        nodes.push(self.parse_expr()?);
        loop {
            self.reader.borrow_mut().skip_white();
            if ends_excmds(&self.reader.borrow().peek()) {
                break;
            }
            nodes.push(self.parse_lvalue()?);
        }
        Ok(nodes)
    }

    fn parse_lvalue_func(&mut self) -> Result<Node, ParseError> {
        let mut parser = NodeParser::new(Rc::clone(&self.reader));
        let node = parser.parse_lv()?;
        match node.kind {
            NodeKind::Identifier
            | NodeKind::CurlyName
            | NodeKind::Subscript
            | NodeKind::Dot
            | NodeKind::Option
            | NodeKind::Env
            | NodeKind::Reg => Ok(node),
            _ => Err(ParseError {
                msg: "Invalid expression".to_string(),
                pos: node.pos,
            }),
        }
    }

    fn separate_nextcmd(&mut self, ea: &ExArg) -> Result<Position, ParseError> {
        if ["vimgrep", "vimgrepadd", "lvimgrep", "lvimgrepadd"].contains(&ea.cmd.name.as_str()) {
            self.skip_vimgrep_pat()?;
        }
        let mut pc = String::new();
        let mut end = self.reader.borrow().getpos();
        let mut nospend = end;
        loop {
            end = self.reader.borrow().getpos();
            if !iswhite(&pc) {
                nospend = end;
            }
            let mut c = self.reader.borrow().peek();
            if c == "\n" || c == "<EOF>" {
                break;
            } else if c == "\x16" {
                self.reader.borrow_mut().get();
                end = self.reader.borrow().getpos();
                nospend = end;
                c = self.reader.borrow().peek();
                if c == "\n" || c == "<EOF>" {
                    break;
                }
                self.reader.borrow_mut().get();
            } else if self.reader.borrow().peekn(2) == "`="
                && (ea.cmd.flags.contains(&Flag::Xfile)
                    || ea.cmd.flags.contains(&Flag::Files)
                    || ea.cmd.flags.contains(&Flag::File1))
            {
                self.reader.borrow_mut().getn(2);
                self.parse_expr()?;
                c = self.reader.borrow().peekn(1);
                if c != "`" {
                    return self.err(&format!("unexpected character: {}", c));
                }
                self.reader.borrow_mut().getn(1);
            } else if ["|", "\n", "\""].contains(&c.as_str())
                && !ea.cmd.flags.contains(&Flag::Notrlcom)
                && (ea.cmd.name != "@" && ea.cmd.name != "*"
                    || self.reader.borrow().getpos() != ea.argpos)
                && (ea.cmd.name != "redir"
                    || self.reader.borrow().getpos().cursor != ea.argpos.cursor + 1
                    || pc != "@")
            {
                if !ea.cmd.flags.contains(&Flag::Usectrlv) && pc == "\\" {
                    self.reader.borrow_mut().get();
                } else {
                    break;
                }
            } else {
                self.reader.borrow_mut().get();
            }
            pc = c
        }
        if !ea.cmd.flags.contains(&Flag::Notrlcom) {
            end = nospend;
        }
        Ok(end)
    }

    fn skip_vimgrep_pat(&mut self) -> Result<(), ParseError> {
        let c = self.reader.borrow().peek();
        if c == "\n" {
        } else if iswordc(&c) {
            self.reader.borrow_mut().read_nonwhite();
        } else {
            let c = self.reader.borrow_mut().get();
            let (_, endc) = self.parse_pattern(&c)?;
            if c != endc {
                return Ok(());
            }
            while self.reader.borrow().peek() == "g" || self.reader.borrow().peek() == "j" {
                self.reader.borrow_mut().get();
            }
        }
        Ok(())
    }

    fn parse_argcmd(&mut self) {
        if self.reader.borrow().peek() == "+" {
            self.reader.borrow_mut().get();
            if self.reader.borrow().peek() != " " {
                self.read_cmdarg();
            }
        }
    }

    fn read_cmdarg(&mut self) {
        loop {
            let c = self.reader.borrow().peekn(1);
            if c == "" || c.chars().collect::<Vec<char>>()[0].is_whitespace() {
                break;
            }
            self.reader.borrow_mut().get();
            if c == "\\" {
                self.reader.borrow_mut().get();
            }
        }
    }

    fn parse_argopt(&mut self) -> Result<(), ParseError> {
        lazy_static! {
            static ref BIN_RE: Regex = Regex::new("^\\+\\+bin\\b").unwrap();
            static ref NOBIN_RE: Regex = Regex::new("^\\+\\+nobin\\b").unwrap();
            static ref EDIT_RE: Regex = Regex::new("^\\+\\+edit\\b").unwrap();
            static ref FF_RE: Regex = Regex::new("^\\+\\+ff=(dos|unix|mac)\\b").unwrap();
            static ref FILEFORMAT_RE: Regex =
                Regex::new("^\\+\\+fileformat=(dos|unix|mac)\\b").unwrap();
            static ref ENC_RE: Regex = Regex::new("^\\+\\+enc=\\S").unwrap();
            static ref ENCODING_RE: Regex = Regex::new("^\\+\\+encoding=\\S").unwrap();
            static ref BAD_OUTER_RE: Regex = Regex::new("^\\+\\+bad=(keep|drop|.)\\b").unwrap();
            static ref BAD_INNER_RE: Regex = Regex::new("^\\+\\+bad=(keep|drop)").unwrap();
        }
        while self.reader.borrow().peekn(2) == "++" {
            let s = self.reader.borrow().peekn(20);
            if BIN_RE.is_match(&s) {
                self.reader.borrow_mut().getn(5);
            } else if NOBIN_RE.is_match(&s) {
                self.reader.borrow_mut().getn(7);
            } else if EDIT_RE.is_match(&s) {
                self.reader.borrow_mut().getn(6);
            } else if FF_RE.is_match(&s) {
                self.reader.borrow_mut().getn(5);
            } else if FILEFORMAT_RE.is_match(&s) {
                self.reader.borrow_mut().getn(13);
            } else if ENC_RE.is_match(&s) {
                self.reader.borrow_mut().getn(6);
                self.reader.borrow_mut().read_nonwhite();
            } else if ENCODING_RE.is_match(&s) {
                self.reader.borrow_mut().getn(11);
                self.reader.borrow_mut().read_nonwhite();
            } else if BAD_OUTER_RE.is_match(&s) {
                self.reader.borrow_mut().getn(6);
                if BAD_INNER_RE.is_match(&s) {
                    self.reader.borrow_mut().getn(4);
                } else {
                    self.reader.borrow_mut().get();
                }
            } else if s.starts_with("++") {
                return self.err("E474: Invalid Argument");
            } else {
                break;
            }
            self.reader.borrow_mut().skip_white();
        }
        Ok(())
    }

    fn find_command(&mut self) -> Option<Command> {
        let c = self.reader.borrow().peek();
        let mut name = "".to_string();
        lazy_static! {
            static ref SUB_RE: Regex = Regex::new("^s(c[^sr][^i][^p]|g|i[^mlg]|I|r[^e])").unwrap();
            static ref DEL_RE: Regex = Regex::new("^d(elete|elet|ele|el|e)[lp]$").unwrap();
        }
        if c == "k" {
            name.push_str(&self.reader.borrow_mut().get());
        } else if c == "s" && SUB_RE.is_match(&self.reader.borrow().peekn(5)) {
            self.reader.borrow_mut().get();
            name.push_str("substitute");
        } else if ["@", "*", "!", "=", ">", "<", "&", "~", "#"].contains(&c.as_str()) {
            name.push_str(&self.reader.borrow_mut().get());
        } else if self.reader.borrow().peekn(2) == "py" {
            name.push_str(&self.reader.borrow_mut().read_alnum());
        } else {
            let pos = self.reader.borrow().tell();
            name.push_str(&self.reader.borrow_mut().read_alpha());
            if name != "del" && DEL_RE.is_match(&name) {
                self.reader.borrow_mut().seek_set(pos);
                name = self.reader.borrow_mut().getn(name.len() - 1);
            }
        }
        if name == "" {
            return None;
        }
        // TODO: add command cache here?
        let mut cmd: Option<Command> = None;
        for command in self.commands.iter() {
            if command.name.starts_with(&name) && name.len() >= command.minlen {
                cmd = Some(command.clone());
                break;
            }
        }
        if cmd.is_none() && name.starts_with(|c: char| c.is_uppercase()) {
            name.push_str(&self.reader.borrow_mut().read_alnum());
            cmd = Some(Command {
                name: name,
                minlen: 0,
                flags: vec![Flag::Usercmd],
                parser: ParserKind::Usercmd,
            })
        }
        cmd
    }

    fn parse_cmd_modifier_range(&mut self, ea: ExArg) {
        let mut node = Node::new(NodeKind::ExCmd);
        node.pos = ea.cmdpos;
        let pos = self.reader.borrow().getpos();
        node.string = self.reader.borrow_mut().getstr(ea.linepos, pos);
        node.ea = Some(ea);
        self.add_node(Rc::new(RefCell::new(node)));
    }

    fn parse_trail(&mut self) -> Result<(), ParseError> {
        self.reader.borrow_mut().skip_white();
        let c = self.reader.borrow().peek();
        match c.as_str() {
            "<EOF>" => Ok(()),
            "\n" | "|" => {
                self.reader.borrow_mut().get();
                Ok(())
            }
            "\"" => {
                self.parse_comment()?;
                self.reader.borrow_mut().get();
                Ok(())
            }
            _ => self.err(&format!("E488: Trailing characters: {}", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let reader = Reader::from_file("auto-gutters.vim").unwrap();
        let mut parser = Parser::new(reader, false);
        println!("{:#?}", parser.parse());
        assert!(false);
    }
}
