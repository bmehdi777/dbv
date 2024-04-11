use super::{
    components::*,
    config::Config,
    events::{key::Keys, EventState},
    sql::database::{DatabaseConnection, DatabaseConnectionList},
};
use ratatui::{prelude::*, widgets::*, Frame};
use std::collections::HashMap;

pub struct AppState {
    pub config: Config,
    pub connection_list: DatabaseConnectionList,
    pub exit: bool,
    pub selected_pane: (u8, u8), //x,y
    pub previous_selected_pane: (u8, u8),
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            config: Config::default().load(),
            connection_list: DatabaseConnectionList::new(),
            exit: false,
            selected_pane: (0, 0),
            previous_selected_pane: (0, 0),
        }
    }
}

pub struct App<'a> {
    tab: TabComponent,
    connection_list: ConnectionListComponent,
    database_list: DatabaseListComponent,
    table_list: TableListComponent,
    command: CommandComponent,
    records_view: RecordsViewComponent<'a>,
    result_view: ResultViewComponent,
    help_text: HelpTextComponent,
    help_view: HelpViewComponent,

    popup: InputPopupComponent,

    max_pane_column: [u8; 2],
    pub app_state: AppState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let tab = TabComponent::new();
        let connection_list = ConnectionListComponent::new();
        let database_list = DatabaseListComponent::new();
        let table_list = TableListComponent::new();
        let command = CommandComponent::new();
        let records_view = RecordsViewComponent::new();
        let result_view = ResultViewComponent::new();
        let help_text = HelpTextComponent::new();
        let help_view =
            HelpViewComponent::new(0, "Connections list".into(), App::help_view_text((0, 0)));
        let popup = InputPopupComponent::default();

        let app_state = AppState::new();
        App {
            tab,
            connection_list,
            database_list,
            table_list,

            records_view,
            result_view,
            command,

            help_text,
            help_view,

            popup,

            max_pane_column: [3, 2],
            app_state,
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        if let Some(paragraph) = self.verify_space_available(frame) {
            frame.render_widget(paragraph, frame.size());
            return Ok(());
        }

        let select_connection_list =
            self.app_state.selected_pane.0 == 0 && self.app_state.selected_pane.1 == 0;
        let select_database_list =
            self.app_state.selected_pane.0 == 0 && self.app_state.selected_pane.1 == 1;
        let select_table_list =
            self.app_state.selected_pane.0 == 0 && self.app_state.selected_pane.1 == 2;

        let select_tab = self.app_state.selected_pane.0 == 1 && self.app_state.selected_pane.1 == 0;
        let select_records_view =
            self.app_state.selected_pane.0 == 1 && self.app_state.selected_pane.1 == 1;
        let select_command =
            self.app_state.selected_pane.0 == 1 && self.app_state.selected_pane.1 == 3;

        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.size());

        let sub_main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(2), Constraint::Fill(6)])
            .split(main_area[0]);

        let left_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(if select_connection_list { 5 } else { 1 }),
                Constraint::Fill(if select_database_list { 5 } else { 1 }),
                Constraint::Fill(if select_table_list { 5 } else { 1 }),
            ])
            .split(sub_main_area[0]);
        let right_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(2),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(sub_main_area[1]);

        self.connection_list
            .draw(frame, left_area[0], select_connection_list, &self.app_state)?;
        self.database_list
            .draw(frame, left_area[1], select_database_list, &self.app_state)?;
        self.table_list
            .draw(frame, left_area[2], select_table_list, &self.app_state)?;

        self.tab
            .draw(frame, right_area[0], select_tab, &self.app_state)?;
        self.result_view
            .draw(frame, right_area[2], false, &self.app_state)?;
        self.records_view
            .draw(frame, right_area[1], select_records_view, &self.app_state)?;
        self.command
            .draw(frame, right_area[3], select_command, &self.app_state)?;

        self.help_text
            .draw(frame, main_area[1], false, &self.app_state)?;

        match self.app_state.selected_pane {
            (100, 100) => {
                self.help_view.draw(
                    frame,
                    centered_rect(main_area[0], 25, 50),
                    true,
                    &self.app_state,
                )?;
            }
            (101, 101) => {
                if self.popup.title == "" {
                    self.popup =
                        InputPopupComponent::new(String::from("Connection string"), String::new());
                }
                self.popup.draw(
                    frame,
                    centered_rect(main_area[0], 40, 5),
                    true,
                    &self.app_state,
                )?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn event_handling(&mut self, k: Keys) -> anyhow::Result<()> {
        self.event(&k)?;
        match self.app_state.selected_pane {
            (0, 0) => {
                if self.help_view.id != 0 {
                    self.help_view = HelpViewComponent::new(
                        0,
                        "Connection list".into(),
                        App::help_view_text(self.app_state.selected_pane),
                    );
                }
                self.connection_list.event(&k, &mut self.app_state)?;
            }
            (0, 1) => {
                if self.help_view.id != 1 {
                    self.help_view = HelpViewComponent::new(
                        1,
                        "Database list".into(),
                        App::help_view_text(self.app_state.selected_pane),
                    );
                }
                self.database_list.event(&k, &mut self.app_state)?;
            }
            (0, 2) => {
                if self.help_view.id != 2 {
                    self.help_view = HelpViewComponent::new(
                        2,
                        "Table list".into(),
                        App::help_view_text(self.app_state.selected_pane),
                    );
                }
                self.table_list.event(&k, &mut self.app_state)?;
            }
            (1, 0) => {
                self.tab.event(&k, &mut self.app_state)?;
                self.help_view = HelpViewComponent::new(
                    3,
                    "Tab".into(),
                    App::help_view_text(self.app_state.selected_pane),
                );
            }
            (1, 3) => {
                if self.help_view.id != 4 {
                    self.help_view = HelpViewComponent::new(
                        4,
                        "Command".into(),
                        App::help_view_text(self.app_state.selected_pane),
                    );
                }
                let event = self.command.event(&k, &mut self.app_state)?;
                if let EventState::ConfirmedText(content) = event {
                    // todo
                    self.app_state.selected_pane = self.app_state.previous_selected_pane;
                }
            }
            (100, 100) => {
                self.help_view.event(&k, &mut self.app_state)?;
            }
            (101, 101) => {
                let event = self.popup.event(&k, &mut self.app_state)?;
                if let EventState::ConfirmedText(content) = event {
                    self.app_state
                        .connection_list
                        .list
                        .push(DatabaseConnection::new(content));
                    self.app_state.selected_pane = self.app_state.previous_selected_pane;
                }
            }
            (_, _) => {}
        }
        Ok(())
    }

    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        match input {
            Keys::CtrlChar('j') => {
                let max_pane = self.max_pane_column[self.app_state.selected_pane.0 as usize];
                if self.app_state.selected_pane.1 == max_pane - 1 {
                    self.app_state.selected_pane.1 = 0;
                } else {
                    self.app_state.selected_pane.1 += 1;
                }
            }
            Keys::CtrlChar('k') => {
                let max_pane = self.max_pane_column[self.app_state.selected_pane.0 as usize];
                if self.app_state.selected_pane.1 == 0 {
                    self.app_state.selected_pane.1 = max_pane - 1;
                } else {
                    self.app_state.selected_pane.1 -= 1;
                }
            }
            Keys::CtrlChar('l') => {
                if self.app_state.selected_pane.1 >= 2 {
                    self.app_state.selected_pane.1 = 1;
                }
                self.app_state.selected_pane.0 = if self.app_state.selected_pane.0 == 1 {
                    0
                } else {
                    1
                };
            }
            Keys::CtrlChar('h') => {
                if self.app_state.selected_pane.1 >= 2 {
                    self.app_state.selected_pane.1 = 1;
                }
                self.app_state.selected_pane.0 = if self.app_state.selected_pane.0 == 0 {
                    1
                } else {
                    0
                };
            }
            Keys::Char('q') => {
                // don't quit if we are in command pane
                if self.app_state.selected_pane == (1, 3)
                    || self.app_state.selected_pane == (101, 101)
                {
                    return Ok(EventState::Wasted);
                }

                if self.app_state.selected_pane == (100, 100) {
                    self.app_state.selected_pane = self.app_state.previous_selected_pane;
                    return Ok(EventState::Consumed);
                }

                self.app_state.exit = true;
            }
            Keys::Esc => {
                if self.app_state.selected_pane == (100, 100) {
                    self.app_state.selected_pane = self.app_state.previous_selected_pane;
                    return Ok(EventState::Consumed);
                }
            }
            Keys::Char('?') => {
                self.app_state.previous_selected_pane = self.app_state.selected_pane;
                self.app_state.selected_pane = (100, 100);
            }
            Keys::Char(':') => {
                self.app_state.previous_selected_pane = self.app_state.selected_pane;
                self.app_state.selected_pane = (1, 3);
            }
            _ => return Ok(EventState::Wasted),
        }
        Ok(EventState::Consumed)
    }

    fn help_view_text(selected_pane: (u8, u8)) -> Option<HashMap<&'static str, &'static str>> {
        match selected_pane {
            (0, 0) => return Some(DatabaseListComponent::help_content_text()),
            _ => return None,
        }
    }

    fn verify_space_available(&mut self, frame: &mut Frame) -> Option<Paragraph> {
        let size = frame.size();
        if size.width <= 50 || size.height <= 21 {
            let color = Color::Rgb(
                self.app_state.config.theme_config.unselected_color[0],
                self.app_state.config.theme_config.unselected_color[1],
                self.app_state.config.theme_config.unselected_color[2],
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
