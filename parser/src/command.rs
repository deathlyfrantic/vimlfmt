use bitflags::bitflags;
use lazy_static::lazy_static;
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

pub fn valid_autocmds() -> &'static HashMap<&'static str, String> {
    lazy_static! {
        static ref VALID_AUTOCMDS: HashMap<&'static str, String> = hashmap! {
            "bufadd"               => "BufAdd".to_string(),
            "bufcreate"            => "BufCreate".to_string(),
            "bufdelete"            => "BufDelete".to_string(),
            "bufenter"             => "BufEnter".to_string(),
            "buffilepost"          => "BufFilePost".to_string(),
            "buffilepre"           => "BufFilePre".to_string(),
            "bufhidden"            => "BufHidden".to_string(),
            "bufleave"             => "BufLeave".to_string(),
            "bufnew"               => "BufNew".to_string(),
            "bufnewfile"           => "BufNewFile".to_string(),
            "bufread"              => "BufRead".to_string(),
            "bufreadcmd"           => "BufReadCmd".to_string(),
            "bufreadpost"          => "BufReadPost".to_string(),
            "bufreadpre"           => "BufReadPre".to_string(),
            "bufunload"            => "BufUnload".to_string(),
            "bufwinenter"          => "BufWinEnter".to_string(),
            "bufwinleave"          => "BufWinLeave".to_string(),
            "bufwipeout"           => "BufWipeout".to_string(),
            "bufwrite"             => "BufWrite".to_string(),
            "bufwritecmd"          => "BufWriteCmd".to_string(),
            "bufwritepost"         => "BufWritePost".to_string(),
            "bufwritepre"          => "BufWritePre".to_string(),
            "chaninfo"             => "ChanInfo".to_string(),
            "chanopen"             => "ChanOpen".to_string(),
            "cmdundefined"         => "CmdUndefined".to_string(),
            "cmdlineenter"         => "CmdlineEnter".to_string(),
            "cmdlineleave"         => "CmdlineLeave".to_string(),
            "cmdwinenter"          => "CmdwinEnter".to_string(),
            "cmdwinleave"          => "CmdwinLeave".to_string(),
            "colorscheme"          => "ColorScheme".to_string(),
            "completedone"         => "CompleteDone".to_string(),
            "cursorhold"           => "CursorHold".to_string(),
            "cursorholdi"          => "CursorHoldI".to_string(),
            "cursormoved"          => "CursorMoved".to_string(),
            "cursormovedi"         => "CursorMovedI".to_string(),
            "dirchanged"           => "DirChanged".to_string(),
            "fileappendcmd"        => "FileAppendCmd".to_string(),
            "fileappendpost"       => "FileAppendPost".to_string(),
            "fileappendpre"        => "FileAppendPre".to_string(),
            "filechangedro"        => "FileChangedRO".to_string(),
            "filechangedshell"     => "FileChangedShell".to_string(),
            "filechangedshellpost" => "FileChangedShellPost".to_string(),
            "filereadcmd"          => "FileReadCmd".to_string(),
            "filereadpost"         => "FileReadPost".to_string(),
            "filereadpre"          => "FileReadPre".to_string(),
            "filetype"             => "FileType".to_string(),
            "filewritecmd"         => "FileWriteCmd".to_string(),
            "filewritepost"        => "FileWritePost".to_string(),
            "filewritepre"         => "FileWritePre".to_string(),
            "filterreadpost"       => "FilterReadPost".to_string(),
            "filterreadpre"        => "FilterReadPre".to_string(),
            "filterwritepost"      => "FilterWritePost".to_string(),
            "filterwritepre"       => "FilterWritePre".to_string(),
            "focusgained"          => "FocusGained".to_string(),
            "focuslost"            => "FocusLost".to_string(),
            "funcundefined"        => "FuncUndefined".to_string(),
            "guienter"             => "GUIEnter".to_string(),
            "guifailed"            => "GUIFailed".to_string(),
            "insertchange"         => "InsertChange".to_string(),
            "insertcharpre"        => "InsertCharPre".to_string(),
            "insertenter"          => "InsertEnter".to_string(),
            "insertleave"          => "InsertLeave".to_string(),
            "menupopup"            => "MenuPopup".to_string(),
            "optionset"            => "OptionSet".to_string(),
            "quickfixcmdpost"      => "QuickFixCmdPost".to_string(),
            "quickfixcmdpre"       => "QuickFixCmdPre".to_string(),
            "quitpre"              => "QuitPre".to_string(),
            "remotereply"          => "RemoteReply".to_string(),
            "sessionloadpost"      => "SessionLoadPost".to_string(),
            "shellcmdpost"         => "ShellCmdPost".to_string(),
            "shellfilterpost"      => "ShellFilterPost".to_string(),
            "sourcecmd"            => "SourceCmd".to_string(),
            "sourcepre"            => "SourcePre".to_string(),
            "spellfilemissing"     => "SpellFileMissing".to_string(),
            "stdinreadpost"        => "StdinReadPost".to_string(),
            "stdinreadpre"         => "StdinReadPre".to_string(),
            "swapexists"           => "SwapExists".to_string(),
            "syntax"               => "Syntax".to_string(),
            "tabclosed"            => "TabClosed".to_string(),
            "tabenter"             => "TabEnter".to_string(),
            "tableave"             => "TabLeave".to_string(),
            "tabnew"               => "TabNew".to_string(),
            "tabnewentered"        => "TabNewEntered".to_string(),
            "termclose"            => "TermClose".to_string(),
            "termopen"             => "TermOpen".to_string(),
            "termresponse"         => "TermResponse".to_string(),
            "textchanged"          => "TextChanged".to_string(),
            "textchangedi"         => "TextChangedI".to_string(),
            "textchangedp"         => "TextChangedP".to_string(),
            "textyankpost"         => "TextYankPost".to_string(),
            "user"                 => "User".to_string(),
            "vimenter"             => "VimEnter".to_string(),
            "vimleave"             => "VimLeave".to_string(),
            "vimleavepre"          => "VimLeavePre".to_string(),
            "vimresized"           => "VimResized".to_string(),
            "vimresume"            => "VimResume".to_string(),
            "vimsuspend"           => "VimSuspend".to_string(),
            "winenter"             => "WinEnter".to_string(),
            "winleave"             => "WinLeave".to_string(),
            "winnew"               => "WinNew".to_string(),
        };
    }
    &VALID_AUTOCMDS
}

bitflags! {
    /// flags taken directly from ex_cmds_defs.h in neovim source
    pub struct Flag: u32 {
        /// allow a linespecs
        const RANGE     = 0b000000000000000000000001;
        /// allow a ! after the command name
        const BANG      = 0b000000000000000000000010;
        /// allow extra args after command name
        const EXTRA     = 0b000000000000000000000100;
        /// expand wildcards in extra part
        const XFILE     = 0b000000000000000000001000;
        /// no spaces allowed in the extra part
        const NOSPC     = 0b000000000000000000010000;
        /// default file range is 1,$
        const DFLALL    = 0b000000000000000000100000;
        /// extend range to include whole fold also when less than two numbers given
        const WHOLEFOLD = 0b000000000000000001000000;
        /// argument required
        const NEEDARG   = 0b000000000000000010000000;
        /// check for trailing vertical bar
        const TRLBAR    = 0b000000000000000100000000;
        /// allow "x for register designation
        const REGSTR    = 0b000000000000001000000000;
        /// allow count in argument, after command
        const COUNT     = 0b000000000000010000000000;
        /// no trailing comment allowed
        const NOTRLCOM  = 0b000000000000100000000000;
        /// zero line number allowed
        const ZEROR     = 0b000000000001000000000000;
        /// do not remove CTRL-V from argument
        const USECTRLV  = 0b000000000010000000000000;
        /// number before command is not an address
        const NOTADR    = 0b000000000100000000000000;
        /// allow "+command" argument
        const EDITCMD   = 0b000000010000000000000000;
        /// accepts buffer name
        const BUFNAME   = 0b000000001000000000000000;
        /// accepts unlisted buffer too
        const BUFUNL    = 0b000000100000000000000000;
        /// allow "++opt=val" argument
        const ARGOPT    = 0b000001000000000000000000;
        /// allowed in the sandbox
        const SBOXOK    = 0b000010000000000000000000;
        /// allowed in cmdline window; when missing disallows editing another buffer when
        /// curbuf_lock is set
        const CMDWIN    = 0b000100000000000000000000;
        /// forbidden in non-'modifiable' buffer
        const MODIFY    = 0b001000000000000000000000;
        /// allow flags after count in argument
        const EXFLAGS   = 0b010000000000000000000000;
        /// multiple extra files allowed
        const FILES     = Self::XFILE.bits | Self::EXTRA.bits;
        /// one extra word allowed
        const WORD1     = Self::EXTRA.bits | Self::NOSPC.bits;
        /// 1 file allowed, defaults to current file
        const FILE1     = Self::FILES.bits | Self::NOSPC.bits;
        /// whether this is a user-defined command or a built-in one (specific to this parser)
        const USERCMD   = 0b100000000000000000000000;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserKind {
    Append,
    Augroup,
    Autocmd,
    Break,
    Call,
    Catch,
    Common,
    Continue,
    DelFunction,
    Echo,
    EchoHl,
    Else,
    ElseIf,
    EndFor,
    EndFunction,
    EndIf,
    EndTry,
    EndWhile,
    Execute,
    Finally,
    Finish,
    For,
    Function,
    If,
    Insert,
    Lang,
    Let,
    LoadKeymap,
    LockVar,
    Mapping,
    Return,
    Syntax,
    Throw,
    Try,
    Unlet,
    UserCmd,
    While,
    WinCmd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub name: String,
    pub minlen: usize,
    pub flags: Flag,
    pub parser: ParserKind,
}

fn command_vec() -> Vec<Command> {
    vec![
        Command {
            name: "append".to_string(),
            minlen: 1,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::ZEROR
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Append,
        },
        Command {
            name: "abbreviate".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "abclear".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "aboveleft".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "all".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "amenu".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "anoremenu".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "args".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILES | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "argadd".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::FILES
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "argdelete".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::FILES | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "argedit".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::FILE1
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "argdo".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "argglobal".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::FILES | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "arglocal".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::FILES | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "argument".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ascii".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "autocmd".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::EXTRA | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Autocmd,
        },
        Command {
            name: "augroup".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Augroup,
        },
        Command {
            name: "aunmenu".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "buffer".to_string(),
            minlen: 1,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::BUFNAME
                | Flag::BUFUNL
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "bNext".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ball".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "badd".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::FILE1 | Flag::EDITCMD | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "bdelete".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::BUFNAME
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "behave".to_string(),
            minlen: 2,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "belowright".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "bfirst".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "blast".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "bmodified".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "bnext".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "botright".to_string(),
            minlen: 2,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "bprevious".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "brewind".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "break".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Break,
        },
        Command {
            name: "breakadd".to_string(),
            minlen: 6,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "breakdel".to_string(),
            minlen: 6,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "breaklist".to_string(),
            minlen: 6,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "browse".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "bufdo".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "buffers".to_string(),
            minlen: 7,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "bunload".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::BUFNAME
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "bwipeout".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::BUFNAME
                | Flag::BUFUNL
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "change".to_string(),
            minlen: 1,
            flags: Flag::BANG
                | Flag::WHOLEFOLD
                | Flag::RANGE
                | Flag::COUNT
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "cNext".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cNfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cabbrev".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cabclear".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "caddbuffer".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::WORD1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "caddexpr".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::NOTRLCOM | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "caddfile".to_string(),
            minlen: 5,
            flags: Flag::TRLBAR | Flag::FILE1,
            parser: ParserKind::Common,
        },
        Command {
            name: "call".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Call,
        },
        Command {
            name: "catch".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Catch,
        },
        Command {
            name: "cbuffer".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::WORD1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "cc".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cclose".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "cd".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "center".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR
                | Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "cexpr".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::NOTRLCOM | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cfile".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::FILE1 | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cfirst".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cgetbuffer".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::NOTADR | Flag::WORD1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "cgetexpr".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::NOTRLCOM | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "cgetfile".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::FILE1,
            parser: ParserKind::Common,
        },
        Command {
            name: "changes".to_string(),
            minlen: 7,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "chdir".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "checkpath".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::BANG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "checktime".to_string(),
            minlen: 6,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BUFNAME
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "clist".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "clast".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "close".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "cmapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cmenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cnext".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cnewer".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "cnfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cnoremap".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "cnoreabbrev".to_string(),
            minlen: 6,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cnoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "copy".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "colder".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "colorscheme".to_string(),
            minlen: 4,
            flags: Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "command".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::BANG | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "comclear".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "compiler".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1 | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "continue".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Continue,
        },
        Command {
            name: "confirm".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "copen".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "cprevious".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cpfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cquit".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "crewind".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "cscope".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "cstag".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "cunmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cunabbrev".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cunmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "cwindow".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "delete".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::REGSTR
                | Flag::COUNT
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "delmarks".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "debug".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "debuggreedy".to_string(),
            minlen: 6,
            flags: Flag::RANGE | Flag::NOTADR | Flag::ZEROR | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "delcommand".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "delfunction".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::NEEDARG | Flag::WORD1 | Flag::CMDWIN,
            parser: ParserKind::DelFunction,
        },
        Command {
            name: "diffupdate".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "diffget".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::EXTRA | Flag::TRLBAR | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "diffoff".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "diffpatch".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::FILE1 | Flag::TRLBAR | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "diffput".to_string(),
            minlen: 6,
            flags: Flag::RANGE | Flag::EXTRA | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "diffsplit".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::FILE1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "diffthis".to_string(),
            minlen: 5,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "digraphs".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "display".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "djump".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::DFLALL | Flag::WHOLEFOLD | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "dlist".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::DFLALL
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "doautocmd".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "doautoall".to_string(),
            minlen: 7,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "drop".to_string(),
            minlen: 2,
            flags: Flag::FILES | Flag::EDITCMD | Flag::NEEDARG | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "dsearch".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::DFLALL
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "dsplit".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::RANGE | Flag::DFLALL | Flag::WHOLEFOLD | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "edit".to_string(),
            minlen: 1,
            flags: Flag::BANG | Flag::FILE1 | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "earlier".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::EXTRA | Flag::NOSPC | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "echo".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Echo,
        },
        Command {
            name: "echoerr".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Echo,
        },
        Command {
            name: "echohl".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::EchoHl,
        },
        Command {
            name: "echomsg".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Echo,
        },
        Command {
            name: "echon".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Echo,
        },
        Command {
            name: "else".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Else,
        },
        Command {
            name: "elseif".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::ElseIf,
        },
        Command {
            name: "emenu".to_string(),
            minlen: 2,
            flags: Flag::NEEDARG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "endif".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::EndIf,
        },
        Command {
            name: "endfor".to_string(),
            minlen: 5,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::EndFor,
        },
        Command {
            name: "endfunction".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::EndFunction,
        },
        Command {
            name: "endtry".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::EndTry,
        },
        Command {
            name: "endwhile".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::EndWhile,
        },
        Command {
            name: "enew".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ex".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "execute".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Execute,
        },
        Command {
            name: "exit".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::DFLALL
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "exusage".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "file".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::BANG
                | Flag::FILE1
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "files".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "filetype".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "find".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::FILE1
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "finally".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Finally,
        },
        Command {
            name: "finish".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Finish,
        },
        Command {
            name: "first".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::BANG | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "fixdel".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "fold".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "foldclose".to_string(),
            minlen: 5,
            flags: Flag::RANGE
                | Flag::BANG
                | Flag::WHOLEFOLD
                | Flag::TRLBAR
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "folddoopen".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::DFLALL | Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "folddoclosed".to_string(),
            minlen: 7,
            flags: Flag::RANGE | Flag::DFLALL | Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "foldopen".to_string(),
            minlen: 5,
            flags: Flag::RANGE
                | Flag::BANG
                | Flag::WHOLEFOLD
                | Flag::TRLBAR
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "for".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::For,
        },
        Command {
            name: "function".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::BANG | Flag::CMDWIN,
            parser: ParserKind::Function,
        },
        Command {
            name: "global".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::EXTRA
                | Flag::DFLALL
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "goto".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::TRLBAR
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "grep".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "grepadd".to_string(),
            minlen: 5,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "gui".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILES
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "gvim".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILES
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "hardcopy".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::DFLALL
                | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "help".to_string(),
            minlen: 1,
            flags: Flag::BANG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "helpfind".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "helpgrep".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::NEEDARG,
            parser: ParserKind::Common,
        },
        Command {
            name: "helptags".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::FILES | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "highlight".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::EXTRA | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "hide".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "history".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "insert".to_string(),
            minlen: 1,
            flags: Flag::BANG | Flag::RANGE | Flag::TRLBAR | Flag::CMDWIN | Flag::MODIFY,
            parser: ParserKind::Insert,
        },
        Command {
            name: "iabbrev".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "iabclear".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "if".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::If,
        },
        Command {
            name: "ijump".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::DFLALL | Flag::WHOLEFOLD | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "ilist".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::DFLALL
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "imap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "imapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "imenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "inoremap".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "inoreabbrev".to_string(),
            minlen: 6,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "inoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "intro".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "isearch".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::DFLALL
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "isplit".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::RANGE | Flag::DFLALL | Flag::WHOLEFOLD | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "iunmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "iunabbrev".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "iunmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "join".to_string(),
            minlen: 1,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "jumps".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "k".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WORD1 | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "keepalt".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "keepmarks".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "keepjumps".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "keeppatterns".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "lNext".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lNfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "list".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "laddexpr".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::NOTRLCOM | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "laddbuffer".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::NOTADR | Flag::WORD1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "laddfile".to_string(),
            minlen: 5,
            flags: Flag::TRLBAR | Flag::FILE1,
            parser: ParserKind::Common,
        },
        Command {
            name: "last".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::BANG | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "language".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "later".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::EXTRA | Flag::NOSPC | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lbuffer".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::WORD1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lcd".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lchdir".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lclose".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lcscope".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "left".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR
                | Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "leftabove".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "let".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Let,
        },
        Command {
            name: "lexpr".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::NOTRLCOM | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lfile".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::FILE1 | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lfirst".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lgetbuffer".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::NOTADR | Flag::WORD1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lgetexpr".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::NOTRLCOM | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lgetfile".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::FILE1,
            parser: ParserKind::Common,
        },
        Command {
            name: "lgrep".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "lgrepadd".to_string(),
            minlen: 6,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "lhelpgrep".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::NEEDARG,
            parser: ParserKind::Common,
        },
        Command {
            name: "ll".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "llast".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "list".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lmake".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::EXTRA | Flag::NOTRLCOM | Flag::TRLBAR | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "lmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "lmapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lnext".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lnewer".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lnfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lnoremap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "loadkeymap".to_string(),
            minlen: 5,
            flags: Flag::CMDWIN,
            parser: ParserKind::LoadKeymap,
        },
        Command {
            name: "loadview".to_string(),
            minlen: 2,
            flags: Flag::FILE1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lockmarks".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "lockvar".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::EXTRA | Flag::NEEDARG | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::LockVar,
        },
        Command {
            name: "lolder".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lopen".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "lprevious".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lpfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "lrewind".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR | Flag::BANG,
            parser: ParserKind::Common,
        },
        Command {
            name: "ls".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "ltag".to_string(),
            minlen: 2,
            flags: Flag::NOTADR | Flag::TRLBAR | Flag::BANG | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "lunmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lua".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "luado".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::DFLALL | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "luafile".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "lvimgrep".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "lvimgrepadd".to_string(),
            minlen: 9,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "lwindow".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "move".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "mark".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::WORD1 | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "make".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::EXTRA | Flag::NOTRLCOM | Flag::TRLBAR | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "map".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "mapclear".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "marks".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "match".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::EXTRA | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "menu".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::BANG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "menutranslate".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "messages".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "mkexrc".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "mksession".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "mkspell".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "mkvimrc".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "mkview".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "mode".to_string(),
            minlen: 3,
            flags: Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "mzscheme".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::EXTRA
                | Flag::DFLALL
                | Flag::NEEDARG
                | Flag::CMDWIN
                | Flag::SBOXOK,
            parser: ParserKind::Lang,
        },
        Command {
            name: "mzfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "nbclose".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "nbkey".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTADR | Flag::NEEDARG,
            parser: ParserKind::Common,
        },
        Command {
            name: "nbstart".to_string(),
            minlen: 3,
            flags: Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "next".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::FILES
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "new".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "nmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "nmapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "nmenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "nnoremap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "nnoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "noautocmd".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "noremap".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "nohlsearch".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "noreabbrev".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "noremenu".to_string(),
            minlen: 6,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::BANG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "normal".to_string(),
            minlen: 4,
            flags: Flag::RANGE
                | Flag::BANG
                | Flag::EXTRA
                | Flag::NEEDARG
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "number".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "nunmap".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "nunmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "oldfiles".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "open".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::BANG | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "omap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "omapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "omenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "only".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "onoremap".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "onoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "options".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ounmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "ounmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "ownsyntax".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "pclose".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "pedit".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "perl".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::EXTRA
                | Flag::DFLALL
                | Flag::NEEDARG
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "print".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::SBOXOK,
            parser: ParserKind::Common,
        },
        Command {
            name: "profdel".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "profile".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "promptfind".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "promptrepl".to_string(),
            minlen: 7,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "perldo".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::EXTRA | Flag::DFLALL | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "pop".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::COUNT
                | Flag::TRLBAR
                | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "popup".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG
                | Flag::EXTRA
                | Flag::BANG
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "ppop".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::COUNT
                | Flag::TRLBAR
                | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "preserve".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "previous".to_string(),
            minlen: 4,
            flags: Flag::EXTRA
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::BANG
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "psearch".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::RANGE | Flag::WHOLEFOLD | Flag::DFLALL | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptag".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::WORD1
                | Flag::TRLBAR
                | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptNext".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptfirst".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptjump".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptlast".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptnext".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptprevious".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptrewind".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "ptselect".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "put".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::REGSTR
                | Flag::TRLBAR
                | Flag::ZEROR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "pwd".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "py3".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "python3".to_string(),
            minlen: 7,
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "py3file".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "python".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "pyfile".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "pydo".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::DFLALL | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "py3do".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::DFLALL | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "quit".to_string(),
            minlen: 1,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "quitall".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "qall".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "read".to_string(),
            minlen: 1,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::TRLBAR
                | Flag::ZEROR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "recover".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "redo".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "redir".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::FILES | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "redraw".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "redrawstatus".to_string(),
            minlen: 7,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "registers".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "resize".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "retab".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR
                | Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::DFLALL
                | Flag::BANG
                | Flag::WORD1
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "return".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Return,
        },
        Command {
            name: "rewind".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::BANG | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "right".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR
                | Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "rightbelow".to_string(),
            minlen: 6,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "ruby".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "rubydo".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::DFLALL | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "rubyfile".to_string(),
            minlen: 5,
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "rundo".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG | Flag::FILE1,
            parser: ParserKind::Common,
        },
        Command {
            name: "runtime".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::FILES
                | Flag::TRLBAR
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "rviminfo".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "substitute".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "sNext".to_string(),
            minlen: 2,
            flags: Flag::EXTRA
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::BANG
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sandbox".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "sargument".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sall".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "saveas".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::DFLALL
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::CMDWIN
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbuffer".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::BUFNAME
                | Flag::BUFUNL
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbNext".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sball".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbfirst".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sblast".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbmodified".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbnext".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbprevious".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sbrewind".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "scriptnames".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "scriptencoding".to_string(),
            minlen: 7,
            flags: Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "scscope".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "set".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::EXTRA | Flag::CMDWIN | Flag::SBOXOK,
            parser: ParserKind::Common,
        },
        Command {
            name: "setfiletype".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "setglobal".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::EXTRA | Flag::CMDWIN | Flag::SBOXOK,
            parser: ParserKind::Common,
        },
        Command {
            name: "setlocal".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::EXTRA | Flag::CMDWIN | Flag::SBOXOK,
            parser: ParserKind::Common,
        },
        Command {
            name: "sfind".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sfirst".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::BANG | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "shell".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "simalt".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "sign".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::RANGE | Flag::NOTADR | Flag::EXTRA | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "silent".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG
                | Flag::EXTRA
                | Flag::BANG
                | Flag::NOTRLCOM
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "sleep".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "slast".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::BANG | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "smagic".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "smap".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "smapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "smenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "snext".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::FILES
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sniff".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "snomagic".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "snoremap".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "snoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "sort".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::DFLALL
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "source".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "spelldump".to_string(),
            minlen: 6,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "spellgood".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "spellinfo".to_string(),
            minlen: 6,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "spellrepall".to_string(),
            minlen: 6,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "spellundo".to_string(),
            minlen: 6,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "spellwrong".to_string(),
            minlen: 6,
            flags: Flag::BANG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "split".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sprevious".to_string(),
            minlen: 3,
            flags: Flag::EXTRA
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::BANG
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "srewind".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::BANG | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "stop".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::BANG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "stag".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::WORD1
                | Flag::TRLBAR
                | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "startinsert".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "startgreplace".to_string(),
            minlen: 6,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "startreplace".to_string(),
            minlen: 6,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "stopinsert".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "stjump".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "stselect".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "sunhide".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "sunmap".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "sunmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "suspend".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::BANG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "sview".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "swapname".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "syntax".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Syntax,
        },
        Command {
            name: "syntime".to_string(),
            minlen: 5,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "syncbind".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "t".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "tNext".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabNext".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabclose".to_string(),
            minlen: 4,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::BANG
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabdo".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabedit".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabfind".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::NEEDARG
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabfirst".to_string(),
            minlen: 6,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tablast".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabmove".to_string(),
            minlen: 4,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::NOSPC
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabnew".to_string(),
            minlen: 6,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabnext".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabonly".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabprevious".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabrewind".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tabs".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tab".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "tag".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::WORD1
                | Flag::TRLBAR
                | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tags".to_string(),
            minlen: 4,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tcl".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Lang,
        },
        Command {
            name: "tcldo".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::DFLALL | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tclfile".to_string(),
            minlen: 4,
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tearoff".to_string(),
            minlen: 2,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tfirst".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "throw".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NEEDARG | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Throw,
        },
        Command {
            name: "tjump".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "tlast".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "tmenu".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tnext".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "topleft".to_string(),
            minlen: 2,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "tprevious".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "trewind".to_string(),
            minlen: 2,
            flags: Flag::RANGE | Flag::NOTADR | Flag::BANG | Flag::TRLBAR | Flag::ZEROR,
            parser: ParserKind::Common,
        },
        Command {
            name: "try".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Try,
        },
        Command {
            name: "tselect".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR | Flag::WORD1,
            parser: ParserKind::Common,
        },
        Command {
            name: "tunmenu".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "undo".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::ZEROR
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "undojoin".to_string(),
            minlen: 5,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "undolist".to_string(),
            minlen: 5,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "unabbreviate".to_string(),
            minlen: 3,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "unhide".to_string(),
            minlen: 3,
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "unlet".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::EXTRA | Flag::NEEDARG | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Unlet,
        },
        Command {
            name: "unlockvar".to_string(),
            minlen: 4,
            flags: Flag::BANG | Flag::EXTRA | Flag::NEEDARG | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::LockVar,
        },
        Command {
            name: "unmap".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "unmenu".to_string(),
            minlen: 4,
            flags: Flag::BANG
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "unsilent".to_string(),
            minlen: 3,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "update".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::DFLALL
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "vglobal".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::DFLALL | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "version".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "verbose".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::SBOXOK
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "vertical".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "vimgrep".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "vimgrepadd".to_string(),
            minlen: 8,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::TRLBAR
                | Flag::XFILE,
            parser: ParserKind::Common,
        },
        Command {
            name: "visual".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "viusage".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "view".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::EDITCMD | Flag::ARGOPT | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "vmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "vmapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "vmenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "vnew".to_string(),
            minlen: 3,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "vnoremap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "vnoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "vsplit".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "vunmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "vunmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "windo".to_string(),
            minlen: 5,
            flags: Flag::BANG | Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "write".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::DFLALL
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "wNext".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::NOTADR
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "wall".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "while".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTRLCOM | Flag::SBOXOK | Flag::CMDWIN,
            parser: ParserKind::While,
        },
        Command {
            name: "winsize".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NEEDARG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "wincmd".to_string(),
            minlen: 4,
            flags: Flag::NEEDARG | Flag::WORD1 | Flag::RANGE | Flag::NOTADR,
            parser: ParserKind::WinCmd,
        },
        Command {
            name: "winpos".to_string(),
            minlen: 4,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "wnext".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "wprevious".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "wq".to_string(),
            minlen: 2,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::DFLALL
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "wqall".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::ARGOPT | Flag::DFLALL | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "wsverb".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::NOTADR | Flag::NEEDARG,
            parser: ParserKind::Common,
        },
        Command {
            name: "wundo".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::NEEDARG | Flag::FILE1,
            parser: ParserKind::Common,
        },
        Command {
            name: "wviminfo".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "xit".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::BANG
                | Flag::FILE1
                | Flag::ARGOPT
                | Flag::DFLALL
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "xall".to_string(),
            minlen: 2,
            flags: Flag::BANG | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "xmapclear".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "xmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "xmenu".to_string(),
            minlen: 3,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "xnoremap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Mapping,
        },
        Command {
            name: "xnoremenu".to_string(),
            minlen: 7,
            flags: Flag::RANGE
                | Flag::NOTADR
                | Flag::ZEROR
                | Flag::EXTRA
                | Flag::TRLBAR
                | Flag::NOTRLCOM
                | Flag::USECTRLV
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "xunmap".to_string(),
            minlen: 2,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "xunmenu".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "yank".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::REGSTR
                | Flag::COUNT
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "z".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::EXTRA
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "!".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::BANG | Flag::FILES | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "#".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "&".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::CMDWIN | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "*".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "<".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "=".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::TRLBAR | Flag::DFLALL | Flag::EXFLAGS | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: ">".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN
                | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            name: "@".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "Next".to_string(),
            minlen: 1,
            flags: Flag::EXTRA
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::COUNT
                | Flag::BANG
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "Print".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "X".to_string(),
            minlen: 1,
            flags: Flag::TRLBAR,
            parser: ParserKind::Common,
        },
        Command {
            name: "~".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::WHOLEFOLD | Flag::EXTRA | Flag::CMDWIN | Flag::MODIFY,
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::TRLBAR,
            minlen: 3,
            name: "cbottom".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::DFLALL,
            minlen: 3,
            name: "cdo".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::DFLALL,
            minlen: 3,
            name: "cfdo".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::TRLBAR,
            minlen: 3,
            name: "chistory".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::TRLBAR | Flag::CMDWIN,
            minlen: 3,
            name: "clearjumps".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG | Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            minlen: 4,
            name: "filter".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::RANGE | Flag::NOTADR | Flag::COUNT | Flag::TRLBAR,
            minlen: 5,
            name: "helpclose".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::TRLBAR,
            minlen: 3,
            name: "lbottom".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::DFLALL,
            minlen: 2,
            name: "ldo".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG
                | Flag::NEEDARG
                | Flag::EXTRA
                | Flag::NOTRLCOM
                | Flag::RANGE
                | Flag::NOTADR
                | Flag::DFLALL,
            minlen: 3,
            name: "lfdo".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::TRLBAR,
            minlen: 3,
            name: "lhistory".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG | Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            minlen: 3,
            name: "llist".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::NOTRLCOM,
            minlen: 3,
            name: "noswapfile".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG
                | Flag::FILE1
                | Flag::NEEDARG
                | Flag::TRLBAR
                | Flag::SBOXOK
                | Flag::CMDWIN,
            minlen: 2,
            name: "packadd".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::BANG | Flag::TRLBAR | Flag::SBOXOK | Flag::CMDWIN,
            minlen: 5,
            name: "packloadall".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::TRLBAR | Flag::CMDWIN | Flag::SBOXOK,
            minlen: 3,
            name: "smile".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            minlen: 3,
            name: "pyx".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::RANGE | Flag::DFLALL | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            minlen: 4,
            name: "pyxdo".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::RANGE | Flag::EXTRA | Flag::NEEDARG | Flag::CMDWIN,
            minlen: 7,
            name: "pythonx".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::RANGE | Flag::FILE1 | Flag::NEEDARG | Flag::CMDWIN,
            minlen: 4,
            name: "pyxfile".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::RANGE | Flag::BANG | Flag::FILES | Flag::CMDWIN,
            minlen: 3,
            name: "terminal".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            minlen: 3,
            name: "tmap".to_string(),
            parser: ParserKind::Mapping,
        },
        Command {
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::CMDWIN,
            minlen: 5,
            name: "tmapclear".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            minlen: 3,
            name: "tnoremap".to_string(),
            parser: ParserKind::Mapping,
        },
        Command {
            flags: Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::USECTRLV | Flag::CMDWIN,
            minlen: 5,
            name: "tunmap".to_string(),
            parser: ParserKind::Common,
        },
        Command {
            name: "rshada".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "wshada".to_string(),
            minlen: 3,
            flags: Flag::BANG | Flag::FILE1 | Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "Print".to_string(),
            minlen: 1,
            flags: Flag::RANGE
                | Flag::WHOLEFOLD
                | Flag::COUNT
                | Flag::EXFLAGS
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "fixdel".to_string(),
            minlen: 3,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "helpfind".to_string(),
            minlen: 5,
            flags: Flag::EXTRA | Flag::NOTRLCOM,
            parser: ParserKind::Common,
        },
        Command {
            name: "open".to_string(),
            minlen: 1,
            flags: Flag::RANGE | Flag::BANG | Flag::EXTRA,
            parser: ParserKind::Common,
        },
        Command {
            name: "shell".to_string(),
            minlen: 2,
            flags: Flag::TRLBAR | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "tearoff".to_string(),
            minlen: 2,
            flags: Flag::NEEDARG | Flag::EXTRA | Flag::TRLBAR | Flag::NOTRLCOM | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
        Command {
            name: "gvim".to_string(),
            minlen: 2,
            flags: Flag::BANG
                | Flag::FILES
                | Flag::EDITCMD
                | Flag::ARGOPT
                | Flag::TRLBAR
                | Flag::CMDWIN,
            parser: ParserKind::Common,
        },
    ]
}

fn command_hashmap(commands: Vec<Command>) -> HashMap<String, Rc<Command>> {
    let mut map = HashMap::new();
    for cmd in commands {
        let cmd = Rc::new(cmd);
        for i in cmd.minlen..=cmd.name.len() {
            let key = cmd.name.get(0..i).unwrap().to_string();
            map.insert(key, Rc::clone(&cmd));
        }
    }
    map
}

pub fn commands() -> HashMap<String, Rc<Command>> {
    command_hashmap(command_vec())
}
