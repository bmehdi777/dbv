use anyhow::Result;
use ratatui::{layout::Rect, prelude::*, Frame};

use crate::{
    app::AppState,
    config::ThemeConfig,
    events::{key::Keys, EventState},
};

pub mod connection_list;
pub mod database_list;
pub mod help_text;
pub mod help_view;
pub mod records_view;
pub mod result_view;
pub mod tab;
pub mod table_list;
pub mod input_popup;

pub mod command;

pub use command::CommandComponent;
pub use connection_list::ConnectionListComponent;
pub use database_list::DatabaseListComponent;
pub use help_text::HelpTextComponent;
pub use help_view::HelpViewComponent;
pub use records_view::RecordsViewComponent;
pub use result_view::ResultViewComponent;
pub use result_view::LogContent;
pub use tab::TabComponent;
pub use table_list::TableListComponent;
pub use input_popup::InputPopupComponent;

pub use help_view::HelpContentText;

fn selected_color(selected: bool, theme_config: ThemeConfig) -> Color {
    if selected {
        Color::Rgb(
            theme_config.selected_color[0],
            theme_config.selected_color[1],
            theme_config.selected_color[2],
        )
    } else {
        Color::Rgb(
            theme_config.unselected_color[0],
            theme_config.unselected_color[1],
            theme_config.unselected_color[2],
        )
    }
}
fn get_color(rgb: [u8; 3]) -> Color {
    Color::Rgb(rgb[0], rgb[1], rgb[2])
}

pub trait Component {
    fn event(&self, input: &Keys, app_state: &AppState) -> Result<EventState>;
    fn draw(
        &self,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        app_state: &AppState,
    ) -> Result<()>;

    fn selected_color(&self, selected: bool, theme_config: ThemeConfig) -> Color {
        selected_color(selected, theme_config)
    }
    fn get_color(&self, rgb: [u8; 3]) -> Color {
        get_color(rgb)
    }
}

pub trait MutableComponent {
    fn event(&mut self, input: &Keys, app_state: &mut AppState) -> Result<EventState>;
    fn draw(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        selected: bool,
        app_state: &AppState,
    ) -> Result<()>;

    fn selected_color(&self, selected: bool, theme_config: ThemeConfig) -> Color {
        selected_color(selected, theme_config)
    }
    fn get_color(&self, rgb: [u8; 3]) -> Color {
        get_color(rgb)
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
