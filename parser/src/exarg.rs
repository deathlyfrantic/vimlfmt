use super::Position;
use crate::{
    command::{Command, Flag, ParserKind},
    modifier::Modifier,
};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ExArg {
    pub(crate) bang: bool,
    pub(crate) use_filter: bool,
    pub(crate) linepos: Position,
    pub(crate) cmdpos: Position,
    pub(crate) argpos: Position,
    pub(crate) cmd: Rc<Command>,
    pub(crate) modifiers: Vec<Modifier>,
    pub(crate) range: Vec<String>,
}

impl ExArg {
    pub(crate) fn new() -> Self {
        Self {
            bang: false,
            use_filter: false,
            linepos: Position::empty(),
            cmdpos: Position::empty(),
            argpos: Position::empty(),
            cmd: Rc::new(Command {
                name: "Dummy".to_string(),
                minlen: 0,
                flags: Flag::empty(),
                parser: ParserKind::UserCmd,
            }),
            modifiers: vec![],
            range: vec![],
        }
    }
}
