use super::{
    components::TabComponent,
    components::{Component, DrawableComponent},
    events::{key::Keys, EventState},
};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{layout::Rect, prelude::*, Frame};

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
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(5), Constraint::Fill(1)])
            .split(frame.size());

        self.tab.draw(frame, main_area[0])?;
        Ok(())
    }

    pub fn event_handling(&mut self) -> anyhow::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let k: Keys = key.into();
                    self.event(&k)?;
                    self.tab.event(&k)?;
                }
            }
        }
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
