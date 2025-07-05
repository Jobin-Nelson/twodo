use crate::app::{
    model::{AddTaskMode, App, AppMode},
    update::message::Message,
};

use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
        match self.mode.app_mode {
            AppMode::FocusTask => on_focus_task_key_event(key),
            AppMode::FocusProject => on_focus_project_task_key_event(key),
            AppMode::AddTask => self.on_add_task_key_event(key),
            AppMode::Quit => unreachable!(),
        }
    }

    fn on_add_task_key_event(&mut self, key: KeyEvent) -> Message {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Message::AddTaskAbort
            }
            (KeyModifiers::CONTROL, KeyCode::Char(' ')) => Message::AddTaskCommit,

            (_, KeyCode::Tab) => match self.mode.add_task_mode {
                AddTaskMode::AddTitle => Message::FocusAddTaskDescription,
                AddTaskMode::AddDescription => Message::FocusAddTaskTitle,
            },

            _ => {
                match self.mode.add_task_mode {
                    AddTaskMode::AddTitle => self.popover.add_task.title.input(key),
                    AddTaskMode::AddDescription => self.popover.add_task.description.input(key),
                };
                Message::Noop
            }
        }
    }
}

fn on_focus_task_key_event(key: KeyEvent) -> Message {
    match (key.modifiers, key.code) {
        // Task navigation
        (_, KeyCode::Char('j')) => Message::SelectNextTask,
        (_, KeyCode::Char('k')) => Message::SelectPrevTask,
        (_, KeyCode::Char('i')) | (_, KeyCode::Char('a')) => Message::AddTaskBegin,
        (_, KeyCode::Char('r')) => Message::ReloadTask,
        (_, KeyCode::Char('x')) => Message::DeleteTask,
        (_, KeyCode::Char(' ')) => Message::ToggleTaskStatus,
        (_, KeyCode::Char('g')) => Message::SelectFirstTask,
        (_, KeyCode::Char('G')) => Message::SelectLastTask,

        (_, KeyCode::Tab) => Message::FocusProject,

        // Other key handlers
        _ => on_global_key_event(key),
    }
}

fn on_focus_project_task_key_event(key: KeyEvent) -> Message {
    match (key.modifiers, key.code) {
        // Task navigation
        (_, KeyCode::Char('j')) => Message::SelectNextProject,
        (_, KeyCode::Char('k')) => Message::SelectPrevProject,
        (_, KeyCode::Char('g')) => Message::SelectFirstProject,
        (_, KeyCode::Char('G')) => Message::SelectLastProject,
        (_, KeyCode::Tab) => Message::FocusTask,

        // Other key handlers
        _ => on_global_key_event(key),
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
