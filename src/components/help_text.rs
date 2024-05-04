use super::Component;
use crate::{
    application::Store,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

pub struct HelpTextComponent {}

impl HelpTextComponent {
    pub fn new() -> Self {
        HelpTextComponent {}
    }
}

impl Component for HelpTextComponent {
    fn event(&self, _input: &Keys, _store: &Store) -> anyhow::Result<EventState> {
        Ok(EventState::Wasted)
    }
    fn draw(
        &self,
        frame: &mut Frame,
        area: Rect,
        _selected: bool,
        store: &Store,
    ) -> anyhow::Result<()> {
        let color_text = store.preference.theme_config.help_text_color;
        let help_text = Paragraph::new("<C-h>: Move left, <C-j>: Move down, <C-k>: Move up, <C-l>: Move right, q: Quit, ?: Help")
            .style(Style::default().fg(Color::Rgb(color_text[0], color_text[1], color_text[2])));

        frame.render_widget(help_text, area);

        Ok(())
    }
}
