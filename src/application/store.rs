use crate::{
    config::Config,
    sql::connection::ConnectionList,
    components::LogContent,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};

pub enum StoreAction {
    SendDatabaseData(Vec<String>),
    SendTablesData(Vec<String>),
}

pub struct Store {
    pub config: Config,
    pub connection_list: ConnectionList,
    pub database_list: Vec<String>,
    pub tables_list: Vec<String>,
    pub exit: bool,
    pub selected_pane: (u8, u8), //x,y
    pub previous_selected_pane: (u8, u8),

    log_contents: Vec<LogContent>,

    pub actions_tx:  UnboundedSender<StoreAction>,
    actions_rx: UnboundedReceiver<StoreAction>
}

impl Store {
    pub fn new() -> Self {
        let (actions_tx, actions_rx) = unbounded_channel();
        Store {
            config: Config::default().load(),
            connection_list: ConnectionList::new(),
            database_list: Vec::new(),
            tables_list: Vec::new(),
            exit: false,
            selected_pane: (0, 0),
            previous_selected_pane: (0, 0),
            log_contents: Vec::new(),
            actions_tx,
            actions_rx,
        }
    }

    pub fn update(&mut self) {
        while let Ok(action) = self.actions_rx.try_recv() {
            match action {
                StoreAction::SendDatabaseData(data) => {
                    self.log(&format!("{:?}", data));
                    self.database_list = data;
                    self.selected_pane = (0,1);
                }
                StoreAction::SendTablesData(data) => {
                    self.log(&format!("{:?}", data));
                    self.tables_list = data;
                    self.selected_pane = (0,2);
                }
                _ => {}
            }
        }
    }
 
    pub fn log(&mut self, content: &str) {
        self.log_contents.push(LogContent::Info(content.into()))
    }
    pub fn success(&mut self, content: &str) {
        self.log_contents.push(LogContent::Success(content.into()))
    }
    pub fn error(&mut self, content: &str) {
        self.log_contents.push(LogContent::Error(content.into()))
    }
    pub fn debug(&mut self, content: &str) {
        self.log_contents.push(LogContent::Debug(content.into()))
    }
    pub fn log_contents(&self) -> &Vec<LogContent> {
        &self.log_contents
    }
} 
