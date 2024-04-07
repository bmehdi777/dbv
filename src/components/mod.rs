use anyhow::Result;
use ratatui::{layout::Rect, prelude::*, Frame};

use crate::events::{key::Keys, EventState};

pub mod connection_list;
pub mod database_list;
pub mod tab;
pub mod table_list;
pub mod records_view;
pub mod result_view;

pub mod command;

pub use connection_list::ConnectionListComponent;
pub use database_list::DatabaseListComponent;
pub use table_list::TableListComponent;
pub use tab::TabComponent;
pub use command::CommandComponent;
pub use records_view::RecordsViewComponent;
pub use result_view::ResultViewComponent;

pub trait Component {
    fn event(&self, input: &Keys) -> Result<EventState>;
    fn draw(&self, frame: &mut Frame, area: Rect, selected: bool) -> Result<()>;

    fn selected_color(&self, selected: bool) -> Color {
        if selected {
            Color::Green
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
            Color::Green
        } else {
            Color::White
        }
    }
}

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage((100 - percent_y) / 2),
      Constraint::Percentage(percent_y),
      Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage((100 - percent_x) / 2),
      Constraint::Percentage(percent_x),
      Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
