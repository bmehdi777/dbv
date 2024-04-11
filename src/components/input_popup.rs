use super::MutableComponent;
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

pub struct InputPopupComponent {
    pub title: String,
    pub content: String,
}

impl InputPopupComponent {
    pub fn new(title: String, content: String) -> Self {
        InputPopupComponent { title, content }
    }
    pub fn default() -> Self {
        InputPopupComponent {
            title: String::new(),
            content: String::new(),
        }
    }
}

impl MutableComponent for InputPopupComponent {
    fn event(&mut self, input: &Keys, app_state: &mut AppState) -> anyhow::Result<EventState> {
        match input {
            Keys::Backspace => {
                self.content.pop();
            }
            Keys::Enter => return Ok(EventState::ConfirmedText(self.content.clone())),
            Keys::Char(c) => {
                self.content.push_str(&c.to_string());
            }
            Keys::Esc => {
                app_state.selected_pane = app_state.previous_selected_pane;
            }
            _ => return Ok(EventState::Wasted),
        }
        Ok(EventState::Consumed)
    }
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _selected: bool,
        app_state: &AppState,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title(&*self.title)
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.get_color(app_state.config.theme_config.selected_color)),
            )
            .padding(Padding::left(1))
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(&*self.content)
            .block(container)
            .style(Style::new().fg(self.get_color(app_state.config.theme_config.unselected_color)));

        frame.render_widget(Clear, area);
        frame.set_cursor(area.x + 2 + self.content.len() as u16, area.y + 1);
        frame.render_widget(text, area);
        Ok(())
    }
}
