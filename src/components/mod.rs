use anyhow::Result;
use ratatui::{layout::Rect, Frame};

use crate::events::{key::Keys, EventState};

pub mod tab;
pub use tab::TabComponent;

pub trait Component {
    fn event(&mut self, input: &Keys) -> Result<EventState>;
    fn draw(&self, frame: &mut Frame, area: Rect) -> Result<()>;
}

