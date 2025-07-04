use crate::app::model::App;
use ratatui::{
    layout::Rect,
    prelude::Buffer,
    style::{Style, Stylize},
    text::Line,
    widgets::{block::Position, Block, Borders, List, ListItem, StatefulWidget},
};

impl App {
    pub(super) fn render_tasks(&mut self, frame: Rect, buf: &mut Buffer) {
        let task_block = Block::new()
            .title(Line::from(" Tasks ").centered().style(Style::new().bold()))
            .borders(Borders::ALL)
            .title_position(Position::Top);

        let rows = self
            .twodo
            .tasks
            .iter()
            .map(|t| {
                let done = if t.done { "󰄳 " } else { "󰄰 " };
                ListItem::new(format!("{} {}", done, t.title))
            })
            .collect::<Vec<_>>();

        let table = List::new(rows)
            .block(task_block)
            .highlight_style(Style::new().green())
            .highlight_symbol("󰜴 ");

        StatefulWidget::render(table, frame, buf, &mut self.state.task_state);
    }
}
