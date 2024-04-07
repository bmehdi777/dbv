use super::{
    components::Component,
    components::*,
    events::{key::Keys, EventState},
};
use ratatui::{prelude::*, Frame};

pub struct App<'a> {
    tab: TabComponent,
    connection_list: ConnectionListComponent,
    database_list: DatabaseListComponent,
    table_list: TableListComponent,
    command: CommandComponent,
    records_view: RecordsViewComponent<'a>,
    result_view: ResultViewComponent,

    selected_pane: (u8, u8), // x,y
    max_pane_column: [u8; 2],
    pub exit: bool,
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

        App {
            tab,
            connection_list,
            database_list,
            table_list,

            records_view,
            result_view,
            command,

            selected_pane: (0, 0),
            max_pane_column: [3, 2],
            exit: false,
        }
    }
    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let select_connection_list = self.selected_pane.0 == 0 && self.selected_pane.1 == 0;
        let select_database_list = self.selected_pane.0 == 0 && self.selected_pane.1 == 1;
        let select_table_list = self.selected_pane.0 == 0 && self.selected_pane.1 == 2;

        let select_tab = self.selected_pane.0 == 1 && self.selected_pane.1 == 0;
        let select_records_view = self.selected_pane.0 == 1 && self.selected_pane.1 == 1;
        let select_command = self.selected_pane.0 == 1 && self.selected_pane.1 == 3;

        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.size());


        let sub_main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(2), Constraint::Fill(4)])
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
            .draw(frame, left_area[0], select_connection_list)?;
        self.database_list
            .draw(frame, left_area[1], select_database_list)?;
        self.table_list
            .draw(frame, left_area[2], select_table_list)?;

        self.tab.draw(frame, right_area[0], select_tab)?;
        self.result_view.draw(frame, right_area[2], false)?;
        self.records_view
            .draw(frame, right_area[1], select_records_view)?;
        self.command.draw(frame, right_area[3], select_command)?;

        Ok(())
    }

    pub fn event_handling(&mut self, k: Keys) -> anyhow::Result<()> {
        self.event(&k)?;
        match self.selected_pane {
            (0, 0) => {
                self.connection_list.event(&k)?;
            }
            (0, 1) => {
                self.database_list.event(&k)?;
            }
            (0, 2) => {
                self.table_list.event(&k)?;
            }
            (1, 0) => {
                self.tab.event(&k)?;
            }
            (1, 3) => {
                self.command.event(&k)?;
            }
            (_, _) => {}
        }
        Ok(())
    }

    fn event(&mut self, input: &Keys) -> anyhow::Result<EventState> {
        match input {
            Keys::CtrlChar('j') => {
                let max_pane = self.max_pane_column[self.selected_pane.0 as usize];
                if self.selected_pane.1 == max_pane - 1 {
                    self.selected_pane.1 = 0;
                } else {
                    self.selected_pane.1 += 1;
                }
            }
            Keys::CtrlChar('k') => {
                let max_pane = self.max_pane_column[self.selected_pane.0 as usize];
                if self.selected_pane.1 == 0 {
                    self.selected_pane.1 = max_pane - 1;
                } else {
                    self.selected_pane.1 -= 1;
                }
            }
            Keys::CtrlChar('l') => {
                if self.selected_pane.1 >= 2 {
                    self.selected_pane.1 = 1;
                }
                self.selected_pane.0 = if self.selected_pane.0 == 1 { 0 } else { 1 };
            }
            Keys::CtrlChar('h') => {
                if self.selected_pane.1 >= 2 {
                    self.selected_pane.1 = 1;
                }
                self.selected_pane.0 = if self.selected_pane.0 == 0 { 1 } else { 0 };
            }
            Keys::Char('q') => {
                if self.selected_pane.0 == 1 && self.selected_pane.1 == 3 {
                    return Ok(EventState::Wasted);
                }
                self.exit = true;
            }
            _ => return Ok(EventState::Wasted),
        }
        Ok(EventState::Consumed)
    }
}
