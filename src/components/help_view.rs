use super::MutableComponent;
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

pub struct HelpViewComponent {
    help_content: HashMap<String, String>,
    scrollstate: ScrollbarState,
}

impl HelpViewComponent {
    pub fn new(help_content: Option<HashMap<String, String>>) -> HelpViewComponent {
        if let Some(hc) = help_content {
            return HelpViewComponent {
                help_content: hc.clone(),
                scrollstate: ScrollbarState::new(hc.len()).position(0),
            };
        }
        HelpViewComponent {
            help_content: HashMap::new(),
            scrollstate: ScrollbarState::new(0).position(0),
        }
    }
}

impl MutableComponent for HelpViewComponent {
    fn event(&mut self, input: &Keys, app_state: &AppState) -> anyhow::Result<EventState> {
        Ok(EventState::Wasted)
    }
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        app_state: &AppState,
    ) -> anyhow::Result<()> {
        let container = Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .border_style(
                Style::default().fg(self.selected_color(selected, app_state.config.theme_config)),
            );
        let text = Paragraph::new(
            self.help_content
                .iter()
                .map(|(key, value)| {
                    Line::from(vec![
                        Span::styled(key.to_owned(), Style::new().green()),
                        value.into(),
                    ])
                })
                .collect::<Vec<Line>>(),
        )
        .scroll((0, 0))
        .block(container);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        frame.render_widget(Clear, area);
        frame.render_widget(text, area);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.scrollstate,
        );

        Ok(())
    }
}
