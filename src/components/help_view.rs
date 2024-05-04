use super::MutableComponent;
use crate::{
    application::{Store, StoreAction},
    components::LayoutArea,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

pub trait HelpContentText {
    fn help_content_text() -> HashMap<&'static str, &'static str>;
}

pub struct HelpViewComponent {
    pub id: u32,
    content: HashMap<&'static str, &'static str>,
    title: String,
    scrollstate: ScrollbarState,
    tablestate: TableState,
}

impl HelpViewComponent {
    pub fn new(
        id: u32,
        title: String,
        content: Option<HashMap<&'static str, &'static str>>,
    ) -> HelpViewComponent {
        if let Some(hc) = content {
            return HelpViewComponent {
                id,
                content: hc.clone(),
                title,
                scrollstate: ScrollbarState::new(hc.len()).position(0),
                tablestate: TableState::default(),
            };
        }
        HelpViewComponent {
            id,
            content: HashMap::new(),
            title,
            scrollstate: ScrollbarState::new(0).position(0),
            tablestate: TableState::default(),
        }
    }
}

impl MutableComponent for HelpViewComponent {
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('j') => {
                if let Some(i) = self.tablestate.selected() {
                    let index = if i == self.content.len() - 1 {
                        0
                    } else {
                        i + 1
                    };

                    self.tablestate.select(Some(index));
                    self.scrollstate.next();
                } else {
                    self.tablestate.select(Some(0));
                    self.scrollstate = self.scrollstate.position(0);
                }
            }
            Keys::Char('k') => {
                if let Some(i) = self.tablestate.selected() {
                    let index = if i == 0 {
                        self.content.len() - 1
                    } else {
                        i - 1
                    };
                    self.tablestate.select(Some(index));
                    self.scrollstate.prev();
                } else {
                    let max = self.content.len() - 1;
                    self.tablestate.select(Some(max));
                    self.scrollstate = self.scrollstate.position(max);
                }
            }
            Keys::Enter => {
                if let Some(i) = self.tablestate.selected() {
                    store.selected_pane = store.previous_selected_pane;
                    let key_str = *self.content.keys().collect::<Vec<_>>()[i];
                    let key = Keys::Char(key_str.chars().nth(0).unwrap());
                    store
                        .event_handler
                        .send_key(key)
                        .expect("An error occured while sending keys to thread.");
                }
            }
            _ => return Ok(EventState::Wasted),
        }
        Ok(EventState::Consumed)
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
            .title(format!("Help - {}", self.title))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.preference.theme_config)),
            );

        let table = Table::new(
            self.content
                .iter()
                .map(|(key, value)| {
                    Row::new(vec![
                        Cell::from(*key).style(
                            Style::default()
                                .fg(self.get_color(store.preference.theme_config.help_key_color)),
                        ),
                        Cell::from(*value).style(
                            Style::default()
                                .fg(self.get_color(store.preference.theme_config.help_desc_color))
                                .italic(),
                        ),
                    ])
                })
                .collect::<Vec<Row>>(),
            [Constraint::Length(4), Constraint::Fill(1)],
        )
        .block(container)
        .highlight_style(Style::default().reversed());

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(table, area, &mut self.tablestate);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.scrollstate,
        );

        Ok(())
    }
}
