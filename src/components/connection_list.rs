use super::{centered_rect, HelpContentText, MutableComponent};
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};
use std::collections::HashMap;
use ratatui::{prelude::*, widgets::*};


#[derive(Debug, Clone)]
pub struct ConnectionListComponent {
    list_state: ListState,
}

impl ConnectionListComponent {
    pub fn new() -> Self {
        ConnectionListComponent {
            list_state: ListState::default(),
        }
    }
}

impl MutableComponent for ConnectionListComponent {
    fn event(&mut self, input: &Keys, app_state: &mut AppState) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('i') => {
                app_state.selected_pane = (101, 101);
            }
            _ => {}
        }
        if app_state.connection_list.list.len() > 0 {
            match input {
                Keys::Char('j') => {
                    if let Some(i) = self.list_state.selected() {
                        let index = if i == app_state.connection_list.list.len() - 1 {
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
                            app_state.connection_list.list.len() - 1
                        } else {
                            i - 1
                        };
                        self.list_state.select(Some(index));
                    } else {
                        self.list_state
                            .select(Some(app_state.connection_list.list.len() - 1));
                    }
                }
                Keys::Char('d') => {
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

        if app_state.connection_list.list.len() > 0 {
            let selected_idx = if let Some(index) = self.list_state.selected() {
                index + 1
            } else {
                0
            };
            let list = List::new(
                app_state
                    .connection_list
                    .list
                    .iter()
                    .map(|item| item.connection_string.clone()),
            )
            .block(container.title_bottom(format!(
                "{} of {}",
                selected_idx,
                app_state.connection_list.list.len()
            )))
            .style(
                Style::default().fg(self.get_color(app_state.config.theme_config.unselected_color)),
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
