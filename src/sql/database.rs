use serde::{Deserialize, Serialize};
use sqlx::{Pool, mysql::{MySql , MySqlPoolOptions}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    pub connection_string: String,
}

impl DatabaseConnection {
    pub fn new(connection_string: String) -> Self {
        DatabaseConnection {
            connection_string,
        }
    }

    pub async fn try_connect(&self) -> Result<Pool<MySql>, sqlx::Error> {
        MySqlPoolOptions::new().max_connections(5).connect(&self.connection_string).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnectionList {
    #[serde(flatten)]
    pub list: Vec<DatabaseConnection>,
}

impl DatabaseConnectionList {
    pub fn new() -> Self {
        DatabaseConnectionList { list: Vec::new() }
    }
}
