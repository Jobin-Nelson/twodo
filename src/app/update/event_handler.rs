use crate::app::{model::App, update::message::Message};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use tokio::time::Interval;

impl App {
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
}
