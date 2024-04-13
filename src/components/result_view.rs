use super::MutableComponent;
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

pub enum LogContent {
    Debug(String),
    Info(String),
    Error(String),
}

impl<'a> Into<Paragraph<'a>> for LogContent {
    fn into(self) -> Paragraph<'a> {
        todo!();
    }
}

pub struct ResultViewComponent {
    log: Vec<LogContent>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
}

impl ResultViewComponent {
    pub fn new() -> Self {
        ResultViewComponent {
            log: Vec::new(),
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::new(0).position(0),
        }
    }

    pub fn update_log(&mut self) {
        self.scrollbar_state = self
            .scrollbar_state
            .content_length(self.log.len());
    }
}

impl MutableComponent for ResultViewComponent {
    fn event(&mut self, input: &Keys, _app_state: &mut AppState) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('j') => {
                if let Some(i) = self.list_state.selected() {
                    let index = if i == self.log.len() - 1 {
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
                        self.log.len() - 1
                    } else {
                        i - 1
                    };
                    self.list_state.select(Some(index));
                } else {
                    self.list_state.select(Some(self.log.len() - 1));
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

        let list = List::new(self.log.iter().map(|item| item.clone())).block(container);

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
            &mut self.scrollbar_state,
        );
        Ok(())
    }
}
