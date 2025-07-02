use crate::objects::Task;

use crossterm::event::EventStream;
use ratatui::widgets::TableState;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct App {
    pub db: SqlitePool,
    pub running: RunningState,
    pub event_stream: EventStream,
    pub twodo: Twodo,
    pub state: State,
}

#[derive(Debug, PartialEq, Default)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub struct Twodo {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Default)]
pub struct State {
    pub task_state: TableState,
}
