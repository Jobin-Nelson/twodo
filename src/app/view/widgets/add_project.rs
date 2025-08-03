use crate::app::{
    model::AddProject,
    view::support::{centered_area, focus_textarea},
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Buffer, Stylize},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Clear, Widget},
};
use tui_textarea::TextArea;

impl Widget for &AddProject {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_area(area, 50, 40);

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3)]);
        let [name_layout] = popup_layout.areas(popup_area);
        Widget::render(Clear, popup_area, buf);
        Widget::render(&self.name, name_layout, buf);
    }
}

impl Default for AddProject {
    fn default() -> Self {
        let mut name = TextArea::default();
        name.set_cursor_style(Style::default());
        name.set_cursor_line_style(Style::default());
        name.set_block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(Line::from(" Name ").centered().bold()),
        );
        focus_textarea(&mut name);

        Self { name }
    }
}

impl AddProject {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
