use crate::{cli, controller::delegate_op, Result};

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{style::Stylize, text::Line, DefaultTerminal, Frame};
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::Interval;

#[derive(Debug)]
pub struct App {
    db: SqlitePool,
    running: RunningState,
    event_stream: EventStream,
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
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(db: SqlitePool) -> Self {
        Self {
            db,
            running: Default::default(),
            event_stream: Default::default(),
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
        let title = Line::from("Twodo CLI App").bold().green().centered();
        frame.render_widget(title, frame.area());
    }

    async fn update(&mut self, action: Message) -> Result<Message> {
        match action {
            Message::Quit => self.quit(),
            Message::Op(op) => delegate_op(&self.db, op).await,
            Message::Noop => todo!(),
        }
    }

}
