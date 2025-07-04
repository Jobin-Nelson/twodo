use crate::{
    app::{
        model::{App, AppMode},
        update::{message::Message, read_data::get_twodo},
    },
    Result,
};

use ratatui::DefaultTerminal;
use sqlx::SqlitePool;
use std::time::Duration;

impl App {
    const FRAMES_PER_SECOND: f32 = 30.0;

    pub async fn new(db: SqlitePool) -> Result<Self> {
        let twodo = get_twodo(&db).await?;
        Ok(Self {
            db,
            mode: Default::default(),
            event_stream: Default::default(),
            twodo,
            state: Default::default(),
            popover: Default::default(),
        })
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.state.task_state.select_first();
        self.state.project_state.select_first();

        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);

        while self.mode.app_mode != AppMode::Quit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            let mut action = self.handle_event(&mut interval).await;
            while action != Message::Noop {
                action = self.update(action).await?;
            }
        }

        Ok(())
    }
}
