use super::{centered_rect, MutableComponent};
use crate::events::{key::Keys, EventState};

use ratatui::{prelude::*, widgets::*};

pub struct ResultViewComponent {}

impl ResultViewComponent {
    pub fn new() -> Self {
        ResultViewComponent {}
    }
}

impl MutableComponent for ResultViewComponent {
    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        todo!()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, selected: bool) -> anyhow::Result<()> {
        todo!()
    }
}
