use anyhow::Result;
use ratatui::{layout::Rect, Frame};

use crate::events::{key::Keys, EventState};

pub mod tab;
pub mod connection_list;

pub use connection_list::ConnectionListComponent;
pub use tab::TabComponent;

pub trait Component {
    fn event(&mut self, input: &Keys) -> Result<EventState>;
    fn draw(&self, frame: &mut Frame, area: Rect) -> Result<()>;
}

pub trait MutableComponent {
    fn event(&mut self, input: &Keys) -> Result<EventState>;
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;
}

