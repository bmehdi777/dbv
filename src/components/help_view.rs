use super::MutableComponent;
use crate::{
    app::AppState,
    events::{key::Keys, EventState},
};

use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;

pub trait HelpContentText {
    fn help_content_text() -> HashMap<&'static str, &'static str>;
}

pub struct HelpViewComponent {
    help_content: HashMap<&'static str, &'static str>,
    scrollstate: ScrollbarState,
    tablestate: TableState,
}

impl HelpViewComponent {
    pub fn new(help_content: Option<HashMap<&'static str, &'static str>>) -> HelpViewComponent {
        if let Some(hc) = help_content {
            log::debug!("{:?}", &hc);
            return HelpViewComponent {
                help_content: hc.clone(),
                scrollstate: ScrollbarState::new(hc.len()).position(0),
                tablestate: TableState::default(),
            };
        }
        HelpViewComponent {
            help_content: HashMap::new(),
            scrollstate: ScrollbarState::new(0).position(0),
            tablestate: TableState::default(),
        }
    }
}

impl MutableComponent for HelpViewComponent {
    fn event(&mut self, _input: &Keys, _app_state: &AppState) -> anyhow::Result<EventState> {
        Ok(EventState::Wasted)
        // todo
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
        log::debug!("render");
        log::debug!("{:?}", self.help_content);
        let table = Table::new(
            self.help_content
                .iter()
                .map(|(key, value)| {
                    Row::new(vec![
                        Cell::from(*key).style(
                            Style::default()
                                .fg(self.get_color(app_state.config.theme_config.help_key_color)),
                        ),
                        Cell::from(*value).style(
                            Style::default()
                                .fg(self.get_color(app_state.config.theme_config.help_desc_color))
                                .italic(),
                        ),
                    ])
                })
                .collect::<Vec<Row>>(),
            [Constraint::Length(4), Constraint::Fill(1)],
        )
        .block(container)
        .highlight_style(Style::default().reversed());

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(table, area, &mut self.tablestate);
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
