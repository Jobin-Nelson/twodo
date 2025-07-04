use crate::app::model::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Buffer,
    widgets::Widget,
};

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [project_layout, task_layout] = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(20), Constraint::Percentage(80)],
        )
        .areas(area);

        self.render_tasks(task_layout, buf);
        self.render_projects(project_layout, buf);
        self.render_popup(area, buf);
    }
}
