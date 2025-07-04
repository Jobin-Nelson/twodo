use crate::cli;

#[derive(Debug, PartialEq)]
pub enum Message {
    Noop,
    Quit,
    // Task messages
    FocusTask,
    NextTask,
    PrevTask,
    TaskOp(cli::TaskOp),

    // Project messages
    FocusProject,
    NextProject,
    PrevProject,
}
