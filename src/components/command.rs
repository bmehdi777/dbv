use super::MutableComponent;
use crate::{
    application::Store,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

pub struct CommandComponent {
    pub text_input: String,
}

impl CommandComponent {
    pub fn new() -> Self {
        CommandComponent {
            text_input: String::new(),
        }
    }
}

impl MutableComponent for CommandComponent {
    fn event(&mut self, input: &Keys, store: &mut Store) -> anyhow::Result<EventState> {
        match input {
            Keys::Backspace => {
                self.text_input.pop();
            }
            Keys::Enter => {
                let result = Ok(EventState::ConfirmedText(self.text_input.clone()));
                self.text_input = String::new();
                return result;
            }
            Keys::Char(c) => {
                self.text_input.push_str(&c.to_string());
            }
            Keys::Esc => {
                self.text_input = String::new();
                store.selected_pane = store.previous_selected_pane;
            }
            _ => return Ok(EventState::Wasted),
        }
        Ok(EventState::Consumed)
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        selected: bool,
        store: &Store,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Command")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, store.config.theme_config)),
            )
            .padding(Padding::left(1))
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(&*self.text_input)
            .block(container)
            .style(Style::new().white())
            .alignment(Alignment::Left);
        if selected {
            frame.set_cursor(area.x + 2 + self.text_input.len() as u16, area.y + 1);
        }
        frame.render_widget(text, area);
        Ok(())
    }
}
