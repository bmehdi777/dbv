use super::{
    components::Component,
    components::*,
    events::{key::Keys, EventState},
};
use ratatui::{prelude::*, Frame};

pub struct App {
    tab: TabComponent,
    connection_list: ConnectionListComponent,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        let tab = TabComponent::new();
        let connection_list = ConnectionListComponent::new();
        App { tab, connection_list, exit: false }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(3)])
            .split(frame.size());

        let left_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
            .split(main_area[0]);

        self.connection_list.draw(frame, left_area[0])?;
        self.tab.draw(frame, main_area[1])?;
        Ok(())
    }

    pub fn event_handling(&mut self, k: Keys) -> anyhow::Result<()> {
        self.event(&k)?;
        self.connection_list.event(&k)?;
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
