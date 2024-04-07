use super::{centered_rect, MutableComponent};
use crate::events::{key::Keys, EventState};

use ratatui::{prelude::*, widgets::*};

pub struct ResultViewComponent {}

impl ResultViewComponent {
    pub fn new() -> Self {
        ResultViewComponent {}
    }
}

impl MutableComponent for ResultViewComponent {
    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        todo!()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, selected: bool) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Log")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.selected_color(selected)))
            .border_type(BorderType::Rounded);

        frame.render_widget(container, area);
        Ok(())
    }
}
