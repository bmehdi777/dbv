use super::MutableComponent;
use crate::{
    application::Store,
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
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        match input {
            Keys::Backspace => {
                self.content.pop();
            }
            Keys::Enter => {
                if self.content.trim().len() == 0 {
                    store.selected_pane = store.previous_selected_pane;
                    return Ok(EventState::Escaped);
                }
                let event = Ok(EventState::ConfirmedText(self.content.clone()));
                self.content = String::new();
                return event;
            }
            Keys::Char(c) => {
                self.content.push_str(&c.to_string());
            }
            Keys::Esc => {
                store.selected_pane = store.previous_selected_pane;
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
        store: &Store,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title(&*self.title)
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.get_color(store.config.theme_config.selected_color)),
            )
            .padding(Padding::left(1))
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(&*self.content)
            .block(container)
            .style(Style::new().fg(self.get_color(store.config.theme_config.unselected_color)));

        frame.render_widget(Clear, area);
        frame.set_cursor(area.x + 2 + self.content.len() as u16, area.y + 1);
        frame.render_widget(text, area);
        Ok(())
    }
}
