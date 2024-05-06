use super::{centered_rect, MutableComponent};
use crate::{
    application::Store,
    components::LayoutArea,
    events::{key::Keys, EventState},
    sql::parser::SqlParser,
};
use ratatui::{prelude::*, widgets::*};
use sqlx::{any::AnyRow, Row as SqlRow};

pub struct RecordsViewComponent<'a> {
    header: Row<'a>,
    rows: Vec<Row<'a>>,
    total: Option<i64>,

    table_state: TableState,

    scrollbar_state_right: ScrollbarState,
    scrollbar_state_bottom: ScrollbarState,
}

impl<'a> RecordsViewComponent<'a> {
    pub fn new() -> Self {
        RecordsViewComponent {
            header: Row::default(),
            rows: Vec::new(),
            total: None,
            table_state: TableState::default(),
            scrollbar_state_right: ScrollbarState::default(),
            scrollbar_state_bottom: ScrollbarState::default(),
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
        self.scrollbar_state_bottom = self.scrollbar_state_bottom.content_length(header.len());
    }

    pub fn set_body(&mut self, content: Vec<AnyRow>, _store: &Store) {
        let mut rows: Vec<Row> = Vec::new();
        for r in content.iter() {
            rows.push(Row::new(
                r.columns()
                    .iter()
                    .map(|col| {
                        return SqlParser::convert_from_sqlx_row(&col, &r);
                    })
                    .collect::<Vec<_>>(),
            ))
        }
        self.rows = rows;
        self.scrollbar_state_right = self.scrollbar_state_right
            .content_length(content.first().unwrap().len());
    }

    pub fn set_total(&mut self, total: Option<i64>) {
        self.total = total;
    }
}

impl<'a> MutableComponent for RecordsViewComponent<'a> {
    fn event(&mut self, input: &Keys, _store: &mut Store) -> anyhow::Result<EventState> {
        let rows_len = self.rows.len();
        if rows_len > 0 {
            match input {
                Keys::Char('j') => {
                    if let Some(i) = self.table_state.selected() {
                        let index = if i == rows_len - 1 {
                            self.scrollbar_state_right.first();
                            0
                        } else {
                            self.scrollbar_state_right.next();
                            i + 1
                        };

                        self.table_state.select(Some(index));
                    } else {
                        self.scrollbar_state_right.first();
                        self.table_state.select(Some(0));
                    }
                }
                Keys::Char('k') => {
                    if let Some(i) = self.table_state.selected() {
                        let index = if i == 0 {
                            self.scrollbar_state_right.last();
                            rows_len - 1
                        } else {
                            self.scrollbar_state_right.prev();
                            i - 1
                        };
                        self.table_state.select(Some(index));
                    } else {
                        self.scrollbar_state_right.last();
                        self.table_state.select(Some(rows_len - 1));
                    }
                }
                Keys::Char('l') => {}
                _ => return Ok(EventState::Wasted),
            }
            return Ok(EventState::Consumed);
        } else {
            return Ok(EventState::Wasted);
        }
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        store: &Store,
        _layout: &LayoutArea,
    ) -> anyhow::Result<()> {
        let mut container = Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.preference.theme_config)),
            )
            .border_type(BorderType::Rounded);

        let width = vec![Constraint::Fill(1); 4];

        if self.rows.len() > 0 {
            if let Some(total) = self.total {
                let selected = if let Some(i) = self.table_state.selected() {
                    i + 1
                } else {
                    0
                };
                container = container
                    .title_bottom(Line::from(format!("Total : {}", total)).right_aligned())
                    .title_bottom(Line::from(format!("{} of {}", selected, self.rows.len())));
            }
            let table = Table::new(self.rows.clone(), width)
                .block(container)
                .highlight_style(Style::default().reversed())
                .header(self.header.clone());

            self.scrollbar_state_right = self.scrollbar_state_right.viewport_content_length(frame.size().height.into());
            let scrollbar_right = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));

            self.scrollbar_state_bottom = self.scrollbar_state_bottom.viewport_content_length(frame.size().width.into());
            let scrollbar_bottom = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some("⯇"))
                .end_symbol(Some("⯈"));



            frame.render_stateful_widget(table, area, &mut self.table_state);

            frame.render_stateful_widget(
                scrollbar_right,
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut self.scrollbar_state_right,
            );
            frame.render_stateful_widget(
                scrollbar_bottom,
                area.inner(&Margin {
                    vertical: 0,
                    horizontal: 20,
                }),
                &mut self.scrollbar_state_bottom,
            );
        } else {
            let no_data = Paragraph::new("No records").style(Style::new().italic());

            frame.render_widget(container, area);
            frame.render_widget(no_data, centered_rect(area, 11, 20));
        }
        Ok(())
    }
}
