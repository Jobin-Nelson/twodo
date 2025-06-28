use crate::{cli, controller::delegate_op, objects::Task, Result};

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    text::Line,
    widgets::{block::Position, Block, Borders, Row, Table, TableState},
    DefaultTerminal, Frame,
};
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::Interval;

#[derive(Debug)]
pub struct App {
    db: SqlitePool,
    running: RunningState,
    event_stream: EventStream,
    pub tasks: Vec<Task>,
    pub task_state: TableState,
}

#[derive(Debug, PartialEq, Default)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Op(cli::Op),
    Noop,
    NextTask,
    PrevTask,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn new(db: SqlitePool) -> Result<Self> {
        let tasks = get_todo_tasks(&db).await?;
        Ok(Self {
            db,
            running: Default::default(),
            event_stream: Default::default(),
            tasks,
            task_state: Default::default(),
        })
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.task_state.select_first();

        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while self.running != RunningState::Done {
            terminal.draw(|f| self.view(f));

            let mut action = self.handle_event(&mut interval).await;
            while action != Message::Noop {
                action = self.update(action).await?;
            }
        }

        Ok(())
    }

    pub async fn handle_event(&mut self, interval: &mut Interval) -> Message {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                if let Some(Ok(evt)) = event {
                    match evt {
                        Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                        Event::Mouse(_) => Message::Noop,
                        Event::Resize(_, _) => Message::Noop,
                        _ => Message::Noop,
                    }
                } else {
                    Message::Noop
                }
            },
            // Sleep for a short duration to avoid busy waiting
            _ = interval.tick() => Message::Noop,
        }
    }

    fn on_key_event(&mut self, key: KeyEvent) -> Message {
        match (key.modifiers, key.code) {
            // Quit on Ctrl-C or ESC or q
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => Message::Quit,

            // Task navigation
            (_, KeyCode::Char('j')) => Message::NextTask,
            (_, KeyCode::Char('k')) => Message::PrevTask,

            // Other key handlers
            _ => Message::Noop,
        }
    }

    fn quit(&mut self) -> Result<Message> {
        self.running = RunningState::Done;
        Ok(Message::Noop)
    }

    // ELM Architecture
    pub fn view(&mut self, frame: &mut Frame) {
        self.render_tasks(frame);
    }

    pub fn render_tasks(&mut self, frame: &mut Frame) {
        let header = Row::new(["ID", "Title", "Description"]).bold().dark_gray();

        let task_block = Block::new()
            .title(Line::from(" Tasks ").centered().style(Style::new().bold()))
            .borders(Borders::ALL)
            .title_position(Position::Top);

        let rows = self
            .tasks
            .iter()
            .map(|t| Row::new([t.id.to_string(), t.title.clone(), t.description.clone()]))
            .collect::<Vec<_>>();

        let widths = [
            Constraint::Length(3),
            Constraint::Percentage(60),
            Constraint::Fill(1),
        ];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(header)
            .block(task_block)
            .row_highlight_style(Style::new().green())
            .highlight_symbol("󰜴 ");

        frame.render_stateful_widget(table, frame.area(), &mut self.task_state);
    }

    async fn update(&mut self, action: Message) -> Result<Message> {
        match action {
            Message::Quit => self.quit(),
            Message::Op(op) => delegate_op(&self.db, op).await,
            Message::NextTask => {
                self.task_state.select_next();
                Ok(Message::Noop)
            }
            Message::PrevTask => {
                self.task_state.select_previous();
                Ok(Message::Noop)
            }

            // Update will never be called with Noop
            Message::Noop => unreachable!(),
        }
    }
}

async fn get_todo_tasks(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<Task>> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks where done = 0")
        .fetch_all(db)
        .await
        .map_err(Into::into)
}
