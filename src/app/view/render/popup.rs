use crate::app::model::{App, AppState};
use ratatui::{layout::Rect, prelude::Buffer, widgets::Widget};

impl App {
    pub(super) fn render_popup(&mut self, area: Rect, buf: &mut Buffer) {
        match self.app_state {
            AppState::AddTask => self.popover.add_task.render(area, buf),
            _ => {}
        }
    }
}
