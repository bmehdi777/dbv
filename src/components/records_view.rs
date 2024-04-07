use super::MutableComponent;
use crate::events::{key::Keys, EventState};

use ratatui::{prelude::*, widgets::*};

pub struct RecordsViewComponent<'a> {
    header: Row<'a>,
    rows: Vec<Row<'a>>,

    table_state: TableState,
}

impl<'a> RecordsViewComponent<'a> {
    pub fn new() -> Self {
        let header: Row<'a> = Row::new(vec![
            Cell::from("Hello").style(Style::default().fg(Color::White).bg(Color::Blue)),
            Cell::from("World").style(Style::default().fg(Color::White).bg(Color::Blue)),
        ]);
        let rows: Vec<Row<'a>> = vec![
            Row::new(vec!["teeeeeeeeeeeeeeeeeest", "test"]),
            Row::new(vec!["teeeeeeeeeeeeeeeeeest", "test"]),
            Row::new(vec!["teeeeeeeeeeeeeeeeeest", "test"]),
        ];
        let table_state = TableState::default();
        RecordsViewComponent {
            header,
            rows,
            table_state,
        }
    }
}

impl<'a> MutableComponent for RecordsViewComponent<'a> {
    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        todo!()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, selected: bool) -> anyhow::Result<()> {
        let container = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.selected_color(selected)))
            .border_type(BorderType::Rounded);

        let width = vec![Constraint::Fill(1); self.rows.len()];

        let table = Table::new(self.rows.clone(), width)
            .block(container)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .header(self.header.clone());

        frame.render_stateful_widget(table, area, &mut self.table_state);
        Ok(())
    }
}
