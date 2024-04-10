use super::MutableComponent;
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};

pub struct ResultViewComponent {}

impl ResultViewComponent {
    pub fn new() -> Self {
        ResultViewComponent {}
    }
}

impl MutableComponent for ResultViewComponent {
    fn event(&mut self, _input: &Keys, _app_state: &AppState) -> anyhow::Result<EventState> {
        todo!()
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

        frame.render_widget(container, area);
        Ok(())
    }
}
