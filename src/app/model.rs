use crate::objects::{Project, Task};

use crossterm::event::EventStream;
use ratatui::widgets::{ListState, TableState};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct App {
    pub db: SqlitePool,
    pub app_state: AppState,
    pub event_stream: EventStream,
    pub twodo: Twodo,
    pub state: State,
}

#[derive(Debug, PartialEq, Default)]
pub enum AppState {
    #[default]
    NormalTask,
    NormalProject,
    CloseApp,
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
