use super::MutableComponent;
use crate::events::{key::Keys, EventState};

use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone)]
pub struct ConnectionItem {
    pub name: String,
    pub connection_string: String,
}

#[derive(Debug, Clone)]
pub struct ConnectionListComponent {
    connection_items: Vec<ConnectionItem>,
    list_state: ListState,
}

impl ConnectionListComponent {
    pub fn new() -> Self {
        let connection_items = vec![
            ConnectionItem {
                name: String::from("Home"),
                connection_string: String::from("mariadb://"),
            },
            ConnectionItem {
                name: String::from("Home1"),
                connection_string: String::from("mariadb://"),
            },
            ConnectionItem {
                name: String::from("Home2"),
                connection_string: String::from("mariadb://"),
            },
        ];
        let mut list_state = ListState::default();
        if connection_items.len() > 0 {
            list_state.select(Some(0));
        }
        ConnectionListComponent {
            connection_items,
            list_state,
        }
    }
}

impl MutableComponent for ConnectionListComponent {
    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('j') => {
                if let Some(i) = self.list_state.selected() {
                    let index = if i == self.connection_items.len() - 1 {
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
                        self.connection_items.len() - 1
                    } else {
                        i - 1
                    };
                    self.list_state.select(Some(index));
                } else {
                    self.list_state
                        .select(Some(self.connection_items.len() - 1));
                }
            }
            _ => return Ok(EventState::Wasted),
        }
        Ok(EventState::Consumed)
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Connections")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded);

        let list = List::new(self.connection_items.iter().map(|item| item.name.clone()))
            .block(container)
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }
}