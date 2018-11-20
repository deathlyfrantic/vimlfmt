use super::{isargname, isdigit, isnamec, isvarname, iswhite, iswordc, ParseError, Position};
use command::{neovim_commands, vim_commands, Command, Flag, ParserKind};
use exarg::ExArg;
use modifier::Modifier;
use node::Node;
use reader::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use token::{Token, TokenKind, Tokenizer};

const MAX_FUNC_ARGS: usize = 20;

fn ends_excmds(s: &str) -> bool {
    ["", "|", "\"", "<EOF>", "\n"].contains(&s)
}

#[derive(Debug)]
pub struct Parser<'a> {
    reader: &'a Reader,
    context: Vec<Node>,
    commands: HashMap<String, Rc<Command>>,
}

impl<'a> Parser<'a> {
    pub fn new(reader: &'a Reader, neovim: bool) -> Parser {
        Parser {
            reader,
            context: vec![],
            commands: if neovim {
                neovim_commands()
            } else {
                vim_commands()
            },
        }
    }

    fn ensure_context(&self) {
        if self.context.len() == 0 {
            panic!("no context found");
        }
    }

    fn current_context(&self) -> &Node {
        self.ensure_context();
        &self.context[0]
    }

    fn current_context_mut(&mut self) -> &mut Node {
        self.ensure_context();
        &mut self.context[0]
    }

    fn push_context(&mut self, node: Node) {
        self.context.insert(0, node)
    }

    fn pop_context(&mut self) -> Node {
        self.ensure_context();
        self.context.remove(0)
    }

    fn collapse_context(&mut self) {
        let node = self.pop_context();
        match node {
            Node::Catch { .. } => {
                if let Node::Try {
                    ref mut catches, ..
                } = self.current_context_mut()
                {
                    catches.push(Box::new(node));
                } else {
                    panic!("Catch node parent is not a Try node");
                }
            }
            Node::Finally { .. } => {
                if let Node::Try {
                    ref mut finally, ..
                } = self.current_context_mut()
                {
                    *finally = Some(Box::new(node));
                } else {
                    panic!("Finally node parent is not a Try node");
                }
            }
            Node::ElseIf { .. } => {
                if let Node::If {
                    ref mut elseifs, ..
                } = self.current_context_mut()
                {
                    elseifs.push(Box::new(node));
                } else {
                    panic!("ElseIf node parent is not an If node");
                }
            }
            Node::Else { .. } => {
                if let Node::If { ref mut else_, .. } = self.current_context_mut() {
                    *else_ = Some(Box::new(node));
                } else {
                    panic!("Else node parent is not an If node");
                }
            }
            _ => {
                self.add_node(node);
            }
        };
    }

    fn find_context<T>(&self, func: T) -> bool
    where
        T: Fn(&Node) -> bool,
    {
        self.context.iter().any(|node| func(&node))
    }

    fn add_node(&mut self, node: Node) {
        match self.current_context_mut() {
            Node::Catch { ref mut body, .. }
            | Node::Else { ref mut body, .. }
            | Node::ElseIf { ref mut body, .. }
            | Node::Finally { ref mut body, .. }
            | Node::For { ref mut body, .. }
            | Node::Function { ref mut body, .. }
            | Node::If { ref mut body, .. }
            | Node::TopLevel { ref mut body, .. }
            | Node::Try { ref mut body, .. }
            | Node::While { ref mut body, .. } => {
                body.push(Box::new(node));
            }
            _ => (),
        };
    }

    fn check_missing_endfunction(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if let Node::Function { .. } = self.current_context() {
            Err(ParseError {
                msg: format!("E126: Missing :endfunction:    {}", end),
                pos,
            })
        } else {
            Ok(())
        }
    }

    fn check_missing_endif(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        match self.current_context() {
            Node::If { .. } | Node::ElseIf { .. } | Node::Else { .. } => Err(ParseError {
                msg: format!("E126: Missing :endif:    {}", end),
                pos,
            }),
            _ => Ok(()),
        }
    }

    fn check_missing_endtry(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        match self.current_context() {
            Node::Try { .. } | Node::Catch { .. } | Node::Finally { .. } => Err(ParseError {
                msg: format!("E126: Missing :endtry:    {}", end),
                pos,
            }),
            _ => Ok(()),
        }
    }

    fn check_missing_endwhile(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if let Node::While { .. } = self.current_context() {
            Err(ParseError {
                msg: format!("E126: Missing :endwhile:    {}", end),
                pos,
            })
        } else {
            Ok(())
        }
    }

    fn check_missing_endfor(&self, end: &str, pos: Position) -> Result<(), ParseError> {
        if let Node::For { .. } = self.current_context() {
            Err(ParseError {
                msg: format!("E126: Missing :endfor:    {}", end),
                pos,
            })
        } else {
            Ok(())
        }
    }

    fn err<T>(&self, msg: &str) -> Result<T, ParseError> {
        Err(ParseError {
            msg: msg.to_string(),
            pos: self.reader.getpos(),
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let pos = self.reader.getpos();
        self.push_context(Node::TopLevel { pos, body: vec![] });
        while self.reader.peek() != "<EOF>" {
            self.parse_one_cmd()?;
        }
        self.check_missing_endfunction("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endif("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endtry("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endwhile("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endfor("TOPLEVEL", self.reader.getpos())?;
        Ok(self.pop_context())
    }

    fn parse_expr(&mut self) -> Result<Node, ParseError> {
        ExprParser::new(self.reader).parse()
    }

    fn parse_one_cmd(&mut self) -> Result<(), ParseError> {
        let mut ea = ExArg::new();
        if self.reader.peekn(2) == "#!" {
            self.parse_shebang()?;
            return Ok(());
        }
        self.reader.skip_white_and_colon();
        if self.reader.peek() == "\n" {
            self.reader.get();
            return Ok(());
        }
        if self.reader.peek() == "\"" {
            self.parse_comment()?;
            self.reader.get();
            return Ok(());
        }
        ea.linepos = self.reader.getpos();
        ea.modifiers = self.parse_command_modifiers()?;
        ea.range = self.parse_range()?;
        self.parse_command(ea)?;
        self.parse_trail()?;
        Ok(())
    }

    fn parse_shebang(&mut self) -> Result<(), ParseError> {
        let sb = self.reader.getn(2);
        if sb != "#!" {
            return self.err(&format!("unexpected characters: {}", sb));
        }
        let pos = self.reader.getpos();
        let value = self.reader.get_line();
        self.add_node(Node::Shebang { pos, value });
        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), ParseError> {
        let pos = self.reader.getpos();
        let c = self.reader.get();
        if c != "\"" {
            return Err(ParseError {
                msg: format!("unexpected character: {}", c),
                pos,
            });
        }
        self.add_node(Node::Comment {
            pos,
            value: self.reader.get_line(),
        });
        Ok(())
    }

    fn parse_command_modifiers(&mut self) -> Result<Vec<Modifier>, ParseError> {
        let mut modifiers: Vec<Modifier> = vec![];
        loop {
            let pos = self.reader.tell();
            let mut d = "".to_string();
            let peeked = self.reader.peek();
            if isdigit(&peeked) {
                d = self.reader.read_digit();
                self.reader.skip_white();
            }
            let k = self.reader.read_alpha();
            let c = self.reader.peek();
            self.reader.skip_white();
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
                        self.reader.get();
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
                    self.reader.seek_set(pos);
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
                self.reader.skip_white();
                let c = self.reader.peek();
                match c.as_str() {
                    "" => break,
                    "." | "$" => tokens.push(self.reader.get()),
                    "'" => {
                        if self.reader.peek_ahead(1) == "\n" {
                            break;
                        }
                        tokens.push(self.reader.getn(2));
                    }
                    "/" | "?" => {
                        self.reader.get();
                        let (pattern, _) = self.parse_pattern(&c)?;
                        tokens.push(pattern);
                    }
                    "\\" => {
                        let m = self.reader.peek_ahead(1);
                        if m == "&" || m == "?" || m == "/" {
                            tokens.push(self.reader.getn(2));
                        } else {
                            return self.err("E10: \\\\ should be followed by /, ? or &");
                        }
                    }
                    _ if isdigit(&c) => {
                        tokens.push(self.reader.read_digit());
                    }
                    _ => (),
                }
                loop {
                    self.reader.skip_white();
                    if self.reader.peek() == "\n" {
                        break;
                    }
                    let n = self.reader.read_integer();
                    if n == "" {
                        break;
                    }
                    tokens.push(n);
                }
                if self.reader.peek() != "/" && self.reader.peek() != "?" {
                    break;
                }
            }
            let p = self.reader.peek();
            if p == "%" || p == "*" {
                tokens.push(self.reader.get());
            }
            let p = self.reader.peek();
            if p == ";" || p == "," {
                tokens.push(self.reader.get());
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
            let c = self.reader.getn(1);
            if c == "" {
                break;
            }
            if c == delimiter && in_bracket == 0 {
                endc = c;
                break;
            }
            pattern.push_str(&c);
            if c == "\\" {
                let c = self.reader.peek();
                if c == "\n" {
                    return self.err("E682: Invalid search pattern or delimiter");
                }
                self.reader.getn(1);
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
        self.reader.skip_white_and_colon();
        ea.cmdpos = self.reader.getpos();
        let peeked = self.reader.peek();
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
                    self.reader.peek_line()
                ));
            }
        }
        if self.reader.peek() == "!"
            && !["substitute", "smagic", "snomagic"].contains(&ea.cmd.name.as_str())
        {
            self.reader.get();
            ea.bang = true;
        }
        if !ea.cmd.flags.contains(&Flag::Bang) && ea.bang && !ea.cmd.flags.contains(&Flag::UserCmd)
        {
            return Err(ParseError {
                msg: "E477: No ! allowed".to_string(),
                pos: ea.cmdpos,
            });
        }
        if ea.cmd.name != "!" {
            self.reader.skip_white();
        }
        ea.argpos = self.reader.getpos();
        if ea.cmd.flags.contains(&Flag::ArgOpt) {
            self.parse_argopt()?;
        }
        if ea.cmd.name == "write" || ea.cmd.name == "update" {
            if self.reader.peek() == ">" {
                if self.reader.peek_ahead(1) == ">" {
                    return self.err("E494: Use w or w>>");
                }
                self.reader.seek_cur(2);
                self.reader.skip_white();
            } else if self.reader.peek() == "!" && ea.cmd.name == "write" {
                self.reader.get();
                ea.use_filter = true;
            }
        }
        if ea.cmd.name == "read" {
            if ea.bang {
                ea.use_filter = true;
                ea.bang = false;
            } else if self.reader.peek() == "!" {
                self.reader.get();
                ea.use_filter = true;
            }
        }
        if ea.cmd.name == "<" || ea.cmd.name == ">" {
            while self.reader.peek() == ea.cmd.name {
                self.reader.get();
            }
            self.reader.skip_white();
        }
        if ea.cmd.flags.contains(&Flag::EditCmd) && !ea.use_filter {
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
            ParserKind::Common | ParserKind::UserCmd => self.parse_cmd_common(ea),
            ParserKind::Continue => self.parse_cmd_continue(ea),
            ParserKind::DelFunction => self.parse_cmd_delfunction(ea),
            ParserKind::Echo => self.parse_cmd_echo(ea, "echo"),
            ParserKind::EchoErr => self.parse_cmd_echo(ea, "echoerr"),
            ParserKind::EchoHl => self.parse_cmd_echohl(ea),
            ParserKind::EchoMsg => self.parse_cmd_echo(ea, "echomsg"),
            ParserKind::EchoN => self.parse_cmd_echo(ea, "echon"),
            ParserKind::Else => self.parse_cmd_else(ea),
            ParserKind::ElseIf => self.parse_cmd_elseif(ea),
            ParserKind::EndFor => self.parse_cmd_endfor(ea),
            ParserKind::EndFunction => self.parse_cmd_endfunction(ea),
            ParserKind::EndIf => self.parse_cmd_endif(ea),
            ParserKind::EndTry => self.parse_cmd_endtry(ea),
            ParserKind::EndWhile => self.parse_cmd_endwhile(ea),
            ParserKind::Execute => self.parse_cmd_execute(ea),
            ParserKind::Finally => self.parse_cmd_finally(ea),
            ParserKind::Finish => self.parse_cmd_finish(ea),
            ParserKind::For => self.parse_cmd_for(ea),
            ParserKind::Function => self.parse_cmd_function(ea),
            ParserKind::If => self.parse_cmd_if(ea),
            ParserKind::Let => self.parse_cmd_let(ea),
            ParserKind::LoadKeymap => self.parse_cmd_loadkeymap(ea),
            ParserKind::LockVar => self.parse_cmd_lockvar(ea),
            ParserKind::Lang => self.parse_cmd_lang(ea),
            ParserKind::Return => self.parse_cmd_return(ea),
            ParserKind::Syntax => self.parse_cmd_syntax(ea),
            ParserKind::Throw => self.parse_cmd_throw(ea),
            ParserKind::Try => self.parse_cmd_try(ea),
            ParserKind::Unlet => self.parse_cmd_unlet(ea),
            ParserKind::UnlockVar => self.parse_cmd_unlockvar(ea),
            ParserKind::While => self.parse_cmd_while(ea),
            ParserKind::WinCmd => self.parse_cmd_wincmd(ea),
        }
    }

    fn parse_cmd_append(&mut self, ea: ExArg) {
        self.reader.setpos(ea.linepos);
        let cmdline = self.reader.get_line();
        self.reader.get();
        let mut lines = vec![cmdline];
        loop {
            if self.reader.peek() == "<EOF>" {
                break;
            }
            let line = self.reader.get_line();
            lines.push(line);
            if lines.last().unwrap() == "." {
                break;
            }
            self.reader.get();
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            ea,
            value: lines.join("\n"),
        });
    }

    fn parse_cmd_break(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if !self.find_context(Node::is_while) && !self.find_context(Node::is_for) {
            return self.err("E587: :break without :while or :for");
        }
        self.add_node(Node::Break { pos: ea.cmdpos, ea });
        Ok(())
    }

    fn parse_cmd_call(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let pos = ea.cmdpos;
        self.reader.skip_white();
        if ends_excmds(&self.reader.peek()) {
            return self.err("E471: Argument required");
        }
        let left = self.parse_expr()?;
        match &left {
            &Node::Call { .. } => {
                self.add_node(Node::ExCall {
                    pos,
                    ea,
                    left: Box::new(left),
                });
                Ok(())
            }
            _ => Err(ParseError {
                msg: "Not a function call".to_string(),
                pos,
            }),
        }
    }

    fn parse_cmd_catch(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::Try { .. } => (),
            Node::Catch { .. } => {
                self.collapse_context();
            }
            Node::Finally { .. } => {
                return Err(ParseError {
                    msg: "E604: :catch after :finally".to_string(),
                    pos: ea.cmdpos,
                });
            }
            _ => {
                return Err(ParseError {
                    msg: "E604: :catch without :try".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        let pattern = if !ends_excmds(&self.reader.peek()) {
            let p = self.reader.get();
            let (pat, _) = self.parse_pattern(&p)?;
            Some(pat)
        } else {
            None
        };
        self.push_context(Node::Catch {
            pos: ea.cmdpos,
            ea,
            pattern,
            body: vec![],
        });
        Ok(())
    }

    fn parse_cmd_common(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut end;
        if ea.cmd.flags.contains(&Flag::TrlBar) && !ea.use_filter {
            end = self.separate_nextcmd(&ea)?;
        } else {
            loop {
                end = self.reader.getpos();
                if self.reader.getn(1) == "" {
                    break;
                }
            }
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            value: self.reader.getstr(ea.linepos, end),
            ea,
        });
        Ok(())
    }

    fn parse_cmd_continue(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if !self.find_context(Node::is_while) && !self.find_context(Node::is_for) {
            return Err(ParseError {
                msg: "E586: :continue without :while or :for".to_string(),
                pos: ea.cmdpos,
            });
        }
        self.add_node(Node::Continue { pos: ea.cmdpos, ea });
        Ok(())
    }

    fn parse_cmd_delfunction(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let node = Node::DelFunction {
            pos: ea.cmdpos,
            ea,
            left: Box::new(self.parse_lvalue_func()?),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_echo(&mut self, ea: ExArg, cmd: &str) -> Result<(), ParseError> {
        let node = Node::Echo {
            pos: ea.cmdpos,
            ea,
            cmd: cmd.to_string(),
            list: self
                .parse_exprlist()?
                .into_iter()
                .map(|n| Box::new(n))
                .collect::<Vec<Box<Node>>>(),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_execute(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let node = Node::Execute {
            pos: ea.cmdpos,
            ea,
            list: self
                .parse_exprlist()?
                .into_iter()
                .map(|n| Box::new(n))
                .collect::<Vec<Box<Node>>>(),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_echohl(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut value = String::new();
        while !ends_excmds(&self.reader.peek()) {
            value.push_str(&self.reader.get());
        }
        self.add_node(Node::EchoHl {
            pos: ea.cmdpos,
            ea,
            value,
        });
        Ok(())
    }

    fn parse_cmd_else(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::If { .. } => (),
            Node::ElseIf { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E581: :else without :if".to_string(),
                    pos: ea.cmdpos,
                })
            }
        };
        self.push_context(Node::Else {
            pos: ea.cmdpos,
            ea,
            body: vec![],
        });
        Ok(())
    }

    fn parse_cmd_elseif(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::If { .. } => (),
            Node::ElseIf { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E582: :elseif without :if".to_string(),
                    pos: ea.cmdpos,
                })
            }
        };
        let node = Node::ElseIf {
            pos: ea.cmdpos,
            ea,
            cond: Box::new(self.parse_expr()?),
            body: vec![],
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_endfor(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context_mut() {
            Node::For { ref mut end, .. } => {
                let node = Node::End { pos: ea.cmdpos, ea };
                *end = Some(Box::new(node));
            }
            _ => {
                return Err(ParseError {
                    msg: "E588: :endfor without :for".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        self.collapse_context();
        Ok(())
    }

    fn parse_cmd_endfunction(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.check_missing_endif("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endtry("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endwhile("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endfor("ENDFUNCTION", ea.cmdpos)?;
        match self.current_context_mut() {
            Node::Function { ref mut end, .. } => {
                let node = Node::End { pos: ea.cmdpos, ea };
                *end = Some(Box::new(node));
            }
            _ => {
                return Err(ParseError {
                    msg: "E193: :endfunction not inside a function".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        self.reader.get_line();
        self.collapse_context();
        Ok(())
    }

    fn parse_cmd_endif(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::If { .. } => (),
            Node::ElseIf { .. } | Node::Else { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E580: :endif without :if".to_string(),
                    pos: ea.cmdpos,
                })
            }
        };
        if let Node::If { ref mut end, .. } = self.current_context_mut() {
            let node = Node::End { pos: ea.cmdpos, ea };
            *end = Some(Box::new(node));
        }
        self.collapse_context();
        Ok(())
    }

    fn parse_cmd_endtry(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::Try { .. } => (),
            Node::Catch { .. } | Node::Finally { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E580: :endtry without :try".to_string(),
                    pos: ea.cmdpos,
                })
            }
        };
        if let Node::Try { ref mut end, .. } = self.current_context_mut() {
            let node = Node::End { pos: ea.cmdpos, ea };
            *end = Some(Box::new(node));
        }
        self.collapse_context();
        Ok(())
    }

    fn parse_cmd_endwhile(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::While { .. } => {
                let node = Node::End { pos: ea.cmdpos, ea };
                if let Node::While { ref mut end, .. } = self.current_context_mut() {
                    *end = Some(Box::new(node));
                }
                self.collapse_context();
                Ok(())
            }
            _ => Err(ParseError {
                msg: "E588: :endwhile without :while".to_string(),
                pos: ea.cmdpos,
            }),
        }
    }

    fn parse_cmd_finally(&mut self, ea: ExArg) -> Result<(), ParseError> {
        match self.current_context() {
            Node::Try { .. } => (),
            Node::Catch { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E606: :finally without :try".to_string(),
                    pos: ea.cmdpos,
                })
            }
        };
        self.push_context(Node::Finally {
            pos: ea.cmdpos,
            ea,
            body: vec![],
        });
        Ok(())
    }

    fn parse_cmd_finish(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let rv = self.parse_cmd_common(ea);
        if let Node::TopLevel { .. } = self.current_context() {
            self.reader.seek_end();
        }
        rv
    }

    fn parse_cmd_for(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let (var, list, rest) = self.parse_letlhs()?;
        self.reader.skip_white();
        let epos = self.reader.getpos();
        if self.reader.read_alpha() != "in" {
            return Err(ParseError {
                msg: "Missing \"in\" after :for".to_string(),
                pos: epos,
            });
        }
        let right = Box::new(self.parse_expr()?);
        self.push_context(Node::For {
            pos: ea.cmdpos,
            ea,
            var,
            list,
            rest,
            right,
            body: vec![],
            end: None,
        });
        Ok(())
    }

    fn parse_cmd_if(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let node = Node::If {
            pos: ea.cmdpos,
            ea,
            cond: Box::new(self.parse_expr()?),
            elseifs: vec![],
            else_: None,
            body: vec![],
            end: None,
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_let(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let pos = self.reader.tell();
        self.reader.skip_white();
        if ends_excmds(&self.reader.peek()) {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let (var, list, rest) = self.parse_letlhs()?;
        self.reader.skip_white();
        let s1 = self.reader.peek();
        let s2 = self.reader.peekn(2);
        if ends_excmds(&s1) || s2 != "+=" && s2 != "-=" && s2 != ".=" && s1 != "=" {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let op = if s2 == "+=" || s2 == "-=" || s2 == ".=" {
            self.reader.getn(2);
            s2
        } else if s1 == "=" {
            self.reader.get();
            s1
        } else {
            return self.err("NOT REACHED");
        };
        let node = Node::Let {
            pos: ea.cmdpos,
            ea,
            var,
            list,
            rest,
            op,
            right: Box::new(self.parse_expr()?),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_loadkeymap(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.reader.setpos(ea.linepos);
        let mut lines = vec![self.reader.get_line()];
        loop {
            if self.reader.peek() == "<EOF>" {
                break;
            }
            lines.push(self.reader.get_line());
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            ea,
            value: lines.join("\n"),
        });
        Ok(())
    }

    fn parse_cmd_lockvar(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.reader.skip_white();
        let depth = if isdigit(&self.reader.peek()) {
            Some(self.reader.read_digit().parse::<usize>().unwrap())
        } else {
            None
        };
        let node = Node::LockVar {
            pos: ea.cmdpos,
            ea,
            depth,
            list: self
                .parse_lvaluelist()?
                .into_iter()
                .map(|n| Box::new(n))
                .collect::<Vec<Box<Node>>>(),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_lang(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut lines = vec![];
        self.reader.skip_white();
        if self.reader.peekn(2) == "<<" {
            self.reader.getn(2);
            self.reader.skip_white();
            let mut m = self.reader.get_line();
            if m == "" {
                m = ".".to_string();
            }
            self.reader.setpos(ea.linepos);
            lines.push(self.reader.get_line());
            self.reader.get();
            loop {
                if self.reader.peek() == "<EOF>" {
                    break;
                }
                lines.push(self.reader.get_line());
                if lines.last().unwrap() == &m {
                    break;
                }
                self.reader.get();
            }
        } else {
            self.reader.setpos(ea.linepos);
            lines.push(self.reader.get_line());
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            ea,
            value: lines.join("\n"),
        });
        Ok(())
    }

    fn parse_cmd_return(&mut self, ea: ExArg) -> Result<(), ParseError> {
        if !self.find_context(Node::is_function) {
            return Err(ParseError {
                msg: "E133: :return not inside a function".to_string(),
                pos: ea.cmdpos,
            });
        }
        self.reader.skip_white();
        let c = self.reader.peek();
        let left = if c == "\"" || !ends_excmds(&c) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.add_node(Node::Return {
            pos: ea.cmdpos,
            ea,
            left,
        });
        Ok(())
    }

    fn parse_cmd_syntax(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let mut end;
        loop {
            end = self.reader.getpos();
            let c = self.reader.peek();
            if c == "/" || c == "'" || c == "\"" {
                self.reader.get();
                self.parse_pattern(&c)?;
            } else if c == "=" {
                self.reader.get();
                self.parse_pattern(" ")?;
            } else if ends_excmds(&c) {
                break;
            }
            let peeked = self.reader.peek();
            if !["/", "'", "\"", "="].contains(&peeked.as_str()) {
                self.reader.getn(1);
            }
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            value: self.reader.getstr(ea.linepos, end),
            ea,
        });
        Ok(())
    }

    fn parse_cmd_throw(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let node = Node::Throw {
            pos: ea.cmdpos,
            ea,
            err: Box::new(self.parse_expr()?),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_try(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.push_context(Node::Try {
            pos: ea.cmdpos,
            ea,
            body: vec![],
            catches: vec![],
            finally: None,
            end: None,
        });
        Ok(())
    }

    fn parse_cmd_unlet(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let node = Node::Unlet {
            pos: ea.cmdpos,
            ea,
            list: self
                .parse_lvaluelist()?
                .into_iter()
                .map(|n| Box::new(n))
                .collect::<Vec<Box<Node>>>(),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_unlockvar(&mut self, ea: ExArg) -> Result<(), ParseError> {
        self.reader.skip_white();
        let depth = if isdigit(&self.reader.peek()) {
            Some(self.reader.read_digit().parse::<usize>().unwrap())
        } else {
            None
        };
        let node = Node::UnlockVar {
            pos: ea.cmdpos,
            ea,
            depth,
            list: self
                .parse_exprlist()?
                .into_iter()
                .map(|n| Box::new(n))
                .collect::<Vec<Box<Node>>>(),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_while(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let node = Node::While {
            pos: ea.cmdpos,
            ea,
            body: vec![],
            cond: Box::new(self.parse_expr()?),
            end: None,
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_wincmd(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let c = self.reader.getn(1);
        if c == "" {
            return self.err("E471: Argument required");
        } else if c == "g" || c == "\x07" {
            let c2 = self.reader.getn(1);
            if c2 == "" || iswhite(&c2) {
                return self.err("E474: Invalid argument");
            }
        }
        let end = self.reader.getpos();
        self.reader.skip_white();
        if !ends_excmds(&self.reader.peek()) {
            return self.err("E474: Invalid argument");
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            value: self.reader.getstr(ea.linepos, end),
            ea,
        });
        Ok(())
    }

    fn parse_letlhs(
        &mut self,
    ) -> Result<(Option<Box<Node>>, Vec<Box<Node>>, Option<Box<Node>>), ParseError> {
        let mut tokenizer = Tokenizer::new(self.reader);
        let mut nodes = vec![];
        let mut left = None;
        let mut rest = None;
        if tokenizer.peek()?.kind == TokenKind::SqOpen {
            tokenizer.get()?;
            loop {
                nodes.push(Box::new(self.parse_lvalue()?));
                let mut token = tokenizer.get()?;
                match token.kind {
                    TokenKind::SqClose => {
                        break;
                    }
                    TokenKind::Comma => {
                        continue;
                    }
                    TokenKind::Semicolon => {
                        rest = Some(Box::new(self.parse_lvalue()?));
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
            left = Some(Box::new(self.parse_lvalue()?));
        }
        Ok((left, nodes, rest))
    }

    fn parse_cmd_function(&mut self, ea: ExArg) -> Result<(), ParseError> {
        let pos = self.reader.tell();
        self.reader.skip_white();
        if ends_excmds(&self.reader.peek()) || self.reader.peek() == "/" {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let left = self.parse_lvalue_func()?;
        self.reader.skip_white();
        if let &Node::Identifier { pos, ref value, .. } = &left {
            if !value.starts_with("<")
                && !value.starts_with(|c: char| c.is_uppercase())
                && !value.contains(":")
                && !value.contains("#")
            {
                return Err(ParseError {
                    msg: format!(
                        "E128: Function name must start with a capital or contain a colon: {}",
                        value
                    ),
                    pos,
                });
            }
        }
        if self.reader.peek() != "(" {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let name = Box::new(left);
        self.reader.getn(1);
        let mut tokenizer = Tokenizer::new(self.reader);
        let mut args = vec![];
        if tokenizer.peek()?.kind == TokenKind::PClose {
            tokenizer.get()?;
        } else {
            let mut named: Vec<String> = vec![];
            loop {
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
                    args.push(Box::new(Node::Identifier {
                        pos: token.pos,
                        value: token.value,
                    }));
                    if iswhite(&self.reader.peek()) && tokenizer.peek()?.kind == TokenKind::Comma {
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
                    args.push(Box::new(Node::Identifier {
                        pos: token.pos,
                        value: token.value,
                    }));
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
        let mut attrs = vec![];
        loop {
            self.reader.skip_white();
            let epos = self.reader.getpos();
            let key = self.reader.read_alpha();
            match key.as_str() {
                "" => {
                    break;
                }
                "range" | "abort" | "dict" | "closure" => attrs.push(key),
                _ => {
                    return Err(ParseError {
                        msg: format!("unexpected token: {}", key),
                        pos: epos,
                    });
                }
            }
        }
        let node = Node::Function {
            pos: ea.cmdpos,
            ea,
            name,
            args,
            body: vec![],
            attrs,
            end: None,
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_exprlist(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes = vec![];
        loop {
            self.reader.skip_white();
            let c = self.reader.peek();
            if c != "\"" && ends_excmds(&c) {
                break;
            }
            let node = self.parse_expr()?;
            nodes.push(node);
        }
        Ok(nodes)
    }

    fn parse_lvalue(&mut self) -> Result<Node, ParseError> {
        let mut parser = ExprParser::new(self.reader);
        let node = parser.parse_lv()?;
        match node {
            Node::Identifier { pos, ref value, .. } => {
                if !isvarname(value) {
                    Err(ParseError {
                        msg: format!("E461: Illegal variable name: {}", value),
                        pos,
                    })
                } else {
                    Ok(node.clone())
                }
            }
            Node::CurlyName { .. }
            | Node::Subscript { .. }
            | Node::Slice { .. }
            | Node::Dot { .. }
            | Node::Option { .. }
            | Node::Env { .. }
            | Node::Reg { .. } => Ok(node),
            _ => Err(ParseError {
                msg: "Invalid expression".to_string(),
                pos: self.reader.getpos(),
            }),
        }
    }

    fn parse_lvaluelist(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes = vec![];
        nodes.push(self.parse_expr()?);
        loop {
            self.reader.skip_white();
            if ends_excmds(&self.reader.peek()) {
                break;
            }
            nodes.push(self.parse_lvalue()?);
        }
        Ok(nodes)
    }

    fn parse_lvalue_func(&mut self) -> Result<Node, ParseError> {
        let mut parser = ExprParser::new(self.reader);
        let node = parser.parse_lv()?;
        match node {
            Node::Identifier { .. }
            | Node::CurlyName { .. }
            | Node::Subscript { .. }
            | Node::Dot { .. }
            | Node::Option { .. }
            | Node::Env { .. }
            | Node::Reg { .. } => Ok(node),
            _ => Err(ParseError {
                msg: "Invalid expression".to_string(),
                pos: self.reader.getpos(),
            }),
        }
    }

    fn separate_nextcmd(&mut self, ea: &ExArg) -> Result<Position, ParseError> {
        if ["vimgrep", "vimgrepadd", "lvimgrep", "lvimgrepadd"].contains(&ea.cmd.name.as_str()) {
            self.skip_vimgrep_pat()?;
        }
        let mut pc = String::new();
        let mut end = self.reader.getpos();
        let mut nospend = end;
        loop {
            end = self.reader.getpos();
            if !iswhite(&pc) {
                nospend = end;
            }
            let mut c = self.reader.peek();
            if c == "\n" || c == "<EOF>" {
                break;
            } else if c == "\x16" {
                self.reader.get();
                end = self.reader.getpos();
                nospend = end;
                c = self.reader.peek();
                if c == "\n" || c == "<EOF>" {
                    break;
                }
                self.reader.get();
            } else if self.reader.peekn(2) == "`="
                && (ea.cmd.flags.contains(&Flag::Xfile)
                    || ea.cmd.flags.contains(&Flag::Files)
                    || ea.cmd.flags.contains(&Flag::File1))
            {
                self.reader.getn(2);
                self.parse_expr()?;
                c = self.reader.peekn(1);
                if c != "`" {
                    return self.err(&format!("unexpected character: {}", c));
                }
                self.reader.getn(1);
            } else if ["|", "\n", "\""].contains(&c.as_str())
                && !ea.cmd.flags.contains(&Flag::NoTrlCom)
                && (ea.cmd.name != "@" && ea.cmd.name != "*" || self.reader.getpos() != ea.argpos)
                && (ea.cmd.name != "redir"
                    || self.reader.getpos().cursor != ea.argpos.cursor + 1
                    || pc != "@")
            {
                if !ea.cmd.flags.contains(&Flag::UseCtrlV) && pc == "\\" {
                    self.reader.get();
                } else {
                    break;
                }
            } else {
                self.reader.get();
            }
            pc = c
        }
        if !ea.cmd.flags.contains(&Flag::NoTrlCom) {
            end = nospend;
        }
        Ok(end)
    }

    fn skip_vimgrep_pat(&mut self) -> Result<(), ParseError> {
        let c = self.reader.peek();
        if c == "\n" {
        } else if iswordc(&c) {
            self.reader.read_nonwhite();
        } else {
            let c = self.reader.get();
            let (_, endc) = self.parse_pattern(&c)?;
            if c != endc {
                return Ok(());
            }
            while self.reader.peek() == "g" || self.reader.peek() == "j" {
                self.reader.get();
            }
        }
        Ok(())
    }

    fn parse_argcmd(&mut self) {
        if self.reader.peek() == "+" {
            self.reader.get();
            if self.reader.peek() != " " {
                self.read_cmdarg();
            }
        }
    }

    fn read_cmdarg(&mut self) {
        loop {
            let c = self.reader.peekn(1);
            if c == "" || c.chars().collect::<Vec<char>>()[0].is_whitespace() {
                break;
            }
            self.reader.get();
            if c == "\\" {
                self.reader.get();
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
        while self.reader.peekn(2) == "++" {
            let s = self.reader.peekn(20);
            if BIN_RE.is_match(&s) {
                self.reader.getn(5);
            } else if NOBIN_RE.is_match(&s) {
                self.reader.getn(7);
            } else if EDIT_RE.is_match(&s) {
                self.reader.getn(6);
            } else if FF_RE.is_match(&s) {
                self.reader.getn(5);
            } else if FILEFORMAT_RE.is_match(&s) {
                self.reader.getn(13);
            } else if ENC_RE.is_match(&s) {
                self.reader.getn(6);
                self.reader.read_nonwhite();
            } else if ENCODING_RE.is_match(&s) {
                self.reader.getn(11);
                self.reader.read_nonwhite();
            } else if BAD_OUTER_RE.is_match(&s) {
                self.reader.getn(6);
                if BAD_INNER_RE.is_match(&s) {
                    self.reader.getn(4);
                } else {
                    self.reader.get();
                }
            } else if s.starts_with("++") {
                return self.err("E474: Invalid Argument");
            } else {
                break;
            }
            self.reader.skip_white();
        }
        Ok(())
    }

    fn find_command(&mut self) -> Option<Rc<Command>> {
        let c = self.reader.peek();
        let mut name = "".to_string();
        lazy_static! {
            static ref SUB_RE: Regex = Regex::new("^s(c[^sr][^i][^p]|g|i[^mlg]|I|r[^e])").unwrap();
            static ref DEL_RE: Regex = Regex::new("^d(elete|elet|ele|el|e)[lp]$").unwrap();
        }
        if c == "k" {
            name.push_str(&self.reader.get());
        } else if c == "s" && SUB_RE.is_match(&self.reader.peekn(5)) {
            self.reader.get();
            name.push_str("substitute");
        } else if ["@", "*", "!", "=", ">", "<", "&", "~", "#"].contains(&c.as_str()) {
            name.push_str(&self.reader.get());
        } else if self.reader.peekn(2) == "py" {
            name.push_str(&self.reader.read_alnum());
        } else {
            let pos = self.reader.tell();
            name.push_str(&self.reader.read_alpha());
            if name != "del" && DEL_RE.is_match(&name) {
                self.reader.seek_set(pos);
                name = self.reader.getn(name.len() - 1);
            }
        }
        if name == "" {
            return None;
        }
        if self.commands.contains_key(&name) {
            Some(Rc::clone(self.commands.get(&name).unwrap()))
        } else if name.starts_with(|c: char| c.is_uppercase()) {
            name.push_str(&self.reader.read_alnum());
            let cmd = Rc::new(Command {
                name: name.clone(),
                minlen: 0,
                flags: vec![Flag::UserCmd],
                parser: ParserKind::UserCmd,
            });
            self.commands.insert(name, Rc::clone(&cmd));
            Some(cmd)
        } else {
            None
        }
    }

    fn parse_cmd_modifier_range(&mut self, ea: ExArg) {
        let pos = self.reader.getpos();
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            value: self.reader.getstr(ea.linepos, pos),
            ea,
        });
    }

    fn parse_trail(&mut self) -> Result<(), ParseError> {
        self.reader.skip_white();
        let c = self.reader.peek();
        match c.as_str() {
            "<EOF>" => Ok(()),
            "\n" | "|" => {
                self.reader.get();
                Ok(())
            }
            "\"" => {
                self.parse_comment()?;
                self.reader.get();
                Ok(())
            }
            _ => self.err(&format!("E488: Trailing characters: {}", c)),
        }
    }
}

#[derive(Debug)]
pub struct ExprParser<'a> {
    reader: &'a Reader,
    tokenizer: Tokenizer<'a>,
}

impl<'a> ExprParser<'a> {
    pub fn new(reader: &'a Reader) -> ExprParser {
        ExprParser {
            reader,
            tokenizer: Tokenizer::new(reader),
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
        let pos = self.reader.tell();
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
            self.reader.seek_set(pos);
        }
        Ok(left)
    }

    fn parse_expr2(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr3()?;
        loop {
            let pos = self.reader.tell();
            let token = self.tokenizer.get()?;
            if token.kind == TokenKind::OrOr {
                let node = Node::Or {
                    pos: token.pos,
                    left: Box::new(left),
                    right: Box::new(self.parse_expr3()?),
                };
                left = node;
            } else {
                self.reader.seek_set(pos);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr3(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr4()?;
        loop {
            let pos = self.reader.tell();
            let token = self.tokenizer.get()?;
            if token.kind == TokenKind::AndAnd {
                let node = Node::And {
                    pos: token.pos,
                    left: Box::new(left),
                    right: Box::new(self.parse_expr4()?),
                };
                left = node;
            } else {
                self.reader.seek_set(pos);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr4(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr5()?;
        let cursor = self.reader.tell();
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
                self.reader.seek_set(cursor);
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
            let cursor = self.reader.tell();
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
                    self.reader.seek_set(cursor);
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
            let cursor = self.reader.tell();
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
                    self.reader.seek_set(cursor);
                    break;
                }
            };
            left = node;
        }
        Ok(left)
    }

    fn parse_expr7(&mut self) -> Result<Node, ParseError> {
        let cursor = self.reader.tell();
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
                self.reader.seek_set(cursor);
                return self.parse_expr8();
            }
        };
        Ok(node)
    }

    fn parse_expr8(&mut self) -> Result<Node, ParseError> {
        let mut left = self.parse_expr9()?;
        loop {
            let cursor = self.reader.tell();
            let c = self.reader.peek();
            let token = self.tokenizer.get()?;
            if !iswhite(&c) && token.kind == TokenKind::SqOpen {
                left = self.parse_slice(left, token.pos)?;
            } else if token.kind == TokenKind::POpen {
                let pos = token.pos;
                let name = Box::new(left);
                let mut args = vec![];
                if self.tokenizer.peek()?.kind == TokenKind::PClose {
                    self.tokenizer.get()?;
                } else {
                    loop {
                        args.push(Box::new(self.parse_expr1()?));
                        let token = self.tokenizer.get()?;
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
                        self.reader.seek_set(cursor);
                        break;
                    }
                }
            } else {
                self.reader.seek_set(cursor);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr9(&mut self) -> Result<Node, ParseError> {
        let cursor = self.reader.tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::Number => Node::Number {
                pos,
                value: token.value,
            },
            TokenKind::DQuote => {
                self.reader.seek_set(cursor);
                Node::String {
                    pos,
                    value: format!("\"{}\"", self.tokenizer.get_dstring()?),
                }
            }
            TokenKind::SQuote => {
                self.reader.seek_set(cursor);
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
                let savepos = self.reader.tell();
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
                                if iswhite(&self.reader.peek()) && maybe_comma == TokenKind::Comma {
                                    return Err(ParseError {
                                        msg: String::from(
                                            "E475: invalid argument: White space is not allowed before comma"
                                        ),
                                        pos: self.reader.getpos()
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
                self.reader.seek_set(savepos);
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
                        self.reader.seek_set(cursor);
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
                && self.reader.peekn(4).eq_ignore_ascii_case("SID>") =>
            {
                self.reader.seek_set(cursor);
                self.parse_identifier()?
            }
            TokenKind::Identifier
            | TokenKind::Is
            | TokenKind::IsCS
            | TokenKind::IsNot
            | TokenKind::IsNotCS => {
                self.reader.seek_set(cursor);
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
        self.reader.skip_white();
        let pos = self.reader.getpos();
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
        let c = self.reader.peek();
        let pos = self.reader.getpos();
        if c == "<" && self.reader.peekn(5).eq_ignore_ascii_case("<SID>") {
            let name = self.reader.getn(5);
            curly_parts.push(Node::CurlyNamePart { pos, value: name });
        }
        loop {
            let c = self.reader.peek();
            if isnamec(&c) {
                let pos = self.reader.getpos();
                let name = self.reader.read_name();
                curly_parts.push(Node::CurlyNamePart { pos, value: name });
            } else if c == "{" {
                self.reader.get();
                let pos = self.reader.getpos();
                curly_parts.push(Node::CurlyNameExpr {
                    pos,
                    expr: Box::new(self.parse_expr1()?),
                });
                self.reader.skip_white();
                let c = self.reader.peek();
                if c != "}" {
                    return Err(ParseError {
                        msg: format!("unexpected token: {}", c),
                        pos: self.reader.getpos(),
                    });
                }
                self.reader.seek_cur(1);
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
        if !iswordc(&self.reader.peek()) {
            return None;
        }
        let pos = self.reader.getpos();
        let name = self.reader.read_word();
        if isnamec(&self.reader.peek()) {
            return None;
        }
        let right = Box::new(Node::Identifier { pos, value: name });
        Some(Node::Dot {
            pos: token.pos,
            left: Box::new(left),
            right,
        })
    }

    fn parse_slice(&mut self, name: Node, pos: Position) -> Result<Node, ParseError> {
        let name = Box::new(name);
        if self.tokenizer.peek()?.kind == TokenKind::Colon {
            self.tokenizer.get()?;
            let left = None;
            let token = self.tokenizer.peek()?;
            let right = if token.kind != TokenKind::SqClose {
                Some(Box::new(self.parse_expr1()?))
            } else {
                None
            };
            let node = Node::Slice {
                pos,
                name,
                left,
                right,
            };
            let token = self.tokenizer.get()?;
            if token.kind != TokenKind::SqClose {
                return self.token_err(token);
            }
            Ok(node)
        } else {
            let expr = self.parse_expr1()?;
            if self.tokenizer.peek()?.kind == TokenKind::Colon {
                self.tokenizer.get()?;
                let left = Some(Box::new(expr));
                let token = self.tokenizer.peek()?;
                let right = if token.kind != TokenKind::SqClose {
                    Some(Box::new(self.parse_expr1()?))
                } else {
                    None
                };
                let node = Node::Slice {
                    pos,
                    name,
                    left,
                    right,
                };
                let token = self.tokenizer.get()?;
                if token.kind != TokenKind::SqClose {
                    return self.token_err(token);
                }
                Ok(node)
            } else {
                let node = Node::Subscript {
                    pos,
                    name,
                    index: Box::new(expr),
                };
                let token = self.tokenizer.get()?;
                if token.kind != TokenKind::SqClose {
                    return self.token_err(token);
                }
                Ok(node)
            }
        }
    }

    pub fn parse_lv(&mut self) -> Result<Node, ParseError> {
        // this differs from parse_expr8() insofar as it will not parse function calls. this method
        // is used for parsing the lhs of a `for` or `let` command, e.g. `let foo = bar`. in this
        // case a function call is not valid, e.g. `let foo() = bar` is not valid syntax, so we
        // should not parse it.
        let mut left = self.parse_lv9()?;
        loop {
            let cursor = self.reader.tell();
            let c = self.reader.peek();
            let token = self.tokenizer.get()?;
            if !iswhite(&c) && token.kind == TokenKind::SqOpen {
                left = self.parse_slice(left, token.pos)?;
            } else if !iswhite(&c) && token.kind == TokenKind::Dot {
                match self.parse_dot(token, left.clone()) {
                    Some(n) => {
                        left = n;
                    }
                    None => {
                        self.reader.seek_set(cursor);
                        break;
                    }
                }
            } else {
                self.reader.seek_set(cursor);
                break;
            }
        }
        Ok(left)
    }

    fn parse_lv9(&mut self) -> Result<Node, ParseError> {
        let cursor = self.reader.tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::COpen | TokenKind::Identifier => {
                self.reader.seek_set(cursor);
                let mut node = self.parse_identifier()?;
                if let Node::Identifier { ref mut value, .. } = node {
                    *value = token.value;
                };
                node
            }
            _ if token.kind == TokenKind::LT
                && self.reader.peekn(4).eq_ignore_ascii_case("SID>") =>
            {
                self.reader.seek_set(cursor);
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

#[cfg(test)]
mod tests {
    use super::super::parse_lines;

    #[test]
    fn test_parser() {
        let code = r#"#! /pointless/shebang
function! z#preview(text) abort
    if &previewwindow
        return
    endif
    let l:win = win_getid()
    let l:winview = winsaveview()
    pclose!
    execute 'topleft' &previewheight 'new'
    set previewwindow noswapfile nobuflisted buftype=nofile
    nnoremap <silent> <buffer> q :pclose!<CR>
    nnoremap <silent> <buffer> <C-c> :pclose!<CR>
    let l:text = type(a:text) == v:t_list ? a:text : split(a:text, '\n')
    call append(0, l:text)
    call cursor(1, 1)
    call win_gotoid(l:win)
    call winrestview(l:winview)
endfunction

delfunction z#preview

" this is a comment
function! z#enumerate(l, ...) abort
    let start = a:0 ? a:1 : 0
    let collection = type(a:l) == v:t_string ? split(a:l, '\zs') : a:l
    unlet start
    return map(collection, {i, v -> [(i + start), v]})
endfunction

function! z#zip(a, b) abort
    let collection = len(a:a) > len(a:b) ? a:a[:len(a:b)-1] : a:a
    return map(collection, {i, v -> [v, a:b[i]]})
endfunction

function! z#flatten(list) abort
    let rv = []
    let d = {'foo': 'bar'}
    let d.baz = d.foo / 7
    let s = 'quux' . 'garply' * 12
    for item in a:list
        let rv += type(item) == v:t_list ? z#flatten(item) : [item]
    endfor
    return rv
endfunction

function! s:while_loop() dict
    " dummy function to add some more node types
    let i = 0
    while i < 10
        lockvar 3 some_{i}_variable
        if i % 3 == 0
            continue
        elseif i && 9
            unlockvar some_other_thing
            break
        endif
    endwhile
endfunction

function! z#echohl(hl, msg) abort
    let l:msg = type(a:msg) == v:t_list ? a:msg : [a:msg]
    let l:echo = 'WarningMsg\|ErrorMsg' =~? a:hl ? 'echomsg' : 'echo'
    execute 'echohl' a:hl
    try
        for m in l:msg
            execute l:echo 'm'
        endfor
    catch
        echoerr 'error'
    finally
        echohl None
    endtry
endfunction

function! z#multisub(expr, pat, sub, ...)
    let flags = a:0 ? a:1 : ''
    let pat = type(a:pat) == v:t_list ? a:pat : [a:pat]
    if type(a:sub) == v:t_list || +1
        let sub = a:sub
        let minus_reg = -@x
        let not_env = !$ENV
        throw 'foobar'
    else
        let sub = []
        for _ in pat
            let sub += [a:sub]
        endfor
    endif
    let rv = a:expr
    for [search, replace] in z#zip(pat, sub)
        let rv = substitute(rv, search, replace, flags)
    endfor
    return rv
endfunction"#;

        // this output came from running vimlparser.py on the above code (plus manually adding the
        // shebang), so it is a reasonable test to assure the parser behaves like the original.
        let expected = r#"(#! " /pointless/shebang")
(function (z#preview text)
  (if &previewwindow
    (return))
  (let = l:win (win_getid))
  (let = l:winview (winsaveview))
  (excmd "pclose!")
  (execute 'topleft' &previewheight 'new')
  (excmd "set previewwindow noswapfile nobuflisted buftype=nofile")
  (excmd "nnoremap <silent> <buffer> q :pclose!<CR>")
  (excmd "nnoremap <silent> <buffer> <C-c> :pclose!<CR>")
  (let = l:text (?: (== (type a:text) v:t_list) a:text (split a:text '\n')))
  (call (append 0 l:text))
  (call (cursor 1 1))
  (call (win_gotoid l:win))
  (call (winrestview l:winview)))
(delfunction z#preview)
; this is a comment
(function (z#enumerate l . ...)
  (let = start (?: a:0 a:1 0))
  (let = collection (?: (== (type a:l) v:t_string) (split a:l '\zs') a:l))
  (unlet start)
  (return (map collection (lambda (i v) (list (+ i start) v)))))
(function (z#zip a b)
  (let = collection (?: (> (len a:a) (len a:b)) (slice a:a nil (- (len a:b) 1)) a:a))
  (return (map collection (lambda (i v) (list v (subscript a:b i))))))
(function (z#flatten list)
  (let = rv (list))
  (let = d (dict ('foo' 'bar')))
  (let = (dot d baz) (/ (dot d foo) 7))
  (let = s (concat 'quux' (* 'garply' 12)))
  (for item a:list
    (let += rv (?: (== (type item) v:t_list) (z#flatten item) (list item))))
  (return rv))
(function (s:while_loop)
  ; dummy function to add some more node types
  (let = i 0)
  (while (< i 10)
    (lockvar 3 some_{i}_variable)
    (if (== (% i 3) 0)
      (continue)
     elseif (&& i 9)
      (unlockvar some_other_thing)
      (break))))
(function (z#echohl hl msg)
  (let = l:msg (?: (== (type a:msg) v:t_list) a:msg (list a:msg)))
  (let = l:echo (?: (=~? 'WarningMsg\|ErrorMsg' a:hl) 'echomsg' 'echo'))
  (execute 'echohl' a:hl)
  (try
    (for m l:msg
      (execute l:echo 'm'))
   catch
    (echoerr 'error')
   finally
    (echohl "None")))
(function (z#multisub expr pat sub . ...)
  (let = flags (?: a:0 a:1 ''))
  (let = pat (?: (== (type a:pat) v:t_list) a:pat (list a:pat)))
  (if (|| (== (type a:sub) v:t_list) (+ 1))
    (let = sub a:sub)
    (let = minus_reg (- @x))
    (let = not_env (! $ENV))
    (throw 'foobar')
   else
    (let = sub (list))
    (for _ pat
      (let += sub (list a:sub))))
  (let = rv a:expr)
  (for (search replace) (z#zip pat sub)
    (let = rv (substitute rv search replace flags)))
  (return rv))"#;

        assert_eq!(
            format!(
                "{}",
                parse_lines(&code.split("\n").collect::<Vec<&str>>(), true).unwrap()
            ),
            expected
        );
    }
}
