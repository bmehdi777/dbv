use super::{preferences::Preference, user_data::UserData};
use crate::{components::LogContent, events::events::EventsHandling, utils};
use std::{fs, path::Path};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

const STORE_FILENAME: &'static str = "user_data.json";

#[derive(Debug, Clone)]
pub enum StoreAction {
    SendDatabaseData(Vec<String>),
    SendTablesData(Vec<String>),

    SendEditConnectionItem(usize),

    SendError(String),
}

pub struct Store<'a> {
    pub preference: Preference,
    pub user_data: UserData,
    pub event_handler: &'a EventsHandling,
    pub database_list: Vec<String>,
    pub tables_list: Vec<String>,
    pub exit: bool,
    pub selected_pane: (u8, u8), //x,y
    pub previous_selected_pane: (u8, u8),
    pub is_lock: bool,

    log_contents: Vec<LogContent>,

    pub actions_tx: UnboundedSender<StoreAction>,
    pub actions_rx: UnboundedReceiver<StoreAction>,
}

impl<'a> Store<'a> {
    pub fn new(event_handler: &'a EventsHandling) -> Self {
        let (actions_tx, actions_rx) = unbounded_channel();
        Store {
            preference: Preference::default().load(),
            event_handler,
            user_data: UserData::new(),
            database_list: Vec::new(),
            tables_list: Vec::new(),
            exit: false,
            selected_pane: (0, 0),
            previous_selected_pane: (0, 0),
            is_lock: false,
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
                    self.user_data.connection_list.is_loading = false;
                    self.database_list = data;
                    self.selected_pane = (0, 1);
                }
                StoreAction::SendTablesData(data) => {
                    self.log(&format!("{:?}", data));
                    self.tables_list = data;
                    self.selected_pane = (0, 2);
                }

                StoreAction::SendError(e) => {
                    self.error(&format!("{:?}", e));
                    self.user_data.connection_list.current_connection = None;
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

    pub fn save(&self) -> anyhow::Result<()> {
        if !Path::new(&utils::get_path_app_folder()).exists() {
            anyhow::bail!("$HOME/.config/dbv/ doesn't exist.");
        }

        let user_data = serde_json::to_string_pretty(&self.user_data)?;
        fs::write(utils::get_path_app_file(STORE_FILENAME), user_data)?;

        Ok(())
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let filepath = utils::get_path_app_file(STORE_FILENAME);
        if !Path::new(&filepath).exists() {
            return Ok(());
        }

        let user_data = fs::read_to_string(&filepath)?;
        self.user_data = serde_json::from_str(&user_data)?;

        Ok(())
    }
}
