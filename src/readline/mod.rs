use std::borrow::Cow::{self, Owned};

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::{Builder, ColorMode, CompletionType, EditMode};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::{Context, Editor, Helper};

use regex::Regex;

use crate::eval_math_expression;

pub struct RLHelper {
    completer: FilenameCompleter,
    highlighter: LineHighlighter,
    hinter: HistoryHinter,
}

struct LineHighlighter {}
impl Highlighter for LineHighlighter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }
    fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
        let op = eval_math_expression(line, 0f64);
        match op {
            Ok(_) => {
                let constants = ["e", "pi"];
                let functions = [
                    "sin", "cos", "tan", "csc", "sec", "cot", "sinh", "cosh", "tanh", "ln", "log",
                    "sqrt", "ceil", "floor", "rad", "deg", "abs", "asin", "acos", "atan", "acsc",
                    "asec", "acot",
                ];
                let ops = Regex::new(r"(?P<o>[\+-/\*%\^!])").unwrap();
                let mut coloured: String = ops.replace_all(line, "\x1b[33m$o\x1b[0m").into();

                for c in &constants {
                    coloured = coloured.replace(c, &format!("\x1b[33m{}\x1b[0m", c));
                }
                for f in &functions {
                    coloured = coloured.replace(f, &format!("\x1b[34m{}\x1b[0m", f));
                }
                Owned(coloured)
            }
            Err(_) => Owned(format!("\x1b[31m{}\x1b[0m", line)),
        }
    }
}

impl Highlighter for RLHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        self.highlighter.highlight_hint(hint)
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
}

impl Completer for RLHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for RLHelper {
    fn hint(&self, line: &str, a: usize, b: &Context) -> Option<String> {
        self.hinter.hint(line, a, b)
    }
}

impl Helper for RLHelper {}

pub fn create_readline() -> Editor<RLHelper> {
    let config_builder = Builder::new();
    let config = config_builder
        .color_mode(ColorMode::Enabled)
        .edit_mode(EditMode::Emacs)
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .max_history_size(1000)
        .build();
    let mut rl = Editor::with_config(config);
    let h = RLHelper {
        completer: FilenameCompleter::new(),
        highlighter: LineHighlighter {},
        hinter: HistoryHinter {},
    };
    rl.set_helper(Some(h));
    rl
}
