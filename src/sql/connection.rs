use serde::{Deserialize, Serialize};
use sqlx::{
    any::{Any, AnyPoolOptions, AnyRow},
    Pool,
};

pub enum SqlThread {
    TableRow(AnyRow),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    #[serde(rename = "connectionString")]
    pub connection_string: String,

    #[serde(skip_serializing, skip_deserializing)]
    pub pool: Option<Pool<Any>>,
}

impl Connection {
    pub fn new(connection_string: String) -> Self {
        Connection {
            connection_string,
            pool: None,
        }
    }

    pub fn set_pool(&mut self) -> Result<(), sqlx::Error> {
        self.pool = Some(
            AnyPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(std::time::Duration::new(5, 0))
                .connect_lazy(&self.connection_string)?,
        );

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionList {
    #[serde(rename = "connectionList")]
    pub list: Vec<Connection>,

    #[serde(skip_serializing, skip_deserializing)]
    pub current_connection: Option<usize>,
    #[serde(skip_serializing, skip_deserializing)]
    pub is_loading: bool,
}

impl ConnectionList {
    pub fn new() -> Self {
        ConnectionList {
            list: Vec::new(),
            current_connection: None,
            is_loading: false,
        }
    }

    pub fn get_pool(&self) -> anyhow::Result<Pool<Any>> {
        if let Some(i) = self.current_connection {
            return Ok(self.list[i]
                .pool
                .as_ref()
                .expect("An error occured while getting the pool.")
                .clone());
        }
        Err(anyhow::anyhow!("No current connection is set."))
    }

    pub fn set_current_connection(&mut self, index: usize) -> Result<(), String> {
        self.current_connection = Some(index);
        if let Err(e) = self.list[index].set_pool() {
            self.current_connection = None;
            return Err(format!("{}", e));
        }
        Ok(())
    }

    pub fn reset_current_connection(&mut self) {
        self.current_connection = None;
    }
}
