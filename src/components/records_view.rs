use super::{centered_rect, MutableComponent};
use crate::{
    application::Store,
    components::LayoutArea,
    events::{key::Keys, EventState},
};
use ratatui::{prelude::*, widgets::*};
use sqlx::{any::AnyRow, Column, Row as SqlRow};

pub struct RecordsViewComponent<'a> {
    header: Row<'a>,
    rows: Vec<Row<'a>>,

    table_state: TableState,
}

impl<'a> RecordsViewComponent<'a> {
    pub fn new() -> Self {
        let table_state = TableState::default();
        RecordsViewComponent {
            header: Row::default(),
            rows: Vec::new(),
            table_state,
        }
    }

    pub fn set_header(&mut self, header: Vec<String>, store: &Store) {
        let color = Color::Rgb(
            store.preference.theme_config.selected_color[0],
            store.preference.theme_config.selected_color[1],
            store.preference.theme_config.selected_color[2],
        );
        self.header = Row::new(
            header
                .iter()
                .map(|item| {
                    Cell::from(item.clone()).style(Style::default().fg(Color::White).bg(color))
                })
                .collect::<Vec<_>>(),
        );
    }

    pub fn set_body(&mut self, content: &'a Vec<AnyRow>, _store: &Store) {
        self.rows = content
            .iter()
            .map(|row| {
                Row::new(
                    row.columns
                        .iter()
                        .map(|col| Cell::from(col.name()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
    }
}

impl<'a> MutableComponent for RecordsViewComponent<'a> {
    fn event(&mut self, _input: &Keys, _store: &mut Store) -> anyhow::Result<EventState> {
        Ok(EventState::Wasted)
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        store: &Store,
        _layout: &LayoutArea,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.preference.theme_config)),
            )
            .border_type(BorderType::Rounded);

        let width = vec![Constraint::Fill(1); 2];

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
