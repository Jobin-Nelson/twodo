use crate::objects::Task;

use crossterm::event::EventStream;
use ratatui::widgets::TableState;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct App {
    pub db: SqlitePool,
    pub running: RunningState,
    pub event_stream: EventStream,
    pub tasks: Vec<Task>,
    pub task_state: TableState,
}

#[derive(Debug, PartialEq, Default)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
