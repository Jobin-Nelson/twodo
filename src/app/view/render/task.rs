use crate::{app::model::App, constants::{T_COMPLETED, T_OPEN}};
use ratatui::{
    layout::Rect,
    prelude::Buffer,
    style::{Style, Stylize},
    text::Line,
    widgets::{block::Position, Block, BorderType, Borders, List, ListItem, StatefulWidget},
};

impl App {
    pub(super) fn render_tasks(&mut self, frame: Rect, buf: &mut Buffer) {
        let task_block = Block::new()
            .title(Line::from(" Tasks ").centered().style(Style::new().bold()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_position(Position::Top);

        let rows = self
            .twodo
            .tasknodes
            .iter()
            .map(|t| {
                let status = match t.status {
                    crate::objects::TaskStatus::Open => T_OPEN,
                    crate::objects::TaskStatus::Completed => T_COMPLETED,
                };
                let depth = "  ".repeat(t.level as usize);
                ListItem::new(format!("{} {} {}", depth, status, t.title))
            })
            .collect::<Vec<_>>();

        let table = List::new(rows)
            .block(task_block)
            .highlight_style(Style::new().green())
            .highlight_symbol("󰜴 ");

        StatefulWidget::render(table, frame, buf, &mut self.state.task_state);
    }
}
