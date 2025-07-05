use crate::app::{
    model::{AddTaskMode, App, AppMode},
    view::support::{focus_textarea, unfocus_textarea},
};
use ratatui::{layout::Rect, prelude::Buffer, widgets::Widget};

impl App {
    pub(super) fn render_popup(&mut self, area: Rect, buf: &mut Buffer) {
        match self.mode.app_mode {
            AppMode::AddTask | AppMode::AddSubTask | AppMode::AddSiblingTask => self.render_add_task(area, buf),
            _ => {}
        }
    }

    pub(super) fn render_add_task(&mut self, area: Rect, buf: &mut Buffer) {
        match self.mode.add_task_mode {
            AddTaskMode::AddTitle => {
                focus_textarea(&mut self.popover.add_task.title);
                unfocus_textarea(&mut self.popover.add_task.description);
            }
            AddTaskMode::AddDescription => {
                focus_textarea(&mut self.popover.add_task.description);
                unfocus_textarea(&mut self.popover.add_task.title);
            }
        }
        self.popover.add_task.render(area, buf);
    }
}
