use super::{centered_rect, HelpContentText, MutableComponent};
use crate::{
    application::Store,
    events::{key::Keys, EventState},
};
use ratatui::{prelude::*, widgets::*};
use sqlx::Row;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConnectionListComponent {
    list_state: ListState,
    selected: isize,
}

impl ConnectionListComponent {
    pub fn new() -> Self {
        ConnectionListComponent {
            list_state: ListState::default(),
            selected: -1,
        }
    }
}

impl MutableComponent for ConnectionListComponent {
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('i') => {
                store.selected_pane = (101, 101);
            }
            _ => {}
        }
        if store.connection_list.list.len() > 0 {
            match input {
                Keys::Char('j') => {
                    if let Some(i) = self.list_state.selected() {
                        let index = if i == store.connection_list.list.len() - 1 {
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
                            store.connection_list.list.len() - 1
                        } else {
                            i - 1
                        };
                        self.list_state.select(Some(index));
                    } else {
                        self.list_state
                            .select(Some(store.connection_list.list.len() - 1));
                    }
                }
                Keys::Char('d') => {
                    if let Some(index) = self.list_state.selected() {
                        store.connection_list.list.remove(index);
                        store.log("Connection string removed.");
                    }
                }
                Keys::Char('e') => {
                    store.selected_pane = (101, 101);
                }
                Keys::Enter => {
                    if let Some(index) = self.list_state.selected() {
                        let connection = &mut store.connection_list.list[index];
                        if let Err(e) = connection.set_pool() {
                            store.error(&format!("{}", e));
                            return Ok(EventState::Wasted);
                        }

                        let pool = connection.pool.as_ref().unwrap().clone();
                        store.current_connection = Some(index);
                        let actions_tx = store.actions_tx.clone();
                        tokio::spawn(async move {
                            let rows = sqlx::query("SHOW databases")
                                .fetch_all(&pool)
                                .await
                                .unwrap();
                            actions_tx.send(crate::application::StoreAction::SendDatabaseData(
                                rows.iter()
                                    .map(|row| row.try_get("Database").unwrap())
                                    .collect(),
                            )).unwrap();
                        });
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
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        store: &Store,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Connections")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.config.theme_config)),
            )
            .border_type(BorderType::Rounded);

        if store.connection_list.list.len() > 0 {
            let selected_idx = if let Some(index) = self.list_state.selected() {
                index + 1
            } else {
                0
            };

            let list = List::new(store.connection_list.list.iter().enumerate().map(
                |(index, item)| {
                    if self.selected == index as isize {
                        format!(" > {}", item.connection_string.clone())
                    } else {
                        item.connection_string.clone()
                    }
                },
            ))
            .block(container.title_bottom(format!(
                "{} of {}",
                selected_idx,
                store.connection_list.list.len()
            )))
            .style(
                Style::default().fg(self.get_color(store.config.theme_config.unselected_color)),
            )
            .highlight_style(Style::default().reversed())
            .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, area, &mut self.list_state);
        } else {
            let no_data = Paragraph::new("No connections registered")
                .style(Style::new().italic())
                .centered();

            frame.render_widget(container.title_bottom("0 of 0"), area);
            frame.render_widget(no_data, centered_rect(area, 50, 20))
        }

        Ok(())
    }
}

impl HelpContentText for ConnectionListComponent {
    fn help_content_text() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("i", "Insert a new database connection"),
            ("d", "Delete the database connection"),
            ("e", "Edit the database connection"),
        ])
    }
}
