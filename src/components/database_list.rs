use super::{centered_rect, MutableComponent};
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};
use sqlx::Row;

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
    fn event(&mut self, input: &Keys, app_state: &mut AppState) -> anyhow::Result<EventState> {
        if app_state.database_list.len() > 0 {
            match input {
                Keys::Char('j') => {
                    if let Some(i) = self.list_state.selected() {
                        let index = if i == app_state.database_list.len() - 1 {
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
                            app_state.database_list.len() - 1
                        } else {
                            i - 1
                        };
                        self.list_state.select(Some(index));
                    } else {
                        self.list_state
                            .select(Some(app_state.database_list.len() - 1));
                    }
                }
                Keys::Enter => {
                    if let Some(index) = self.list_state.selected() {
                        let current_db = app_state.database_list[index].clone();
                        let connection = &app_state.connection_list.list[app_state.current_connection.unwrap()];
                        let pool = connection.pool.as_ref().unwrap().clone();
                        let actions_tx = app_state.actions_tx.clone();
                        tokio::spawn(async move {
                            let rows = sqlx::query(
                                &format!("select table_name from information_schema.tables where table_schema='{}'",current_db)
                            )
                            .fetch_all(&pool)
                            .await
                            .unwrap();
                            actions_tx
                                .send(crate::app::AppStateAction::SendTablesData(
                                    rows.iter()
                                        .map(|row| row.try_get("table_name").unwrap())
                                        .collect(),
                                ))
                                .unwrap();
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
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        selected: bool,
        app_state: &AppState,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Databases")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, app_state.config.theme_config)),
            )
            .border_type(BorderType::Rounded);

        if app_state.database_list.len() > 0 {
            let list = List::new(app_state.database_list.iter().enumerate().map(
                |(index, item)| {
                    if self.selected == index as isize {
                        format!(" > {}", item.clone())
                    } else {
                        item.clone()
                    }
                },
            ))
            .block(container)
            .highlight_style(Style::default().fg(Color::LightGreen))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, area, &mut self.list_state);
        } else {
            let no_data = Paragraph::new("No databases")
                .style(Style::new().italic())
                .centered();

            frame.render_widget(container, area);
            frame.render_widget(no_data, centered_rect(area, 50, 20));
        }

        Ok(())
    }
}
