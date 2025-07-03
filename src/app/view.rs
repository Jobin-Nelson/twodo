use crate::app::model::App;
use ratatui::{
    Frame,
    layout::Constraint,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Row, Table, block::Position},
};

impl App {
    pub fn view(&mut self, frame: &mut Frame) {
        self.render_tasks(frame);
    }

    pub fn render_tasks(&mut self, frame: &mut Frame) {
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

        frame.render_stateful_widget(table, frame.area(), &mut self.state.task_state);
    }
}
