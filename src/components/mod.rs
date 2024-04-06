use anyhow::Result;
use ratatui::{layout::Rect, prelude::*, Frame};

use crate::events::{key::Keys, EventState};

pub mod connection_list;
pub mod database_list;
pub mod tab;
pub mod table_list;

pub mod command;

pub use connection_list::ConnectionListComponent;
pub use database_list::DatabaseListComponent;
pub use table_list::TableListComponent;
pub use tab::TabComponent;
pub use command::CommandComponent;

pub trait Component {
    fn event(&mut self, input: &Keys) -> Result<EventState>;
    fn draw(&self, frame: &mut Frame, area: Rect, selected: bool) -> Result<()>;

    fn selected_color(&self, selected: bool) -> Color {
        if selected {
            Color::LightGreen
        } else {
            Color::White
        }
    }
}

pub trait MutableComponent {
    fn event(&mut self, input: &Keys) -> Result<EventState>;
    fn draw(&mut self, frame: &mut Frame, area: Rect, selected: bool) -> Result<()>;

    fn selected_color(&self, selected: bool) -> Color {
        if selected {
            Color::LightGreen
        } else {
            Color::White
        }
    }
}
