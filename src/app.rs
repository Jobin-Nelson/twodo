use crate::Result;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{style::Stylize, text::Line, DefaultTerminal, Frame};
use std::time::Duration;
use tokio::time::Interval;

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    event_stream: EventStream,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while self.running {
            terminal.draw(|f| self.render(f));

            tokio::select! {
                event = self.event_stream.next().fuse() =>{
                    if let Some(Ok(evt)) = event {
                        match evt {
                            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                            Event::Mouse(_) => {}
                            Event::Resize(_, _) => {}
                            _ => {}
                        }
                    }
                },
                // Sleep for a short duration to avoid busy waiting
                _ = interval.tick() => {}
            }
        }

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Twodo CLI App").bold().green().centered();
        frame.render_widget(title, frame.area());
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Quit on Ctrl-C or ESC or q
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),

            // Other key handlers
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
