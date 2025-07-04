use crate::app::{
    model::{App, AppState},
    update::message::Message,
};

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

    fn on_key_event(&self, key: KeyEvent) -> Message {
        match self.app_state {
            AppState::NormalTask => on_normal_task_key_event(key),
            AppState::NormalProject => on_normal_project_task_key_event(key),
            AppState::AddTask => on_add_task_key_event(key),
            AppState::CloseApp => unreachable!(),
        }
    }
}

fn on_normal_task_key_event(key: KeyEvent) -> Message {
    match (key.modifiers, key.code) {
        // Task navigation
        (_, KeyCode::Char('j')) => Message::NextTask,
        (_, KeyCode::Char('k')) => Message::PrevTask,
        (_, KeyCode::Char('i')) | (_, KeyCode::Char('a')) => Message::AddTaskBegin,

        (_, KeyCode::Tab) => Message::FocusProject,

        // Other key handlers
        _ => on_global_key_event(key),
    }
}

fn on_normal_project_task_key_event(key: KeyEvent) -> Message {
    match (key.modifiers, key.code) {
        // Task navigation
        (_, KeyCode::Char('j')) => Message::NextProject,
        (_, KeyCode::Char('k')) => Message::PrevProject,
        (_, KeyCode::Tab) => Message::FocusTask,

        // Other key handlers
        _ => on_global_key_event(key),
    }
}

fn on_add_task_key_event(key: KeyEvent) -> Message {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc) | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
            Message::AddTaskAbort
        },
        (KeyModifiers::CONTROL, KeyCode::Enter) => Message::AddTaskCommit,

        _ => Message::Noop,
    }
}

fn on_global_key_event(key: KeyEvent) -> Message {
    match (key.modifiers, key.code) {
        // Quit on Ctrl-C or ESC or q
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => Message::Quit,

        // Navigation
        (_, KeyCode::Char('1')) => Message::FocusProject,
        (_, KeyCode::Char('2')) => Message::FocusTask,
        _ => Message::Noop,
    }
}
