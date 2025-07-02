use crate::cli;

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    TaskOp(cli::TaskOp),
    Noop,
    NextTask,
    PrevTask,
}
