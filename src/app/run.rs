use crate::{
    app::{
        model::{App, RunningState},
        update::{message::Message, read_data::get_tasks},
    },
    Result,
};

use ratatui::DefaultTerminal;
use sqlx::SqlitePool;
use std::time::Duration;

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn new(db: SqlitePool) -> Result<Self> {
        let tasks = get_tasks(&db).await?;
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

        while self.running != RunningState::Done {
            terminal.draw(|f| self.view(f))?;

            let mut action = self.handle_event(&mut interval).await;
            while action != Message::Noop {
                action = self.update(action).await?;
            }
        }

        Ok(())
    }
}
