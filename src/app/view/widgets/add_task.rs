use crate::app::{
    model::AddTask,
    view::support::{centered_area, focus_textarea},
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Buffer, Stylize},
    style::Style,
    text::Line,
    widgets::{Block, Clear, Widget},
};
use tui_textarea::TextArea;

impl Widget for &AddTask {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_area(area, 50, 40);

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)]);
        let [title_layout, description_layout] = popup_layout.areas(popup_area);
        Widget::render(Clear, popup_area, buf);
        Widget::render(&self.title, title_layout, buf);
        Widget::render(&self.description, description_layout, buf);
    }
}

impl Default for AddTask {
    fn default() -> Self {
        let mut title = TextArea::default();
        title.set_cursor_style(Style::default());
        title.set_cursor_line_style(Style::default());
        title.set_block(Block::bordered().title_top(Line::from(" Title ").centered().bold()));
        focus_textarea(&mut title);

        let mut description = TextArea::default();
        description.set_cursor_style(Style::default());
        description.set_cursor_line_style(Style::default());
        description
            .set_block(Block::bordered().title_top(Line::from(" Description ").centered().bold()));

        Self { title, description }
    }
}

impl AddTask {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
