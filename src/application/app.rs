use super::Store;
use crate::{
    components::*,
    events::{key::Keys, EventState},
};
use ratatui::{prelude::*, widgets::*, Frame};
use std::collections::HashMap;

pub struct App<'a> {
    tab: TabComponent,
    connection_list: ConnectionListComponent,
    database_list: DatabaseListComponent,
    table_list: TableListComponent,
    command: CommandComponent,
    records_view: RecordsViewComponent<'a>,
    help_text: HelpTextComponent,
    help_view: HelpViewComponent,
    log_view: LogViewComponent,

    max_pane_column: [u8; 2],
    pub store: Store,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let tab = TabComponent::new();
        let connection_list = ConnectionListComponent::new();
        let database_list = DatabaseListComponent::new();
        let table_list = TableListComponent::new();
        let command = CommandComponent::new();
        let records_view = RecordsViewComponent::new();
        let help_text = HelpTextComponent::new();
        let help_view =
            HelpViewComponent::new(0, "Connections list".into(), App::help_view_text((0, 0)));
        let log_view = LogViewComponent::new();

        let store = Store::new();
        App {
            tab,
            connection_list,
            database_list,
            table_list,

            records_view,
            log_view,
            command,

            help_text,
            help_view,

            max_pane_column: [3, 3],
            store,
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        if let Some(paragraph) = self.verify_space_available(frame) {
            frame.render_widget(paragraph, frame.size());
            return Ok(());
        }

        let select_connection_list =
            self.store.selected_pane.0 == 0 && self.store.selected_pane.1 == 0;
        let select_database_list =
            self.store.selected_pane.0 == 0 && self.store.selected_pane.1 == 1;
        let select_table_list = self.store.selected_pane.0 == 0 && self.store.selected_pane.1 == 2;

        let select_tab = self.store.selected_pane.0 == 1 && self.store.selected_pane.1 == 0;
        let select_records_view =
            self.store.selected_pane.0 == 1 && self.store.selected_pane.1 == 1;
        let select_log_view = self.store.selected_pane.0 == 1 && self.store.selected_pane.1 == 2;
        let select_command = self.store.selected_pane.0 == 1 && self.store.selected_pane.1 == 3;

        let layout = LayoutArea::new(&frame);

        self.connection_list.draw(
            frame,
            layout.left_area[0],
            select_connection_list,
            &self.store,
            &layout,
        )?;
        self.database_list.draw(
            frame,
            layout.left_area[1],
            select_database_list,
            &self.store,
            &layout,
        )?;
        self.table_list.draw(
            frame,
            layout.left_area[2],
            select_table_list,
            &self.store,
            &layout,
        )?;

        self.tab.draw(
            frame,
            layout.right_area[0],
            select_tab,
            &self.store,
            &layout,
        )?;
        self.records_view.draw(
            frame,
            layout.right_area[1],
            select_records_view,
            &self.store,
            &layout,
        )?;
        self.log_view.draw(
            frame,
            layout.right_area[2],
            select_log_view,
            &self.store,
            &layout,
        )?;
        self.command.draw(
            frame,
            layout.right_area[3],
            select_command,
            &self.store,
            &layout,
        )?;

        self.help_text
            .draw(frame, layout.main_area[1], false, &self.store)?;

        match self.store.selected_pane {
            (100, 100) => {
                self.help_view.draw(
                    frame,
                    centered_rect(layout.main_area[0], 25, 50),
                    true,
                    &self.store,
                    &layout,
                )?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn event_handling(&mut self, k: Keys) -> anyhow::Result<()> {
        self.event(&k)?;
        match self.store.selected_pane {
            (0, 0) => {
                self.connection_list.event(&k, &mut self.store)?;
            }
            (0, 1) => {
                self.database_list.event(&k, &mut self.store)?;
            }
            (0, 2) => {
                self.table_list.event(&k, &mut self.store)?;
            }
            (1, 0) => {
                self.tab.event(&k, &mut self.store)?;
            }
            (1, 2) => {
                self.log_view.event(&k, &mut self.store)?;
            }
            (1, 3) => {
                let event = self.command.event(&k, &mut self.store)?;
                if let EventState::ConfirmedText(_content) = event {
                    // todo
                    self.store.selected_pane = self.store.previous_selected_pane;
                }
            }
            (100, 100) => {
                self.help_view.event(&k, &mut self.store)?;
            }
            (_, _) => {}
        }
        Ok(())
    }

    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        if self.store.selected_pane != (101, 101) && self.store.selected_pane != (1, 3) {
            match input {
                Keys::CtrlChar('j') => {
                    if self.store.selected_pane == (100, 100) {
                        self.store.selected_pane = self.store.previous_selected_pane;
                        return Ok(EventState::Consumed);
                    }
                    let max_pane = self.max_pane_column[self.store.selected_pane.0 as usize];
                    if self.store.selected_pane.1 == max_pane - 1 {
                        self.store.selected_pane.1 = 0;
                    } else {
                        self.store.selected_pane.1 += 1;
                    }
                }
                Keys::CtrlChar('k') => {
                    if self.store.selected_pane == (100, 100) {
                        self.store.selected_pane = self.store.previous_selected_pane;
                        return Ok(EventState::Consumed);
                    }
                    let max_pane = self.max_pane_column[self.store.selected_pane.0 as usize];
                    if self.store.selected_pane.1 == 0 {
                        self.store.selected_pane.1 = max_pane - 1;
                    } else {
                        self.store.selected_pane.1 -= 1;
                    }
                }
                Keys::CtrlChar('l') => {
                    if self.store.selected_pane == (100, 100) {
                        self.store.selected_pane = self.store.previous_selected_pane;
                        return Ok(EventState::Consumed);
                    }
                    if self.store.selected_pane.1 >= 3 {
                        self.store.selected_pane.1 = 1;
                    }
                    self.store.selected_pane.0 = if self.store.selected_pane.0 == 1 {
                        0
                    } else {
                        1
                    };
                }
                Keys::CtrlChar('h') => {
                    if self.store.selected_pane == (100, 100) {
                        self.store.selected_pane = self.store.previous_selected_pane;
                        return Ok(EventState::Consumed);
                    }
                    if self.store.selected_pane.1 >= 3 {
                        self.store.selected_pane.1 = 1;
                    }
                    self.store.selected_pane.0 = if self.store.selected_pane.0 == 0 {
                        1
                    } else {
                        0
                    };
                }
                Keys::Char('q') => {
                    // don't quit if we are in command or popup pane
                    if self.store.selected_pane == (1, 3) || self.store.selected_pane == (101, 101)
                    {
                        return Ok(EventState::Wasted);
                    }

                    if self.store.selected_pane == (100, 100) {
                        self.store.selected_pane = self.store.previous_selected_pane;
                        return Ok(EventState::Consumed);
                    }

                    self.store.exit = true;
                }
                Keys::Esc => {
                    if self.store.selected_pane == (100, 100) {
                        self.store.selected_pane = self.store.previous_selected_pane;
                        return Ok(EventState::Consumed);
                    }
                }
                Keys::Char(':') => {
                    if !self.store.is_lock {
                        self.store.previous_selected_pane = self.store.selected_pane;
                        self.store.selected_pane = (1, 3);
                    }
                }
                _ => return Ok(EventState::Wasted),
            }
        }
        Ok(EventState::Consumed)
    }

    fn help_view_text(selected_pane: (u8, u8)) -> Option<HashMap<&'static str, &'static str>> {
        match selected_pane {
            (0, 0) => return Some(ConnectionListComponent::help_content_text()),
            (0, 1) => return Some(DatabaseListComponent::help_content_text()),
            _ => return None,
        }
    }

    fn verify_space_available(&mut self, frame: &mut Frame) -> Option<Paragraph> {
        let size = frame.size();
        if size.width <= 50 || size.height <= 21 {
            let color = Color::Rgb(
                self.store.config.theme_config.unselected_color[0],
                self.store.config.theme_config.unselected_color[1],
                self.store.config.theme_config.unselected_color[2],
            );
            let not_enough_space = Paragraph::default().block(
                Block::default()
                    .title("Not enough space to render")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color)),
            );

            return Some(not_enough_space);
        }
        None
    }
}
