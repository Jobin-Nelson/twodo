use crate::app::{model::AddTask, view::support::centered_area};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Buffer,
    style::Stylize,
    text::Line,
    widgets::{Block, Clear, Widget},
};

impl Widget for &AddTask {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = Block::bordered().title_top(Line::from(" Add Task ").centered().bold());
        let popup_area = centered_area(area, 50, 40);

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)]);
        let [title_layout, description_layout] = popup_layout.areas(popup_area);
        Widget::render(Clear, popup_area, buf);
        Widget::render(popup, popup_area, buf);
        Widget::render(&self.title, title_layout, buf);
        Widget::render(&self.description, description_layout, buf);
    }
}
