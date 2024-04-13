use super::MutableComponent;
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone)]
pub enum LogContent {
    Debug(String),
    Info(String),
    Error(String),
    Success(String),
}

impl<'a> From<LogContent> for ListItem<'a> {
    fn from(val: LogContent) -> Self {
        match val {
            LogContent::Info(content) => {
                log::info!("{}", content);
                return ListItem::new(Line::from(vec![
                    Span::from("[INFO] ").style(Style::new().blue()),
                    content.into(),
                ]));
            }
            LogContent::Success(content) => {
                log::info!("SUCCESS - {}", content);
                return ListItem::new(Line::from(vec![
                    Span::from("[SUCCESS] ").style(Style::new().green()),
                    content.into(),
                ]));
            }
            LogContent::Error(content) => {
                log::error!("{}", content);
                return ListItem::new(Line::from(vec![
                    Span::from("[ERROR] ").style(Style::new().red()),
                    content.into(),
                ]));
            }
            LogContent::Debug(content) => {
                log::debug!("{}", content);
                return ListItem::new(Line::from(vec![
                    Span::from("[DEBUG] ").style(Style::new().gray()),
                    content.into(),
                ]));
            }
        }
    }
}

pub struct LogViewComponent {
    list_state: ListState,
    position_scroll: usize,
}

impl LogViewComponent {
    pub fn new() -> Self {
        LogViewComponent {
            list_state: ListState::default(),
            position_scroll: 0,
        }
    }
}

impl MutableComponent for LogViewComponent {
    fn event(&mut self, input: &Keys, app_state: &mut AppState) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('j') => {
                if let Some(i) = self.list_state.selected() {
                    let index = if i == app_state.log_contents().len() - 1 {
                        0
                    } else {
                        i + 1
                    };

                    self.list_state.select(Some(index));
                    self.position_scroll = index;
                } else {
                    self.list_state.select(Some(0));
                    self.position_scroll = 0;
                }
            }
            Keys::Char('k') => {
                if let Some(i) = self.list_state.selected() {
                    let index = if i == 0 {
                        app_state.log_contents().len() - 1
                    } else {
                        i - 1
                    };
                    self.list_state.select(Some(index));
                    self.position_scroll = index;
                } else {
                    self.list_state
                        .select(Some(app_state.log_contents().len() - 1));
                    self.position_scroll = app_state.log_contents().len() - 1;
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
        app_state: &AppState,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Log")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, app_state.config.theme_config)),
            )
            .border_type(BorderType::Rounded);

        let list = List::new(
            app_state
                .log_contents()
                .iter()
                .map(|item| LogContent::from(item.clone())),
        )
        .block(container)
        .highlight_style(Style::default().reversed());

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(app_state.log_contents().len())
            .viewport_content_length(frame.size().height.into())
            .position(self.position_scroll);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        frame.render_stateful_widget(list, area, &mut self.list_state);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
        Ok(())
    }
}
