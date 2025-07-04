use crate::objects::{Project, Task};

use crossterm::event::EventStream;
use ratatui::widgets::{ListState, TableState};
use sqlx::SqlitePool;
use tui_textarea::TextArea;

#[derive(Debug)]
pub struct App {
    pub db: SqlitePool,
    pub app_state: AppState,
    pub event_stream: EventStream,
    pub twodo: Twodo,
    pub popover: Popover,
    pub state: State,
}

#[derive(Debug, PartialEq, Default)]
pub enum AppState {
    #[default]
    NormalTask,
    AddTask,
    NormalProject,
    CloseApp,
}

#[derive(Debug, Default)]
pub struct Popover {
    pub add_task: AddTask,
}

#[derive(Debug, Default)]
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
    pub task_state: TableState,
    pub project_state: ListState,
}
