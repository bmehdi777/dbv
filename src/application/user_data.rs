use serde::{Serialize, Deserialize};
use crate::sql::connection::ConnectionList;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(flatten)]
    pub connection_list: ConnectionList
}

impl UserData {
    pub fn new() -> Self {
        UserData {
            connection_list: ConnectionList::new()
        }
    }
}
