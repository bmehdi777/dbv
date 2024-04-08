use super::{centered_rect, MutableComponent};
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

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
        let connection_items = vec![];
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
    fn event(&mut self, input: &Keys, _app_state: &AppState) -> anyhow::Result<EventState> {
        if self.connection_items.len() > 0 {
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
        }
        Ok(EventState::Consumed)
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        selected: bool,
        app_state: &AppState,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Connections")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, app_state.config.theme_config)),
            )
            .border_type(BorderType::Rounded);

        if self.connection_items.len() > 0 {
            let list = List::new(self.connection_items.iter().map(|item| item.name.clone()))
                .block(container)
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, area, &mut self.list_state);
        } else {
            let no_data = Paragraph::new("No connections registered")
                .style(Style::new().italic())
                .centered();

            frame.render_widget(container, area);
            frame.render_widget(no_data, centered_rect(area, 50, 20))
        }

        Ok(())
    }
}
