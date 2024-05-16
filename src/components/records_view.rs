use super::{centered_rect, MutableComponent};
use crate::{
    application::Store,
    components::{
        widgets::custom_table::{CustomTable, CustomTableState},
        LayoutArea,
    },
    events::{key::Keys, EventState},
    sql::parser::SqlParser,
};
use ratatui::{prelude::*, widgets::*};
use sqlx::{any::AnyRow, Row as SqlRow};

pub struct RecordsViewComponent {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    total: Option<i64>,

    table_state: CustomTableState,

    scrollbar_state_right: ScrollbarState,
}

impl<'a> RecordsViewComponent {
    pub fn new() -> Self {
        RecordsViewComponent {
            header: Vec::new(),
            rows: Vec::new(),
            total: None,
            table_state: CustomTableState::default(),
            scrollbar_state_right: ScrollbarState::default(),
        }
    }

    pub fn set_header(&mut self, header: Vec<String>) {
        self.table_state = self.table_state.header_length(header.len());
        self.header = header;
    }

    pub fn set_body(&mut self, content: Vec<AnyRow>) {
        let mut rows: Vec<Vec<String>> = Vec::new();
        for r in content.iter() {
            rows.push(
                r.columns()
                    .iter()
                    .map(|col| {
                        return SqlParser::convert_from_sqlx_row(&col, &r);
                    })
                    .collect::<Vec<String>>(),
            )
        }
        let content_length = content.len();
        self.rows = rows;
        self.scrollbar_state_right = self.scrollbar_state_right.content_length(content_length);
        self.table_state = self.table_state.content_length(content_length);
    }

    pub fn set_total(&mut self, total: Option<i64>) {
        self.total = total;
    }
}

impl<'a> MutableComponent for RecordsViewComponent {
    fn event(&mut self, input: &Keys, _store: &mut Store) -> anyhow::Result<EventState> {
        let rows_len = self.rows.len();
        if rows_len > 0 {
            match input {
                Keys::Char('h') => {
                    self.table_state.prev_col();
                }
                Keys::Char('l') => {
                    self.table_state.next_col();
                }

                Keys::Char('j') => {
                    if let Some((_x, y)) = self.table_state.selected() {
                        if y == rows_len - 1 {
                            self.scrollbar_state_right.first();
                            0
                        } else {
                            self.scrollbar_state_right.next();
                            y + 1
                        };
                    } else {
                        self.scrollbar_state_right.first();
                    }
                    self.table_state.next_row();
                }
                Keys::Char('k') => {
                    if let Some((_x, y)) = self.table_state.selected() {
                        if y == 0 {
                            self.scrollbar_state_right.last();
                            rows_len - 1
                        } else {
                            self.scrollbar_state_right.prev();
                            y - 1
                        };
                    } else {
                        self.scrollbar_state_right.last();
                    }
                    self.table_state.prev_row();
                }
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

        if self.rows.len() > 0 {
            if let Some(total) = self.total {
                let selected = if let Some((_, y)) = self.table_state.selected() {
                    self.table_state.offset_y + y
                } else {
                    0
                };
                container = container
                    .title_bottom(Line::from(format!("Total : {}", total)).right_aligned())
                    .title_bottom(Line::from(format!("{} of {}", selected, self.rows.len())));
            }

            self.scrollbar_state_right = self
                .scrollbar_state_right
                .viewport_content_length(frame.size().height.into());
            let scrollbar_right = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));

            let content_style =
                Style::default().fg(self.get_color(store.preference.theme_config.unselected_color));

            let table = CustomTable::new()
                .block(container)
                .header_block_style(
                    Style::default().fg(self.selected_color(true, store.preference.theme_config)),
                )
                .header(self.header.clone())
                .header_style(content_style)
                .rows_style(content_style)
                .highlight_style(content_style.reversed())
                .rows(self.rows.clone());

            frame.render_stateful_widget(table, area, &mut self.table_state);

            frame.render_stateful_widget(
                scrollbar_right,
                area.inner(&Margin {
                    vertical: 2,
                    horizontal: 0,
                }),
                &mut self.scrollbar_state_right,
            );
        } else {
            let no_data = Paragraph::new("No records").style(Style::new().italic());

            frame.render_widget(container, area);
            frame.render_widget(no_data, centered_rect(area, 11, 20));
        }
        Ok(())
    }
}
