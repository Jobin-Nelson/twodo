use crate::objects::{Project, Task};

use crossterm::event::EventStream;
use ratatui::widgets::ListState;
use sqlx::SqlitePool;
use tui_textarea::TextArea;

#[derive(Debug)]
pub struct App {
    pub db: SqlitePool,
    pub mode: Mode,
    pub event_stream: EventStream,
    pub twodo: Twodo,
    pub popover: Popover,
    pub state: State,
    pub view_data: ViewData,
}

#[derive(Debug, Default)]
pub struct Mode {
    pub app_mode: AppMode,
    pub add_task_mode: AddTaskMode,
}

#[derive(Debug, PartialEq, Default)]
pub enum AppMode {
    #[default]
    FocusTask,
    FocusProject,
    AddTask,
    AddSubTask,
    AddSiblingTask,
    Quit,
}

#[derive(Debug, Default)]
pub struct ViewData {
    pub task_depth: Vec<usize>,
}

#[derive(Debug, PartialEq, Default)]
pub enum AddTaskMode {
    #[default]
    AddTitle,
    AddDescription,
}

#[derive(Debug, Default)]
pub struct Popover {
    pub add_task: AddTask,
}

#[derive(Debug)]
pub struct AddTask {
    pub title: TextArea<'static>,
    pub description: TextArea<'static>,
}

#[derive(Debug)]
pub struct Twodo {
    pub tasks: Vec<Task>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Default)]
pub struct State {
    pub task_state: ListState,
    pub project_state: ListState,
}
