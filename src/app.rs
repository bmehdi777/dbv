use super::{
    components::Component,
    components::TabComponent,
    events::{key::Keys, EventState},
};
use crossterm::event::{self, KeyEventKind};
use ratatui::{prelude::*, Frame};

pub struct App {
    tab: TabComponent,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        let tab = TabComponent::new();
        App { tab, exit: false }
    }

    pub fn draw(&self, frame: &mut Frame) -> anyhow::Result<()> {
        let main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(3)])
            .split(frame.size());

        let left_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
            .split(main_area[0]);

        self.tab.draw(frame, left_area[0])?;
        Ok(())
    }

    pub fn event_handling(&mut self, k: Keys) -> anyhow::Result<()> {
        self.event(&k)?;
        self.tab.event(&k)?;
        Ok(())
    }

    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        match input {
            Keys::Char('q') => {
                self.exit = true;
                Ok(EventState::Consumed)
            }
            _ => Ok(EventState::Wasted),
        }
    }
}
