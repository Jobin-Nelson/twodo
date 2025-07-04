use crate::cli;

#[derive(Debug, PartialEq)]
pub enum Message {
    Noop,
    Quit,
    // Task messages
    AddTaskBegin,
    AddTaskCommit,
    AddTaskAbort,
    FocusTask,
    FocusAddTaskTitle,
    FocusAddTaskDescription,
    NextTask,
    PrevTask,
    TaskOp(cli::TaskOp),
    ReloadTask,

    // Project messages
    FocusProject,
    NextProject,
    PrevProject,
}
