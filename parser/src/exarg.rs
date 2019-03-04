use super::Position;
use crate::command::{Command, ParserKind};
use crate::modifier::Modifier;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct ExArg {
    pub bang: bool,
    pub use_filter: bool,
    pub linepos: Position,
    pub cmdpos: Position,
    pub argpos: Position,
    pub cmd: Rc<Command>,
    pub modifiers: Vec<Modifier>,
    pub range: Vec<String>,
}

impl ExArg {
    pub fn new() -> ExArg {
        ExArg {
            bang: false,
            use_filter: false,
            linepos: Position::empty(),
            cmdpos: Position::empty(),
            argpos: Position::empty(),
            cmd: Rc::new(Command {
                name: "Dummy".to_string(),
                minlen: 0,
                flags: vec![],
                parser: ParserKind::UserCmd,
            }),
            modifiers: vec![],
            range: vec![],
        }
    }
}
