use crate::app::model::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Buffer,
    style::{Style, Stylize},
    text::Line,
    widgets::{
        block::Position, Block, Borders, List, ListItem, Row, StatefulWidget, Table, Widget,
    },
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
    }
}

impl App {
    pub fn render_tasks(&mut self, frame: Rect, buf: &mut Buffer) {
        let header = Row::new(["ID", "Title", "Description"]).bold().dark_gray();

        let task_block = Block::new()
            .title(Line::from(" Tasks ").centered().style(Style::new().bold()))
            .borders(Borders::ALL)
            .title_position(Position::Top);

        let rows = self
            .twodo
            .tasks
            .iter()
            .map(|t| Row::new([t.id.to_string(), t.title.clone(), t.description.clone()]))
            .collect::<Vec<_>>();

        let widths = [
            Constraint::Length(3),
            Constraint::Percentage(60),
            Constraint::Fill(1),
        ];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(header)
            .block(task_block)
            .row_highlight_style(Style::new().green())
            .highlight_symbol("󰜴 ");

        StatefulWidget::render(table, frame, buf, &mut self.state.task_state);
    }

    pub fn render_projects(&mut self, frame: Rect, buf: &mut Buffer) {
        let project_block = Block::new()
            .title(
                Line::from(" Projects ")
                    .centered()
                    .style(Style::new().bold()),
            )
            .borders(Borders::ALL)
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
