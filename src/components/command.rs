use super::MutableComponent;
use crate::events::{key::Keys, EventState};

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
    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        match input {
            Keys::Backspace => {
                self.text_input.pop();
            }
            Keys::Enter => {}
            Keys::Char(c) => {
                self.text_input.push_str(&c.to_string());
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
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Command")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.selected_color(selected)))
            .padding(Padding::left(1))
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(&*self.text_input)
            .block(container)
            .style(Style::new().white())
            .alignment(Alignment::Left);

        frame.render_widget(text, area);
        Ok(())
    }
}
