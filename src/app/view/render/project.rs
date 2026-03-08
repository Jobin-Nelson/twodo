use crate::app::model::App;
use ratatui::{
    layout::Rect,
    prelude::Buffer,
    style::{Style, Stylize},
    text::Line,
    widgets::{
        block::Position, Block, BorderType, Borders, List, ListItem, StatefulWidget
    },
};

impl App {
    pub(super) fn render_projects(&mut self, frame: Rect, buf: &mut Buffer) {
        let project_block = Block::new()
            .title(
                Line::from(" Projects ")
                    .centered()
                    .style(Style::new().bold()),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_position(Position::Top);

        let items = self
            .twodo
            .projects
            .iter()
            .map(|p| ListItem::from(p.name.clone()))
            .collect::<Vec<_>>();

        let list = List::new(items)
            .block(project_block)
            .highlight_style(Style::new().green())
            .highlight_symbol("󰜴 ");

        StatefulWidget::render(list, frame, buf, &mut self.state.project_state);
    }

}
