use super::MutableComponent;
use crate::events::{key::Keys, EventState};

use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, Copy)]
pub enum Tab {
    Record,
    Structure,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct TabComponent {
    pub selected_tab: usize,
    tabs: Vec<Tab>,
}

impl TabComponent {
    pub fn new() -> Self {
        TabComponent {
            selected_tab: 0,
            tabs: vec![Tab::Record, Tab::Structure],
        }
    }
}

impl MutableComponent for TabComponent {
    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('l') | Keys::ArrowRight => {
                if self.selected_tab == self.tabs.len() - 1 {
                    self.selected_tab = 0;
                } else {
                    self.selected_tab += 1;
                }

                return Ok(EventState::Consumed);
            }
            Keys::Char('h') | Keys::ArrowLeft => {
                if self.selected_tab == 0 {
                    self.selected_tab = self.tabs.len() - 1;
                } else {
                    self.selected_tab -= 1;
                }

                return Ok(EventState::Consumed);
            }
            _ => return Ok(EventState::Wasted),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, selected: bool) -> anyhow::Result<()> {
        let container = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.selected_color(selected)))
            .border_type(BorderType::Rounded);

        let tabs = Tabs::new(self.tabs.iter().map(|tab| tab.to_string()))
            .block(container)
            .highlight_style(Style::default().fg(Color::Blue))
            .select(self.selected_tab as usize);

        frame.render_widget(tabs, area);
        Ok(())
    }
}
