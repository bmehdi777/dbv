use super::{centered_rect, MutableComponent};
use crate::{
    application::Store,
    events::{key::Keys, EventState},
    sql::tables::Tables,
};

use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone)]
pub struct DatabaseListComponent {
    list_state: ListState,
    selected: isize,
}

impl DatabaseListComponent {
    pub fn new() -> Self {
        DatabaseListComponent {
            list_state: ListState::default(),
            selected: -1,
        }
    }
}

impl MutableComponent for DatabaseListComponent {
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        if store.database_list.len() > 0 {
            match input {
                Keys::Char('j') => {
                    if let Some(i) = self.list_state.selected() {
                        let index = if i == store.database_list.len() - 1 {
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
                            store.database_list.len() - 1
                        } else {
                            i - 1
                        };
                        self.list_state.select(Some(index));
                    } else {
                        self.list_state.select(Some(store.database_list.len() - 1));
                    }
                }
                Keys::Enter => {
                    if let Some(index) = self.list_state.selected() {
                        let current_db = store.database_list[index].clone();
                        let pool = store.connection_list.get_pool().unwrap();
                        let actions_tx = store.actions_tx.clone();

                        Tables::get_tables(pool, actions_tx, current_db);
                        self.selected = index as isize;
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
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Databases")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.config.theme_config)),
            )
            .border_type(BorderType::Rounded);

        if store.database_list.len() > 0 {
            let selected_idx = if let Some(index) = self.list_state.selected() {
                index + 1
            } else {
                0
            };

            let list = List::new(store.database_list.iter().enumerate().map(|(index, item)| {
                if self.selected == index as isize {
                    format!(" * {}", item.clone())
                } else {
                    item.clone()
                }
            }))
            .style(Style::default().fg(self.get_color(store.config.theme_config.unselected_color)))
            .block(container.title_bottom(format!(
                "{} of {}",
                selected_idx,
                store.database_list.len()
            )))
            .highlight_style(Style::default().reversed())
            .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, area, &mut self.list_state);
        } else {
            let no_data = Paragraph::new("No databases")
                .style(Style::new().italic())
                .centered();

            frame.render_widget(container.title_bottom("0 of 0"), area);
            frame.render_widget(no_data, centered_rect(area, 50, 20));
        }

        Ok(())
    }
}
