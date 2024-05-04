use super::{centered_rect, HelpContentText, MutableComponent};
use crate::{
    application::Store,
    components::{InputAction, InputPopupComponent, LayoutArea},
    events::{key::Keys, EventState},
    sql::{connection::Connection, database::Database},
};
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConnectionListComponent {
    list_state: ListState,
    popup: Option<InputPopupComponent>,
}

impl ConnectionListComponent {
    pub fn new() -> Self {
        ConnectionListComponent {
            list_state: ListState::default(),
            popup: None,
        }
    }
}

impl MutableComponent for ConnectionListComponent {
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        if let Some(popup) = &mut self.popup {
            let event = popup.event(&input, store)?;
            match event {
                EventState::ConfirmedText(content) => {
                    match popup.action {
                        InputAction::Insert => {
                            store.connection_list.list.push(Connection::new(content));
                        }
                        InputAction::Edit => {
                            store.connection_list.list[self.list_state.selected().unwrap()]
                                .connection_string = content;
                        }
                    }
                    self.popup = None;
                    store.is_lock = false;
                }
                EventState::Escaped => {
                    self.popup = None;
                    store.is_lock = false;
                }
                _ => {}
            }
            return Ok(EventState::Consumed);
        }

        match input {
            Keys::Char('i') => {
                store.is_lock = true;
                self.popup = Some(InputPopupComponent::new(
                    String::from("Connection string"),
                    String::new(),
                    InputAction::Insert,
                ));
            }
            Keys::Char('?') => {
                store.selected_pane = (100, 100);
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
                    let index = if let Some(i) = self.list_state.selected() {
                        i
                    } else {
                        return Ok(EventState::Consumed);
                    };
                    store.is_lock = true;
                    self.popup = Some(InputPopupComponent::new(
                        String::from("Connection string"),
                        store.connection_list.list[index].connection_string.clone(),
                        InputAction::Edit,
                    ));
                    store.connection_list.reset_current_connection();
                }
                Keys::Enter => {
                    store.log("Trying to connect to the database...");
                    if let Some(index) = self.list_state.selected() {
                        if let Err(e) = &store.connection_list.set_current_connection(index) {
                            store.error(e);
                            return Ok(EventState::Wasted);
                        }
                        store.connection_list.is_loading = true;
                        let pool = store.connection_list.get_pool().unwrap();
                        let actions_tx = store.actions_tx.clone();
                        Database::get_databases(pool, actions_tx);
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
        layout: &LayoutArea,
    ) -> anyhow::Result<()> {
        if let Some(popup) = &mut self.popup {
            if selected {
                popup.draw(
                    frame,
                    centered_rect(layout.main_area[0], 40, 5),
                    true,
                    store,
                    layout,
                )?;
            } else {
                self.popup = None;
            }
        }

        let container = Block::default()
            .title("Connections")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.preference.theme_config)),
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
                    if let Some(i) = store.connection_list.current_connection {
                        if i == index && store.connection_list.is_loading == false {
                            return format!(" * {}", item.connection_string.clone());
                        }
                    }
                    item.connection_string.clone()
                },
            ))
            .block(container.title_bottom(format!(
                "{} of {}",
                selected_idx,
                store.connection_list.list.len()
            )))
            .style(Style::default().fg(self.get_color(store.preference.theme_config.unselected_color)))
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
