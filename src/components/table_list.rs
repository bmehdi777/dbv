use super::{centered_rect, MutableComponent};
use crate::{
    application::Store,
    components::LayoutArea,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone)]
pub struct TableItem {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct TableListComponent {
    list_state: ListState,
    selected: isize,
}

impl TableListComponent {
    pub fn new() -> Self {
        TableListComponent {
            list_state: ListState::default(),
            selected: -1,
        }
    }
}

impl MutableComponent for TableListComponent {
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        if store.tables_list.len() > 0 {
            match input {
                Keys::Char('j') => {
                    if let Some(i) = self.list_state.selected() {
                        let index = if i == store.tables_list.len() - 1 {
                            0
                        } else {
                            i + 1
                        };

                        self.list_state.select(Some(index));
                    } else {
                        self.list_state.select(Some(0));
                    }
                }
                Keys::Char('k') => {
                    if let Some(i) = self.list_state.selected() {
                        let index = if i == 0 {
                            store.tables_list.len() - 1
                        } else {
                            i - 1
                        };
                        self.list_state.select(Some(index));
                    } else {
                        self.list_state.select(Some(store.tables_list.len() - 1));
                    }
                }
                _ => return Ok(EventState::Wasted),
            }
        }
        Ok(EventState::Consumed)
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        selected: bool,
        store: &Store,
        _layout: &LayoutArea,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Tables")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.preference.theme_config)),
            )
            .border_type(BorderType::Rounded);

        if store.tables_list.len() > 0 {
            let selected_idx = if let Some(index) = self.list_state.selected() {
                index + 1
            } else {
                0
            };
            let list = List::new(store.tables_list.iter().enumerate().map(|(index, item)| {
                if self.selected == index as isize {
                    format!(" * {}", item.clone())
                } else {
                    item.clone()
                }
            }))
            .style(
                Style::default().fg(self.get_color(store.preference.theme_config.unselected_color)),
            )
            .block(container.title_bottom(format!(
                "{} of {}",
                selected_idx,
                store.tables_list.len()
            )))
            .highlight_style(Style::default().reversed())
            .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, area, &mut self.list_state);
        } else {
            let no_data = Paragraph::new("No tables")
                .style(Style::new().italic())
                .centered();

            frame.render_widget(container, area);
            frame.render_widget(no_data, centered_rect(area, 50, 20));
        }
        Ok(())
    }
}
