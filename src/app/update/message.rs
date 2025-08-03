use crate::cli;

#[derive(Debug, PartialEq)]
pub enum Message {
    Noop,
    Quit,
    // Task messages
    AddTaskBegin,
    AddSubTaskBegin,
    AddSiblingTaskBegin,
    AddTaskCommit,
    AddTaskAbort,
    FocusTask,
    FocusAddTaskTitle,
    FocusAddTaskDescription,
    TaskOp(cli::TaskOp),
    ReloadTask,
    DeleteTask,
    ToggleTaskStatus,
    SelectNextTask,
    SelectPrevTask,
    SelectFirstTask,
    SelectLastTask,

    // Project messages
    FocusProject,
    SelectNextProject,
    SelectPrevProject,
    SelectFirstProject,
    SelectLastProject,
    AddProjectBegin,
    AddProjectCommit,
    AddProjectAbort,
    FocusAddProjectName,
    ProjectOp(cli::ProjectOp),
    ReloadProject,
}
