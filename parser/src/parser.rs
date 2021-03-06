use super::{isargname, isvarname, CharClassification, ParseError, Position, EOF, EOL};
use crate::{
    command::{commands, valid_autocmds, Command, Flag, ParserKind},
    exarg::ExArg,
    modifier::Modifier,
    node::{BinaryOpKind, Node, UnaryOpKind},
    reader::Reader,
    token::{Token, TokenKind, Tokenizer},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, rc::Rc};

const MAX_FUNC_ARGS: usize = 20;

fn ends_excmds(c: char) -> bool {
    ['|', '"', EOF, EOL].contains(&c)
}

pub(crate) type Result<T> = std::result::Result<T, ParseError>;

fn parse_piped_expressions(s: &str) -> Result<Vec<Node>> {
    let reader = Reader::from_lines(&[s]);
    let mut parser = Parser::new(&reader);
    if let Node::TopLevel { body, .. } = parser.parse()? {
        Ok(body)
    } else {
        Err(ParseError {
            msg: "unknown sub-parser error: node returned was not a TopLevel node".to_string(),
            pos: Position::empty(),
        })
    }
}

fn make_modifier(k: &str) -> Option<Modifier> {
    lazy_static! {
        static ref MODIFIERS: &'static [(&'static str, usize)] = &[
            ("aboveleft", 3),
            ("belowright", 3),
            ("browse", 3),
            ("botright", 2),
            ("confirm", 4),
            ("keepmarks", 3),
            ("keepalt", 5),
            ("keepjumps", 5),
            ("keeppatterns", 5),
            ("hide", 3),
            ("lockmarks", 3),
            ("leftabove", 5),
            ("noautocmd", 3),
            ("noswapfile", 3),
            ("rightbelow", 6),
            ("sandbox", 3),
            ("silent", 3),
            ("tab", 3),
            ("topleft", 2),
            ("unsilent", 3),
            ("vertical", 4),
            ("verbose", 4),
        ];
    }
    for (modifier, min_length) in MODIFIERS.iter() {
        if modifier.starts_with(&k) && k.len() >= *min_length {
            return Some(Modifier::new(modifier));
        }
    }
    None
}

#[derive(Debug)]
pub struct Parser<'a> {
    reader: &'a Reader,
    context: Vec<Node>,
    commands: HashMap<String, Rc<Command>>,
}

impl<'a> Parser<'a> {
    pub fn new(reader: &'a Reader) -> Self {
        Self {
            reader,
            context: vec![],
            commands: commands(),
        }
    }

    fn ensure_context(&self) {
        if self.context.is_empty() {
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
                    catches.push(node);
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
                    elseifs.push(node);
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
                body.push(node);
            }
            _ => (),
        };
    }

    fn check_missing_endfunction(&self, end: &str, pos: Position) -> Result<()> {
        if let Node::Function { .. } = self.current_context() {
            Err(ParseError {
                msg: format!("E126: Missing :endfunction:    {}", end),
                pos,
            })
        } else {
            Ok(())
        }
    }

    fn check_missing_endif(&self, end: &str, pos: Position) -> Result<()> {
        match self.current_context() {
            Node::If { .. } | Node::ElseIf { .. } | Node::Else { .. } => Err(ParseError {
                msg: format!("E126: Missing :endif:    {}", end),
                pos,
            }),
            _ => Ok(()),
        }
    }

    fn check_missing_endtry(&self, end: &str, pos: Position) -> Result<()> {
        match self.current_context() {
            Node::Try { .. } | Node::Catch { .. } | Node::Finally { .. } => Err(ParseError {
                msg: format!("E126: Missing :endtry:    {}", end),
                pos,
            }),
            _ => Ok(()),
        }
    }

    fn check_missing_endwhile(&self, end: &str, pos: Position) -> Result<()> {
        if let Node::While { .. } = self.current_context() {
            Err(ParseError {
                msg: format!("E126: Missing :endwhile:    {}", end),
                pos,
            })
        } else {
            Ok(())
        }
    }

    fn check_missing_endfor(&self, end: &str, pos: Position) -> Result<()> {
        if let Node::For { .. } = self.current_context() {
            Err(ParseError {
                msg: format!("E126: Missing :endfor:    {}", end),
                pos,
            })
        } else {
            Ok(())
        }
    }

    fn err<T>(&self, msg: &str) -> Result<T> {
        Err(ParseError {
            msg: msg.to_string(),
            pos: self.reader.getpos(),
        })
    }

    pub fn parse(&mut self) -> Result<Node> {
        let pos = self.reader.getpos();
        self.push_context(Node::TopLevel { pos, body: vec![] });
        while self.reader.peek() != EOF {
            self.parse_one_cmd()?;
        }
        self.check_missing_endfunction("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endif("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endtry("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endwhile("TOPLEVEL", self.reader.getpos())?;
        self.check_missing_endfor("TOPLEVEL", self.reader.getpos())?;
        Ok(self.pop_context())
    }

    fn parse_expr(&mut self) -> Result<Node> {
        ExprParser::new(self.reader).parse()
    }

    fn parse_one_cmd(&mut self) -> Result<()> {
        if self.reader.peekn(2) == "#!" {
            self.parse_shebang()?;
            return Ok(());
        }
        let pos = self.reader.getpos();
        self.reader.skip_white_and_colon();
        if self.reader.peek() == EOL {
            self.reader.get();
            self.add_node(Node::BlankLine { pos });
            return Ok(());
        }
        if self.reader.peek() == '"' {
            self.parse_comment(false)?;
            self.reader.get();
            return Ok(());
        }
        let ea = ExArg {
            linepos: self.reader.getpos(),
            modifiers: self.parse_command_modifiers()?,
            range: self.parse_range()?,
            ..Default::default()
        };
        self.parse_command(ea)?;
        self.parse_trail()?;
        Ok(())
    }

    fn parse_shebang(&mut self) -> Result<()> {
        let sb = self.reader.getn(2);
        if sb != "#!" {
            return self.err(&format!("unexpected characters: {}", sb));
        }
        let pos = self.reader.getpos();
        let value = self.reader.get_line();
        self.add_node(Node::Shebang { pos, value });
        Ok(())
    }

    fn parse_comment(&mut self, trailing: bool) -> Result<()> {
        let pos = self.reader.getpos();
        let c = self.reader.get();
        if c != '"' {
            return Err(ParseError {
                msg: format!("unexpected character: {}", c),
                pos,
            });
        }
        self.add_node(Node::Comment {
            pos,
            value: self.reader.get_line(),
            trailing,
        });
        Ok(())
    }

    fn parse_command_modifiers(&mut self) -> Result<Vec<Modifier>> {
        let mut modifiers: Vec<Modifier> = vec![];
        loop {
            let pos = self.reader.tell();
            let mut count = "".to_string();
            if self.reader.peek().is_ascii_digit() {
                count = self.reader.read_digit();
                self.reader.skip_white();
            }
            let k = self.reader.read_alpha();
            let c = self.reader.peek();
            self.reader.skip_white();
            if let Some(mut modifier) = make_modifier(&k) {
                match modifier.name.as_str() {
                    "hide" => {
                        if ends_excmds(c) {
                            break;
                        }
                    }
                    "silent" => {
                        if c == '!' {
                            modifier.bang = true;
                            self.reader.get();
                        }
                    }
                    "tab" | "verbose" => {
                        if let Ok(n) = count.parse::<usize>() {
                            modifier.count = Some(n);
                        }
                    }
                    _ => (),
                }
                modifiers.push(modifier);
            } else {
                self.reader.seek_set(pos);
                break;
            }
        }
        Ok(modifiers)
    }

    fn parse_range(&mut self) -> Result<Vec<String>> {
        let mut tokens: Vec<String> = vec![];
        loop {
            loop {
                self.reader.skip_white();
                let c = self.reader.peek();
                match c {
                    '.' | '$' => tokens.push(self.reader.get().to_string()),
                    '\'' => {
                        if self.reader.peek_ahead(1) == EOL {
                            break;
                        }
                        tokens.push(self.reader.getn(2));
                    }
                    '/' | '?' => {
                        self.reader.get();
                        let (pattern, _) = self.parse_pattern(&c.to_string())?;
                        tokens.push(pattern);
                    }
                    '\\' => {
                        let m = self.reader.peek_ahead(1);
                        if m == '&' || m == '?' || m == '/' {
                            tokens.push(self.reader.getn(2));
                        } else {
                            return self.err("E10: \\\\ should be followed by /, ? or &");
                        }
                    }
                    _ if c.is_ascii_digit() => {
                        tokens.push(self.reader.read_digit());
                    }
                    _ => (),
                }
                loop {
                    self.reader.skip_white();
                    if self.reader.peek() == EOL {
                        break;
                    }
                    let n = self.reader.read_integer();
                    if n == "" {
                        break;
                    }
                    tokens.push(n);
                }
                if self.reader.peek() != '/' && self.reader.peek() != '?' {
                    break;
                }
            }
            let p = self.reader.peek();
            if p == '%' || p == '*' {
                tokens.push(self.reader.get().to_string());
            }
            let p = self.reader.peek();
            if p == ';' || p == ',' {
                tokens.push(self.reader.get().to_string());
                continue;
            }
            break;
        }
        Ok(tokens)
    }

    fn parse_pattern(&mut self, delimiter: &str) -> Result<(String, String)> {
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
                if c == EOL {
                    return self.err("E682: Invalid search pattern or delimiter");
                }
                self.reader.getn(1);
                pattern.push(c);
            } else if c == "[" {
                in_bracket += 1;
            } else if c == "]" {
                in_bracket -= 1;
            }
        }
        Ok((pattern, endc))
    }

    fn parse_command(&mut self, mut ea: ExArg) -> Result<()> {
        self.reader.skip_white_and_colon();
        ea.cmdpos = self.reader.getpos();
        if [EOL, '"', EOF].contains(&self.reader.peek()) {
            if !ea.modifiers.is_empty() || !ea.range.is_empty() {
                self.parse_cmd_modifier_range(ea);
            }
            return Ok(());
        }
        if let Some(c) = self.find_command() {
            ea.cmd = c;
        } else {
            return self.err(&format!(
                "E492: Not an editor command: {}",
                self.reader.peek_line()
            ));
        }
        if self.reader.peek() == '!'
            && !["substitute", "smagic", "snomagic"].contains(&ea.cmd.name.as_str())
        {
            self.reader.get();
            ea.bang = true;
        }
        if !ea.cmd.flags.contains(Flag::BANG) && ea.bang && !ea.cmd.flags.contains(Flag::USERCMD) {
            return Err(ParseError {
                msg: "E477: No ! allowed".to_string(),
                pos: ea.cmdpos,
            });
        }
        if ea.cmd.name != "!" {
            self.reader.skip_white();
        }
        ea.argpos = self.reader.getpos();
        if ea.cmd.flags.contains(Flag::ARGOPT) {
            self.parse_argopt()?;
        }
        if ea.cmd.name == "write" || ea.cmd.name == "update" {
            if self.reader.peek() == '>' {
                if self.reader.peek_ahead(1) == '>' {
                    return self.err("E494: Use w or w>>");
                }
                self.reader.seek_cur(2);
                self.reader.skip_white();
            } else if self.reader.peek() == '!' && ea.cmd.name == "write" {
                self.reader.get();
                ea.use_filter = true;
            }
        }
        if ea.cmd.name == "read" {
            if ea.bang {
                ea.use_filter = true;
                ea.bang = false;
            } else if self.reader.peek() == '!' {
                self.reader.get();
                ea.use_filter = true;
            }
        }
        if ea.cmd.name == "<" || ea.cmd.name == ">" {
            while self.reader.peek().to_string() == ea.cmd.name {
                self.reader.get();
            }
            self.reader.skip_white();
        }
        if ea.cmd.flags.contains(Flag::EDITCMD) && !ea.use_filter {
            self.parse_argcmd();
        }
        self._parse_command(ea)
    }

    fn _parse_command(&mut self, ea: ExArg) -> Result<()> {
        match ea.cmd.parser {
            ParserKind::Append | ParserKind::Insert => {
                self.parse_cmd_append(ea);
                Ok(())
            }
            ParserKind::Autocmd => self.parse_cmd_autocmd(ea),
            ParserKind::Break => self.parse_cmd_break(ea),
            ParserKind::Call => self.parse_cmd_call(ea),
            ParserKind::Catch => self.parse_cmd_catch(ea),
            ParserKind::Common | ParserKind::UserCmd => self.parse_cmd_common(ea),
            ParserKind::Continue => self.parse_cmd_continue(ea),
            ParserKind::Echo => self.parse_cmd_echo(ea),
            ParserKind::Else => self.parse_cmd_else(ea),
            ParserKind::ElseIf => self.parse_cmd_elseif(ea),
            ParserKind::EndFor => self.parse_cmd_endfor(ea),
            ParserKind::EndFunction => self.parse_cmd_endfunction(ea),
            ParserKind::EndIf => self.parse_cmd_endif(ea),
            ParserKind::EndTry => self.parse_cmd_endtry(ea),
            ParserKind::EndWhile => self.parse_cmd_endwhile(ea),
            ParserKind::Execute => self.parse_cmd_execute(ea),
            ParserKind::Finally => self.parse_cmd_finally(ea),
            ParserKind::Finish => self.parse_cmd_common(ea),
            ParserKind::For => self.parse_cmd_for(ea),
            ParserKind::Function => self.parse_cmd_function(ea),
            ParserKind::Highlight => self.parse_cmd_highlight(ea),
            ParserKind::If => self.parse_cmd_if(ea),
            ParserKind::Lang => self.parse_cmd_lang(ea),
            ParserKind::Let => self.parse_cmd_let(ea),
            ParserKind::LoadKeymap => self.parse_cmd_loadkeymap(ea),
            ParserKind::LockVar => self.parse_cmd_lockvar(ea),
            ParserKind::Mapping => self.parse_cmd_mapping(ea),
            ParserKind::Return => self.parse_cmd_return(ea),
            ParserKind::Syntax => self.parse_cmd_syntax(ea),
            ParserKind::Throw => self.parse_cmd_throw(ea),
            ParserKind::Try => self.parse_cmd_try(ea),
            ParserKind::Unlet => self.parse_cmd_unlet(ea),
            ParserKind::While => self.parse_cmd_while(ea),
            ParserKind::WinCmd => self.parse_cmd_wincmd(ea),
        }
    }

    fn parse_cmd_append(&mut self, ea: ExArg) {
        self.reader.setpos(ea.linepos);
        self.reader.get_line(); // throw away the command line, it will end with "append"
        self.reader.get();
        let mut lines = vec![];
        loop {
            if self.reader.peek() == EOF {
                break;
            }
            lines.push(self.reader.get_line());
            if lines.last().unwrap() == "." {
                break;
            }
            self.reader.get();
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            command: ea.cmd.name.clone(),
            bang: ea.bang,
            args: if !lines.is_empty() {
                format!("\n{}", lines.join("\n"))
            } else {
                String::new()
            },
        });
    }

    fn parse_cmd_autocmd(&mut self, ea: ExArg) -> Result<()> {
        // this is a mess because autocmd syntax is bonkers - almost everything is optional
        let pos = ea.cmdpos;
        self.reader.skip_white();
        if self.reader.peekn(1) == "" {
            self.add_node(Node::Autocmd {
                pos,
                mods: ea.modifiers,
                bang: ea.bang,
                group: String::new(),
                events: vec![],
                patterns: vec![],
                nested: false,
                body: vec![],
            });
            return Ok(());
        }
        let maybe_group = self.reader.read_nonwhite();
        let (events_str, group) = if maybe_group
            .split(',')
            .all(|word| !valid_autocmds().contains_key(&word.to_lowercase().as_str()))
        {
            // maybe_group contains no autocmd names so assume it's a group
            self.reader.skip_white();
            if self.reader.peekn(1) == "" {
                self.add_node(Node::Autocmd {
                    pos,
                    mods: ea.modifiers,
                    bang: ea.bang,
                    group: maybe_group,
                    events: vec![],
                    patterns: vec![],
                    nested: false,
                    body: vec![],
                });
                return Ok(());
            }
            (self.reader.read_nonwhite(), maybe_group)
        } else {
            // maybe_group contains at least one autocmd name so assume it's a list of events
            (maybe_group, String::new())
        };
        let mut events = vec![];
        for event in events_str.split(",") {
            match valid_autocmds().get(&event.to_lowercase().as_str()) {
                Some(e) => events.push(e.clone()),
                None => return self.err(&format!("E216: No such group or event: {}", event)),
            }
        }
        self.reader.skip_white();
        if self.reader.peekn(1) == "" {
            self.add_node(Node::Autocmd {
                pos,
                mods: ea.modifiers,
                bang: ea.bang,
                group,
                events,
                patterns: vec![],
                nested: false,
                body: vec![],
            });
            return Ok(());
        }
        let patterns = self
            .reader
            .read_nonwhite()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        self.reader.skip_white();
        if self.reader.peekn(1) == "" {
            self.add_node(Node::Autocmd {
                pos,
                mods: ea.modifiers,
                bang: ea.bang,
                group,
                events,
                patterns,
                nested: false,
                body: vec![],
            });
            return Ok(());
        }
        let nested = self.reader.peekn(6).to_lowercase() == "nested";
        if nested {
            self.reader.getn(6);
            self.reader.skip_white();
        }
        if self.reader.peekn(1) == "" {
            self.add_node(Node::Autocmd {
                pos,
                mods: ea.modifiers,
                bang: ea.bang,
                group,
                events,
                patterns,
                nested,
                body: vec![],
            });
            return Ok(());
        }
        let offset = self.reader.tell();
        let result = parse_piped_expressions(&self.reader.get_line());
        let body = match result {
            Ok(body) => body,
            Err(e) => {
                self.reader.seek_set(e.pos.cursor + offset);
                return Err(ParseError {
                    msg: e.msg,
                    pos: self.reader.getpos(),
                });
            }
        };
        self.add_node(Node::Autocmd {
            pos,
            mods: ea.modifiers,
            bang: ea.bang,
            group,
            events,
            patterns,
            nested,
            body,
        });
        Ok(())
    }

    fn parse_cmd_break(&mut self, ea: ExArg) -> Result<()> {
        if !self.find_context(Node::is_while) && !self.find_context(Node::is_for) {
            return self.err("E587: :break without :while or :for");
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            bang: ea.bang,
            command: "break".to_string(),
            args: String::new(),
        });
        Ok(())
    }

    fn parse_cmd_call(&mut self, ea: ExArg) -> Result<()> {
        let pos = ea.cmdpos;
        self.reader.skip_white();
        if ends_excmds(self.reader.peek()) {
            return self.err("E471: Argument required");
        }
        let left = self.parse_expr()?;
        match left {
            Node::Call { .. } => {
                self.add_node(Node::ExCall {
                    pos,
                    mods: ea.modifiers,
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

    fn parse_cmd_catch(&mut self, ea: ExArg) -> Result<()> {
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
        let pattern = if !ends_excmds(self.reader.peek()) {
            let (pat, _) = self.parse_pattern(&self.reader.get().to_string())?;
            Some(pat)
        } else {
            None
        };
        self.push_context(Node::Catch {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            pattern,
            body: vec![],
        });
        Ok(())
    }

    fn parse_cmd_common(&mut self, ea: ExArg) -> Result<()> {
        let mut end;
        if ea.cmd.flags.contains(Flag::TRLBAR) && !ea.use_filter {
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
            mods: ea.modifiers,
            command: ea.cmd.name.clone(),
            args: self.reader.getstr(ea.argpos, end),
            bang: ea.bang,
        });
        Ok(())
    }

    fn parse_cmd_continue(&mut self, ea: ExArg) -> Result<()> {
        if !self.find_context(Node::is_while) && !self.find_context(Node::is_for) {
            return Err(ParseError {
                msg: "E586: :continue without :while or :for".to_string(),
                pos: ea.cmdpos,
            });
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            bang: ea.bang,
            command: "continue".to_string(),
            args: String::new(),
        });
        Ok(())
    }

    fn parse_cmd_echo(&mut self, ea: ExArg) -> Result<()> {
        let node = Node::Echo {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            cmd: ea.cmd.name.clone(),
            list: self.parse_exprlist()?,
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_execute(&mut self, ea: ExArg) -> Result<()> {
        let node = Node::Execute {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            list: self.parse_exprlist()?,
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_else(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context() {
            Node::If { .. } => (),
            Node::ElseIf { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E581: :else without :if".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        self.push_context(Node::Else {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            body: vec![],
        });
        Ok(())
    }

    fn parse_cmd_elseif(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context() {
            Node::If { .. } => (),
            Node::ElseIf { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E582: :elseif without :if".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        let node = Node::ElseIf {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            cond: Box::new(self.parse_expr()?),
            body: vec![],
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_endfor(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context_mut() {
            Node::For { ref mut end, .. } => {
                let node = Node::End {
                    pos: ea.cmdpos,
                    mods: ea.modifiers,
                };
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

    fn parse_cmd_endfunction(&mut self, ea: ExArg) -> Result<()> {
        self.check_missing_endif("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endtry("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endwhile("ENDFUNCTION", ea.cmdpos)?;
        self.check_missing_endfor("ENDFUNCTION", ea.cmdpos)?;
        match self.current_context_mut() {
            Node::Function { ref mut end, .. } => {
                let node = Node::End {
                    pos: ea.cmdpos,
                    mods: ea.modifiers,
                };
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

    fn parse_cmd_endif(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context() {
            Node::If { .. } => (),
            Node::ElseIf { .. } | Node::Else { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E580: :endif without :if".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        if let Node::If { ref mut end, .. } = self.current_context_mut() {
            let node = Node::End {
                pos: ea.cmdpos,
                mods: ea.modifiers,
            };
            *end = Some(Box::new(node));
        }
        self.collapse_context();
        Ok(())
    }

    fn parse_cmd_endtry(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context() {
            Node::Try { .. } => (),
            Node::Catch { .. } | Node::Finally { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E580: :endtry without :try".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        if let Node::Try { ref mut end, .. } = self.current_context_mut() {
            let node = Node::End {
                pos: ea.cmdpos,
                mods: ea.modifiers,
            };
            *end = Some(Box::new(node));
        }
        self.collapse_context();
        Ok(())
    }

    fn parse_cmd_endwhile(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context() {
            Node::While { .. } => {
                let node = Node::End {
                    pos: ea.cmdpos,
                    mods: ea.modifiers,
                };
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

    fn parse_cmd_finally(&mut self, ea: ExArg) -> Result<()> {
        match self.current_context() {
            Node::Try { .. } => (),
            Node::Catch { .. } => {
                self.collapse_context();
            }
            _ => {
                return Err(ParseError {
                    msg: "E606: :finally without :try".to_string(),
                    pos: ea.cmdpos,
                });
            }
        };
        self.push_context(Node::Finally {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            body: vec![],
        });
        Ok(())
    }

    fn parse_cmd_for(&mut self, ea: ExArg) -> Result<()> {
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
            mods: ea.modifiers,
            var: var.map(Box::new),
            list,
            rest: rest.map(Box::new),
            right,
            body: vec![],
            end: None,
        });
        Ok(())
    }

    fn parse_cmd_if(&mut self, ea: ExArg) -> Result<()> {
        let node = Node::If {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            cond: Box::new(self.parse_expr()?),
            elseifs: vec![],
            else_: None,
            body: vec![],
            end: None,
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_lang(&mut self, ea: ExArg) -> Result<()> {
        let mut lines = vec![];
        self.reader.skip_white();
        if self.reader.peekn(2) == "<<" {
            self.reader.getn(2);
            self.reader.skip_white();
            let mut m = self.reader.get_line();
            if m == "" {
                m = ".".to_string();
            }
            self.reader.setpos(ea.argpos);
            lines.push(self.reader.get_line());
            self.reader.get();
            loop {
                if self.reader.peek() == EOF {
                    break;
                }
                lines.push(self.reader.get_line());
                if lines.last().unwrap() == &m {
                    break;
                }
                self.reader.get();
            }
        } else {
            self.reader.setpos(ea.argpos);
            lines.push(self.reader.get_line());
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            bang: ea.bang,
            command: ea.cmd.name.clone(),
            args: lines.join("\n"),
        });
        Ok(())
    }

    fn parse_cmd_let(&mut self, ea: ExArg) -> Result<()> {
        let pos = self.reader.tell();
        self.reader.skip_white();
        if ends_excmds(self.reader.peek()) {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let (var, list, rest) = self.parse_letlhs()?;
        self.reader.skip_white();
        let s1 = self.reader.peek();
        let s2 = self.reader.peekn(2);
        if ends_excmds(s1) || s2 != "+=" && s2 != "-=" && s2 != ".=" && s1 != '=' {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let op = if s2 == "+=" || s2 == "-=" || s2 == ".=" {
            self.reader.getn(2);
            s2
        } else if s1 == '=' {
            self.reader.get();
            s1.to_string()
        } else {
            return self.err("NOT REACHED");
        };
        let node = Node::Let {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            var: var.map(Box::new),
            list,
            rest: rest.map(Box::new),
            op,
            right: Box::new(self.parse_expr()?),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_loadkeymap(&mut self, ea: ExArg) -> Result<()> {
        self.reader.setpos(ea.linepos);
        self.reader.get_line();
        let mut lines = vec![];
        loop {
            if self.reader.peek() == EOF {
                break;
            }
            lines.push(self.reader.get_line());
            self.reader.get();
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            bang: ea.bang,
            command: ea.cmd.name.clone(),
            args: if !lines.is_empty() {
                format!("\n{}", lines.join("\n"))
            } else {
                String::new()
            },
        });
        Ok(())
    }

    fn parse_cmd_lockvar(&mut self, ea: ExArg) -> Result<()> {
        self.reader.skip_white();
        let depth = if self.reader.peek().is_ascii_digit() {
            Some(self.reader.read_digit().parse::<usize>().unwrap())
        } else {
            None
        };
        let node = Node::LockVar {
            cmd: ea.cmd.name.to_string(),
            pos: ea.cmdpos,
            mods: ea.modifiers,
            bang: ea.bang,
            depth,
            list: self.parse_lvaluelist()?,
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_mapping(&mut self, ea: ExArg) -> Result<()> {
        let command = ea.cmd.name.clone();
        let mut attrs = vec![];
        let mut right_expr = None;
        loop {
            self.reader.skip_white();
            let pos = self.reader.getpos();
            if self.reader.peek() != '<' {
                break;
            } else {
                self.reader.get();
                let attr = self.reader.read_alpha();
                match attr.to_lowercase().as_str() {
                    "buffer" | "nowait" | "silent" | "script" | "unique" | "expr" => {
                        attrs.push(attr.to_lowercase());
                        if self.reader.peek() == '>' {
                            self.reader.get();
                        } else {
                            return self.err(&format!("unexpected token: {}", self.reader.peek()));
                        }
                    }
                    _ => {
                        // this is a special key in a mapping, e.g. `nnoremap <C-t> ...`,
                        // so reset position and continue parsing
                        self.reader.setpos(pos);
                        break;
                    }
                }
            }
        }
        let left = if !["|", "", "\n", &EOF.to_string()].contains(&self.reader.peekn(1).as_str()) {
            self.reader.read_nonwhite()
        } else {
            self.add_node(Node::Mapping {
                command,
                attrs,
                left: String::new(),
                right: String::new(),
                right_expr,
                pos: ea.cmdpos,
                mods: ea.modifiers,
            });
            return Ok(());
        };
        self.reader.skip_white();
        let right = if attrs.contains(&"expr".to_string()) {
            right_expr = Some(Box::new(self.parse_expr()?));
            String::new()
        } else {
            let mut right = String::new();
            loop {
                let c = self.reader.peek();
                let c2 = self.reader.peek_ahead(1);
                if c == '\\' && c2 == '|' {
                    self.reader.get();
                    right.push(self.reader.get());
                } else if c != '"' && ends_excmds(c) {
                    break;
                } else {
                    right.push(self.reader.get());
                }
            }
            right.trim_end().to_string()
        };
        self.add_node(Node::Mapping {
            command,
            attrs,
            left,
            right,
            right_expr,
            pos: ea.cmdpos,
            mods: ea.modifiers,
        });
        Ok(())
    }

    fn parse_cmd_return(&mut self, ea: ExArg) -> Result<()> {
        if !self.find_context(Node::is_function) {
            return Err(ParseError {
                msg: "E133: :return not inside a function".to_string(),
                pos: ea.cmdpos,
            });
        }
        self.reader.skip_white();
        let c = self.reader.peek();
        let left = if c == '"' || !ends_excmds(c) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.add_node(Node::Return {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            left,
        });
        Ok(())
    }

    fn parse_cmd_syntax(&mut self, ea: ExArg) -> Result<()> {
        let mut end;
        loop {
            end = self.reader.getpos();
            let c = self.reader.peek();
            if c == '/' || c == '\'' || c == '"' {
                self.reader.get();
                self.parse_pattern(&c.to_string())?;
            } else if c == '=' {
                self.reader.get();
                self.parse_pattern(" ")?;
            } else if ends_excmds(c) {
                break;
            }
            if !['/', '\'', '"', '='].contains(&self.reader.peek()) {
                self.reader.getn(1);
            }
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            command: ea.cmd.name.clone(),
            args: self.reader.getstr(ea.argpos, end),
            bang: ea.bang,
        });
        Ok(())
    }

    fn parse_cmd_throw(&mut self, ea: ExArg) -> Result<()> {
        let node = Node::Throw {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            err: Box::new(self.parse_expr()?),
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_try(&mut self, ea: ExArg) -> Result<()> {
        self.push_context(Node::Try {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            body: vec![],
            catches: vec![],
            finally: None,
            end: None,
        });
        Ok(())
    }

    fn parse_cmd_unlet(&mut self, ea: ExArg) -> Result<()> {
        let node = Node::Unlet {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            bang: ea.bang,
            list: self.parse_lvaluelist()?,
        };
        self.add_node(node);
        Ok(())
    }

    fn parse_cmd_while(&mut self, ea: ExArg) -> Result<()> {
        let node = Node::While {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            body: vec![],
            cond: Box::new(self.parse_expr()?),
            end: None,
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_wincmd(&mut self, ea: ExArg) -> Result<()> {
        let c = self.reader.getn(1);
        if c == "" {
            return self.err("E471: Argument required");
        } else if c == "g" || c == "\x07" {
            let c2 = self.reader.getn(1);
            if ["", " ", "\t"].contains(&c2.as_str()) {
                return self.err("E474: Invalid argument");
            }
        }
        let end = self.reader.getpos();
        self.reader.skip_white();
        if !ends_excmds(self.reader.peek()) {
            return self.err("E474: Invalid argument");
        }
        self.add_node(Node::ExCmd {
            pos: ea.cmdpos,
            mods: ea.modifiers,
            command: ea.cmd.name.clone(),
            args: self.reader.getstr(ea.argpos, end),
            bang: ea.bang,
        });
        Ok(())
    }

    fn parse_letlhs(&mut self) -> Result<(Option<Node>, Vec<Node>, Option<Node>)> {
        let mut tokenizer = Tokenizer::new(self.reader);
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

    fn parse_cmd_function(&mut self, ea: ExArg) -> Result<()> {
        let pos = self.reader.tell();
        self.reader.skip_white();
        if ends_excmds(self.reader.peek()) || self.reader.peek() == '/' {
            self.reader.seek_set(pos);
            return self.parse_cmd_common(ea);
        }
        let left = self.parse_lvalue_func()?;
        self.reader.skip_white();
        if let Node::Identifier { pos, ref value, .. } = left {
            if !value.starts_with('<')
                && !value.starts_with(|c: char| c.is_uppercase())
                && !value.contains(':')
                && !value.contains('#')
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
        if self.reader.peek() != '(' {
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
                    args.push(Node::Identifier {
                        pos: token.pos,
                        value: token.value,
                    });
                    if self.reader.peek().is_white() && tokenizer.peek()?.kind == TokenKind::Comma {
                        return self.err(
                            "E475: Invalid argument: White space is not allowed before comma",
                        );
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
                    args.push(Node::Identifier {
                        pos: token.pos,
                        value: token.value,
                    });
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
            mods: ea.modifiers,
            bang: ea.bang,
            name,
            args,
            body: vec![],
            attrs,
            end: None,
        };
        self.push_context(node);
        Ok(())
    }

    fn parse_cmd_highlight(&mut self, ea: ExArg) -> Result<()> {
        let (pos, mods, bang) = (ea.cmdpos, ea.modifiers, ea.bang);
        let mut attrs = vec![];
        let mut token = self.reader.read_nonwhitespace();
        if token == "" {
            self.add_node(Node::Highlight {
                pos,
                mods,
                bang,
                clear: false,
                default: false,
                link: false,
                group: None,
                none: false,
                to_group: None,
                attrs,
            });
            return Ok(());
        }
        if token.to_lowercase() == "clear" {
            self.reader.skip_white();
            token = self.reader.read_nonwhitespace();
            self.add_node(Node::Highlight {
                pos,
                mods,
                bang,
                clear: true,
                default: false,
                link: false,
                none: false,
                to_group: None,
                attrs,
                group: if token == "" { None } else { Some(token) },
            });
            return Ok(());
        }
        let default = token.to_lowercase() == "default";
        if default {
            self.reader.skip_white();
            token = self.reader.read_nonwhitespace();
            if token == "" {
                self.add_node(Node::Highlight {
                    pos,
                    mods,
                    bang,
                    clear: false,
                    default: true,
                    link: false,
                    none: false,
                    to_group: None,
                    attrs,
                    group: None,
                });
                return Ok(());
            }
        }
        let link = token.to_lowercase() == "link";
        if link {
            self.reader.skip_white();
            token = self.reader.read_nonwhitespace();
            if token == "" {
                return Err(ParseError {
                    msg: "E412: Not enough arguments: \":highlight link \"".to_string(),
                    pos,
                });
            }
        }
        let group = Some(token);
        self.reader.skip_white();
        token = self.reader.read_nonwhitespace();
        if token.to_lowercase() == "none" {
            self.add_node(Node::Highlight {
                pos,
                mods,
                bang,
                clear: false,
                default,
                link,
                none: true,
                to_group: None,
                attrs,
                group,
            });
            return Ok(());
        } else if link {
            return if token == "" {
                Err(ParseError {
                    msg: format!(
                        "E412: Not enough arguments: \":highlight link {}\"",
                        group.unwrap()
                    ),
                    pos,
                })
            } else {
                self.add_node(Node::Highlight {
                    pos,
                    mods,
                    bang,
                    clear: false,
                    default,
                    link,
                    none: false,
                    to_group: Some(token),
                    attrs,
                    group,
                });
                return Ok(());
            };
        }
        lazy_static! {
            static ref VALID_HL_KEYS: &'static [&'static str] = &[
                "cterm", "ctermbg", "ctermfg", "font", "gui", "guibg", "guifg", "guisp", "start",
                "stop", "term",
            ];
        }
        while token != "" {
            if !token.contains('=') {
                return self.err(&format!("E416: missing equal sign: {}", token));
            }
            if token.contains("='") {
                // have to account for e.g. `:highlight String font='Monospace 10'`
                loop {
                    let c = self.reader.get();
                    if c == EOL || c == EOF {
                        return self.err(&format!("E475: Invalid argument: {}", token));
                    }
                    token.push(c);
                    if c == '\'' {
                        break;
                    }
                }
            }
            let splits = token.splitn(2, '=').collect::<Vec<&str>>();
            let (key, value) = (splits[0], splits[1]);
            if !VALID_HL_KEYS.contains(&key.to_lowercase().as_str()) {
                return Err(ParseError {
                    msg: format!("E423: Illegal argument: {}", token),
                    pos,
                });
            }
            attrs.push((key.to_lowercase(), value.to_string()));
            self.reader.skip_white();
            token = self.reader.read_nonwhitespace();
        }
        self.add_node(Node::Highlight {
            pos,
            mods,
            bang,
            clear: false,
            default,
            link,
            none: false,
            to_group: None,
            attrs,
            group,
        });
        Ok(())
    }

    fn parse_exprlist(&mut self) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        loop {
            self.reader.skip_white();
            let c = self.reader.peek();
            if c != '"' && ends_excmds(c) {
                break;
            }
            let node = self.parse_expr()?;
            nodes.push(node);
        }
        Ok(nodes)
    }

    fn parse_lvalue(&mut self) -> Result<Node> {
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

    fn parse_lvaluelist(&mut self) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        nodes.push(self.parse_expr()?);
        loop {
            self.reader.skip_white();
            if ends_excmds(self.reader.peek()) {
                break;
            }
            nodes.push(self.parse_lvalue()?);
        }
        Ok(nodes)
    }

    fn parse_lvalue_func(&mut self) -> Result<Node> {
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

    fn separate_nextcmd(&mut self, ea: &ExArg) -> Result<Position> {
        if ["vimgrep", "vimgrepadd", "lvimgrep", "lvimgrepadd"].contains(&ea.cmd.name.as_str()) {
            self.skip_vimgrep_pat()?;
        }
        let mut pc: char = EOF;
        let mut end = self.reader.getpos();
        let mut nospend = end;
        loop {
            end = self.reader.getpos();
            if !pc.is_white() {
                nospend = end;
            }
            let mut c = self.reader.peek();
            if c == EOL || c == EOF {
                break;
            } else if c == '\x16' {
                self.reader.get();
                end = self.reader.getpos();
                nospend = end;
                c = self.reader.peek();
                if c == EOL || c == EOF {
                    break;
                }
                self.reader.get();
            } else if self.reader.peekn(2) == "`="
                && (ea
                    .cmd
                    .flags
                    .intersects(Flag::XFILE | Flag::FILES | Flag::FILE1))
            {
                self.reader.getn(2);
                self.parse_expr()?;
                let peeked = self.reader.peekn(1);
                if peeked != "`" {
                    return self.err(&format!("unexpected character: {}", c));
                }
                let gotten = self.reader.getn(1);
                c = if gotten.is_empty() {
                    EOF
                } else {
                    gotten.chars().nth(0).unwrap()
                };
            } else if ['|', EOL, '"'].contains(&c)
                && !ea.cmd.flags.contains(Flag::NOTRLCOM)
                && (ea.cmd.name != "@" && ea.cmd.name != "*" || self.reader.getpos() != ea.argpos)
                && (ea.cmd.name != "redir"
                    || self.reader.getpos().cursor != ea.argpos.cursor + 1
                    || pc != '@')
            {
                if !ea.cmd.flags.contains(Flag::USECTRLV) && pc == '\\' {
                    self.reader.get();
                } else {
                    break;
                }
            } else {
                self.reader.get();
            }
            pc = c
        }
        if !ea.cmd.flags.contains(Flag::NOTRLCOM) {
            end = nospend;
        }
        Ok(end)
    }

    fn skip_vimgrep_pat(&mut self) -> Result<()> {
        let c = self.reader.peek();
        if c == EOL {
        } else if c.is_word() {
            self.reader.read_nonwhite();
        } else {
            let c = self.reader.get();
            let (_, endc) = self.parse_pattern(&c.to_string())?;
            if c.to_string() != endc {
                return Ok(());
            }
            while self.reader.peek() == 'g' || self.reader.peek() == 'j' {
                self.reader.get();
            }
        }
        Ok(())
    }

    fn parse_argcmd(&mut self) {
        if self.reader.peek() == '+' {
            self.reader.get();
            if self.reader.peek() != ' ' {
                self.read_cmdarg();
            }
        }
    }

    fn read_cmdarg(&mut self) {
        loop {
            let c = self.reader.peekn(1);
            if c == "" || c.chars().collect::<Vec<char>>()[0].is_white() {
                break;
            }
            self.reader.get();
            if c == "\\" {
                self.reader.get();
            }
        }
    }

    fn parse_argopt(&mut self) -> Result<()> {
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
        if c == 'k' {
            name.push(self.reader.get());
        } else if c == 's' && SUB_RE.is_match(&self.reader.peekn(5)) {
            self.reader.get();
            name.push_str("substitute");
        } else if ['@', '*', '!', '=', '>', '<', '&', '~', '#'].contains(&c) {
            name.push(self.reader.get());
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
        if let Some(cmd) = self.commands.get(&name) {
            Some(Rc::clone(cmd))
        } else if name.starts_with(|c: char| c.is_uppercase()) {
            name.push_str(&self.reader.read_alnum());
            let cmd = Rc::new(Command {
                name: name.clone(),
                minlen: 0,
                flags: Flag::USERCMD | Flag::TRLBAR,
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
            mods: ea.modifiers,
            command: ea.cmd.name.clone(),
            args: self.reader.getstr(ea.argpos, pos),
            bang: ea.bang,
        });
    }

    fn parse_trail(&mut self) -> Result<()> {
        self.reader.skip_white();
        let c = self.reader.peek();
        match c {
            EOF => Ok(()),
            EOL | '|' => {
                self.reader.get();
                Ok(())
            }
            '"' => {
                self.parse_comment(true)?;
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
    pub fn new(reader: &'a Reader) -> Self {
        Self {
            reader,
            tokenizer: Tokenizer::new(reader),
        }
    }

    fn token_err<T>(&self, token: Token) -> Result<T> {
        Err(ParseError {
            msg: format!("unexpected token: {}", token.value),
            pos: token.pos,
        })
    }

    pub fn parse(&mut self) -> Result<Node> {
        self.parse_expr1()
    }

    fn parse_expr1(&mut self) -> Result<Node> {
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

    fn parse_expr2(&mut self) -> Result<Node> {
        let mut left = self.parse_expr3()?;
        loop {
            let pos = self.reader.tell();
            let token = self.tokenizer.get()?;
            if token.kind == TokenKind::OrOr {
                let node = Node::BinaryOp {
                    pos: token.pos,
                    op: BinaryOpKind::Or,
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

    fn parse_expr3(&mut self) -> Result<Node> {
        let mut left = self.parse_expr4()?;
        loop {
            let pos = self.reader.tell();
            let token = self.tokenizer.get()?;
            if token.kind == TokenKind::AndAnd {
                let node = Node::BinaryOp {
                    pos: token.pos,
                    op: BinaryOpKind::And,
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

    fn parse_expr4(&mut self) -> Result<Node> {
        let mut left = self.parse_expr5()?;
        let cursor = self.reader.tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        let left_side = Box::new(left.clone());
        let op = match token.kind {
            TokenKind::EqEq => BinaryOpKind::EqEq,
            TokenKind::EqEqCI => BinaryOpKind::EqEqCI,
            TokenKind::EqEqCS => BinaryOpKind::EqEqCS,
            TokenKind::NotEq => BinaryOpKind::NotEq,
            TokenKind::NotEqCI => BinaryOpKind::NotEqCI,
            TokenKind::NotEqCS => BinaryOpKind::NotEqCS,
            TokenKind::GT => BinaryOpKind::GT,
            TokenKind::GTCI => BinaryOpKind::GTCI,
            TokenKind::GTCS => BinaryOpKind::GTCS,
            TokenKind::GTEq => BinaryOpKind::GTEq,
            TokenKind::GTEqCI => BinaryOpKind::GTEqCI,
            TokenKind::GTEqCS => BinaryOpKind::GTEqCS,
            TokenKind::LT => BinaryOpKind::LT,
            TokenKind::LTCI => BinaryOpKind::LTCI,
            TokenKind::LTCS => BinaryOpKind::LTCS,
            TokenKind::LTEq => BinaryOpKind::LTEq,
            TokenKind::LTEqCI => BinaryOpKind::LTEqCI,
            TokenKind::LTEqCS => BinaryOpKind::LTEqCS,
            TokenKind::Match => BinaryOpKind::Match,
            TokenKind::MatchCI => BinaryOpKind::MatchCI,
            TokenKind::MatchCS => BinaryOpKind::MatchCS,
            TokenKind::NoMatch => BinaryOpKind::NoMatch,
            TokenKind::NoMatchCI => BinaryOpKind::NoMatchCI,
            TokenKind::NoMatchCS => BinaryOpKind::NoMatchCS,
            TokenKind::Is => BinaryOpKind::Is,
            TokenKind::IsCI => BinaryOpKind::IsCI,
            TokenKind::IsCS => BinaryOpKind::IsCS,
            TokenKind::IsNot => BinaryOpKind::IsNot,
            TokenKind::IsNotCI => BinaryOpKind::IsNotCI,
            TokenKind::IsNotCS => BinaryOpKind::IsNotCS,
            _ => {
                self.reader.seek_set(cursor);
                return Ok(left);
            }
        };
        let node = Node::BinaryOp {
            pos,
            op,
            left: left_side,
            right: Box::new(self.parse_expr5()?),
        };
        left = node;
        Ok(left)
    }

    fn parse_expr5(&mut self) -> Result<Node> {
        let mut left = self.parse_expr6()?;
        loop {
            let cursor = self.reader.tell();
            let token = self.tokenizer.get()?;
            let pos = token.pos;
            let left_side = Box::new(left.clone());
            let op = match token.kind {
                TokenKind::Plus => BinaryOpKind::Add,
                TokenKind::Minus => BinaryOpKind::Subtract,
                TokenKind::Dot => BinaryOpKind::Concat,
                _ => {
                    self.reader.seek_set(cursor);
                    break;
                }
            };
            let node = Node::BinaryOp {
                pos,
                op,
                left: left_side,
                right: Box::new(self.parse_expr6()?),
            };
            left = node;
        }
        Ok(left)
    }

    fn parse_expr6(&mut self) -> Result<Node> {
        let mut left = self.parse_expr7()?;
        loop {
            let cursor = self.reader.tell();
            let token = self.tokenizer.get()?;
            let pos = token.pos;
            let left_side = Box::new(left.clone());
            let op = match token.kind {
                TokenKind::Star => BinaryOpKind::Multiply,
                TokenKind::Slash => BinaryOpKind::Divide,
                TokenKind::Percent => BinaryOpKind::Remainder,
                _ => {
                    self.reader.seek_set(cursor);
                    break;
                }
            };
            let node = Node::BinaryOp {
                pos,
                op,
                left: left_side,
                right: Box::new(self.parse_expr7()?),
            };
            left = node;
        }
        Ok(left)
    }

    fn parse_expr7(&mut self) -> Result<Node> {
        let cursor = self.reader.tell();
        let token = self.tokenizer.get()?;
        let pos = token.pos;
        let op = match token.kind {
            TokenKind::Not => UnaryOpKind::Not,
            TokenKind::Minus => UnaryOpKind::Minus,
            TokenKind::Plus => UnaryOpKind::Plus,
            _ => {
                self.reader.seek_set(cursor);
                return self.parse_expr8();
            }
        };
        let node = Node::UnaryOp {
            pos,
            op,
            right: Box::new(self.parse_expr7()?),
        };
        Ok(node)
    }

    fn parse_expr8(&mut self) -> Result<Node> {
        let mut left = self.parse_expr9()?;
        loop {
            let cursor = self.reader.tell();
            let c = self.reader.peek();
            let token = self.tokenizer.get()?;
            if !c.is_white() && token.kind == TokenKind::SqOpen {
                left = self.parse_slice(left, token.pos)?;
            } else if token.kind == TokenKind::POpen {
                let pos = token.pos;
                let name = Box::new(left);
                let mut args = vec![];
                if self.tokenizer.peek()?.kind == TokenKind::PClose {
                    self.tokenizer.get()?;
                } else {
                    loop {
                        args.push(self.parse_expr1()?);
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
            } else if !c.is_white() && token.kind == TokenKind::Dot {
                if let Some(node) = self.parse_dot(token, left.clone()) {
                    left = node;
                } else {
                    self.reader.seek_set(cursor);
                    break;
                }
            } else {
                self.reader.seek_set(cursor);
                break;
            }
        }
        Ok(left)
    }

    fn parse_expr9(&mut self) -> Result<Node> {
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
                        items.push(self.parse_expr1()?);
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
                // dict or lambda
                let savepos = self.reader.tell();
                let mut token = self.tokenizer.get()?;
                let mut is_lambda = token.kind == TokenKind::Arrow;
                if !is_lambda && ![TokenKind::SQuote, TokenKind::DQuote].contains(&token.kind) {
                    let token2 = self.tokenizer.peek()?;
                    is_lambda = [TokenKind::Arrow, TokenKind::Comma].contains(&token2.kind);
                }
                if is_lambda {
                    if let Some(node) = self.parse_lambda(token, pos)? {
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
                        if !items.is_empty() {
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
                let node = Node::ParenExpr {
                    pos: token.pos,
                    expr: Box::new(self.parse_expr1()?),
                };
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

    fn parse_lambda(&mut self, mut token: Token, pos: Position) -> Result<Option<Node>> {
        let mut fallback = false;
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
                            msg: format!("E853: Duplicate argument name: {}", token.value),
                            pos: token.pos,
                        });
                    }
                    named.push(token.value.clone());
                    let varnode = Node::Identifier {
                        pos: token.pos,
                        value: token.value,
                    };
                    let maybe_comma = self.tokenizer.peek()?.kind;
                    if self.reader.peek().is_white() && maybe_comma == TokenKind::Comma {
                        return Err(ParseError {
                            msg: String::from(
                                "E475: invalid argument: White space is not allowed before comma",
                            ),
                            pos: self.reader.getpos(),
                        });
                    }
                    token = self.tokenizer.get()?;
                    args.push(varnode);
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
                    args.push(varnode);
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
            return Ok(Some(node));
        }
        Ok(None)
    }

    fn parse_identifier(&mut self) -> Result<Node> {
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
                pieces: curly_parts.into_iter().collect::<Vec<Node>>(),
            });
        }
        Ok(node.unwrap())
    }

    fn parse_curly_parts(&mut self) -> Result<Vec<Node>> {
        let mut curly_parts = vec![];
        let c = self.reader.peek();
        let pos = self.reader.getpos();
        if c == '<' && self.reader.peekn(5).eq_ignore_ascii_case("<SID>") {
            let name = self.reader.getn(5);
            curly_parts.push(Node::CurlyNamePart { pos, value: name });
        }
        loop {
            let c = self.reader.peek();
            if c.is_name() {
                let pos = self.reader.getpos();
                let name = self.reader.read_name();
                curly_parts.push(Node::CurlyNamePart { pos, value: name });
            } else if c == '{' {
                self.reader.get();
                let pos = self.reader.getpos();
                curly_parts.push(Node::CurlyNameExpr {
                    pos,
                    expr: Box::new(self.parse_expr1()?),
                });
                self.reader.skip_white();
                let c = self.reader.peek();
                if c != '}' {
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
        if !self.reader.peek().is_word() {
            return None;
        }
        let pos = self.reader.getpos();
        let name = self.reader.read_word();
        if self.reader.peek().is_name() {
            return None;
        }
        let right = Box::new(Node::Identifier { pos, value: name });
        Some(Node::Dot {
            pos: token.pos,
            left: Box::new(left),
            right,
        })
    }

    fn parse_slice(&mut self, name: Node, pos: Position) -> Result<Node> {
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

    pub fn parse_lv(&mut self) -> Result<Node> {
        // this differs from parse_expr8() insofar as it will not parse function calls. this method
        // is used for parsing the lhs of a `for` or `let` command, e.g. `let foo = bar`. in this
        // case a function call is not valid, e.g. `let foo() = bar` is not valid syntax, so we
        // should not parse it.
        let mut left = self.parse_lv9()?;
        loop {
            let cursor = self.reader.tell();
            let c = self.reader.peek();
            let token = self.tokenizer.get()?;
            if !c.is_white() && token.kind == TokenKind::SqOpen {
                left = self.parse_slice(left, token.pos)?;
            } else if !c.is_white() && token.kind == TokenKind::Dot {
                if let Some(n) = self.parse_dot(token, left.clone()) {
                    left = n;
                } else {
                    self.reader.seek_set(cursor);
                    break;
                }
            } else {
                self.reader.seek_set(cursor);
                break;
            }
        }
        Ok(left)
    }

    fn parse_lv9(&mut self) -> Result<Node> {
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
    use super::super::{parse_lines, Node, Position};

    fn create_node(s: &str) -> Node {
        if let Node::TopLevel { body, .. } = parse_lines(&[s]).unwrap() {
            return body[0].clone();
        }
        panic!("can't create node from '{}'", s);
    }

    // tests below test parsing and fmt::Display formatting

    #[test]
    fn test_append() {
        let code = ["append", "foo", "bar", "."];
        let expected = "(excmd \"append \nfoo\nbar\n.\")";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_lang() {
        let code = ["python3 print('foo')"];
        let expected = "(excmd \"python3 print('foo')\")";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
        let code = ["python3 <<EOF", "print('foo')", "print('bar')", "EOF"];
        let expected = "(excmd \"python3 <<EOF\nprint('foo')\nprint('bar')\nEOF\")";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_loadkeymap() {
        let code = ["loadkeymap", "a A", "b B comment"];
        let expected = "(excmd \"loadkeymap \n\na A\nb B comment\")";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_augroup_and_autocmds() {
        let code = ["augroup foo", "autocmd VimEnter * Command", "augroup END"];
        let expected = concat!(
            "(excmd \"augroup foo\")\n",
            "(autocmd VimEnter * (excmd \"Command\"))\n",
            "(excmd \"augroup END\")"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_echo_and_binary_op() {
        let code = ["echo foo + bar"];
        let expected = "(echo (+ foo bar))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_blank_line() {
        let node = create_node("\n");
        let expected = Node::BlankLine {
            pos: Position {
                cursor: 0,
                line: 1,
                col: 1,
            },
        };
        assert_eq!(node, expected);
    }

    #[test]
    fn test_call_excall_and_identifier() {
        let code = ["call foo(bar, baz)"];
        let expected = "(call (foo bar baz))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_try_catch_finally_echomsg_and_echoerr() {
        let code = [
            "try",
            "echomsg 1",
            "catch /foo/",
            "echoerr 2",
            "catch",
            "echoerr 3",
            "finally",
            "echomsg 4",
            "endtry",
        ];
        let expected = concat!(
            "(try\n",
            "  (echomsg 1)\n",
            " catch /foo/\n",
            "  (echoerr 2)\n",
            " catch\n",
            "  (echoerr 3)\n",
            " finally\n",
            "  (echomsg 4))"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_comment_squote_string_let_and_unlet() {
        let code = [
            "\" NOT TRAILING COMMENT",
            "let x = 'something' \" trailing comment",
            "unlet x",
        ];
        let expected = concat!(
            "; NOT TRAILING COMMENT\n",
            "(let = x 'something')\n",
            "; trailing comment\n",
            "(unlet x)"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_curly_name() {
        let code = ["let foo{bar}baz = 'something'"];
        let expected = "(let = foo{bar}baz 'something')";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_dict_and_echon() {
        let code = ["echon {}", "echon {'foo': 1, 'bar': 2}"];
        let expected = "(echon (dict))\n(echon (dict ('foo' 1) ('bar' 2)))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_dot() {
        let code = ["echo foo.bar"];
        let expected = "(echo (dot foo bar))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_if_else_elseif_env_option_and_reg() {
        let code = [
            "if foo\n",
            "echo $ENV\n",
            "elseif bar\n",
            "echo &number\n",
            "else\n",
            "echo @r\n",
            "endif",
        ];
        let expected = concat!(
            "(if foo\n",
            "  (echo $ENV)\n",
            " elseif bar\n",
            "  (echo &number)\n",
            " else\n",
            "  (echo @r))"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_execute() {
        let code = ["execute UserCmd"];
        let expected = "(execute UserCmd)";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_excmd() {
        let code = ["UserCmd something 123"];
        let expected = "(excmd \"UserCmd something 123\")";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_for() {
        let code = ["for [a, b; z] in something", "echo a b z", "endfor"];
        let expected = concat!("(for (a b . z) something\n", "  (echo a b z))");
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_function_lambda_list_and_return() {
        let code = [
            "function! s:foo() abort dict",
            "  return map([1, 2, 3], {i, v -> v * 2 + i})",
            "endfunction",
            "delfunction s:foo",
        ];
        let expected = concat!(
            "(function (s:foo)\n",
            "  (return (map (list 1 2 3) (lambda (i v) (+ (* v 2) i)))))\n",
            "(excmd \"delfunction s:foo\")"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_lockvar_mapping_and_unlockvar() {
        let code = [
            "lockvar 1 foo",
            "nnoremap <expr> <silent> <C-x> SomeFunction()",
            "unlockvar 1 foo",
        ];
        let expected = concat!(
            "(lockvar 1 foo)\n",
            "(nnoremap <C-x> (SomeFunction))\n",
            "(unlockvar 1 foo)"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_parenexpr_and_bin_op() {
        let code = ["let x = ((a && b) || c * d)"];
        let expected = "(let = x (|| (&& a b) (* c d)))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_shebang() {
        let code = ["#!/usr/bin/vim"];
        let expected = "(#! \"/usr/bin/vim\")";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_string_subscript_and_slice() {
        let code = ["echo 'foobar'[1:-2][1]"];
        let expected = "(echo (subscript (slice 'foobar' 1 (- 2)) 1))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_ternary() {
        let code = ["echo foo ? 'bar' : 'baz'"];
        let expected = "(echo (?: foo 'bar' 'baz'))";
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_while_break_continue_and_throw() {
        let code = [
            "while 1",
            "throw 'EXCEPTION!!!'",
            "break",
            "continue",
            "endwhile",
        ];
        let expected = concat!(
            "(while 1\n",
            "  (throw 'EXCEPTION!!!')\n",
            "  (break)\n",
            "  (continue))"
        );
        assert_eq!(&format!("{}", parse_lines(&code).unwrap()), expected);
    }

    #[test]
    fn test_highlight() {
        let tests = [
            ("highlight", "(highlight)"),
            ("highlight String", "(highlight String)"),
            ("highlight clear", "(highlight clear)"),
            ("highlight clear String", "(highlight clear String)"),
            ("highlight String NONE", "(highlight String NONE)"),
            ("highlight default String", "(highlight default String)"),
            ("highlight link String NONE", "(highlight link String NONE)"),
            (
                "highlight default link String NONE",
                "(highlight default link String NONE)",
            ),
            (
                "highlight link String Comment",
                "(highlight link String Comment)",
            ),
            (
                "highlight String guifg=#123456 font='Monospace 10'",
                "(highlight String guifg=#123456 font='Monospace 10')",
            ),
        ];
        for (code, expected) in tests.iter() {
            assert_eq!(&format!("{}", parse_lines(&[code]).unwrap()), expected);
        }
        let err_tests = [
            ("highlight link", "E412"),
            ("highlight link String", "E412"),
            ("highlight String guifg", "E416"),
            ("highlight String font='Monospace 10", "E475"),
            ("highlight String foobar=123", "E423"),
        ];
        for (code, err) in err_tests.iter() {
            let result = parse_lines(&[code]);
            assert!(result.is_err());
            assert!(result.unwrap_err().msg.contains(err));
        }
    }
}
