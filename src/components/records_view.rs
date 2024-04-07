use super::{centered_rect, MutableComponent};
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

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
        let rows: Vec<Row<'a>> = vec![];
        let table_state = TableState::default();
        RecordsViewComponent {
            header,
            rows,
            table_state,
        }
    }
}

impl<'a> MutableComponent for RecordsViewComponent<'a> {
    fn event(&mut self, _input: &Keys, _app_state: &AppState) -> anyhow::Result<EventState> {
        Ok(EventState::Wasted)
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        app_state: &AppState,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, app_state.config.theme_config)),
            )
            .border_type(BorderType::Rounded);

        let width = vec![Constraint::Fill(1); self.rows.len()];

        if self.rows.len() > 0 {
            let table = Table::new(self.rows.clone(), width)
                .block(container)
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .header(self.header.clone());

            frame.render_stateful_widget(table, area, &mut self.table_state);
        } else {
            let no_data = Paragraph::new("No records").style(Style::new().italic());

            frame.render_widget(container, area);
            frame.render_widget(no_data, centered_rect(area, 11, 1));
        }
        Ok(())
    }
}
