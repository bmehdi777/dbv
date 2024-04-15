use sqlx::{
    any::{Any, AnyPoolOptions, AnyRow},
    Pool,
};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub enum SqlThread {
    TableRow(AnyRow),
}

#[derive(Debug)]
pub struct DatabaseConnection {
    pub connection_string: String,
    pool: Option<Pool<Any>>,
}

impl DatabaseConnection {
    pub fn new(connection_string: String) -> Self {
        DatabaseConnection {
            connection_string,
            pool: None,
        }
    }

    pub fn try_establish_connection(&mut self) -> Result<(), sqlx::Error> {
        self.pool = Some(
            AnyPoolOptions::new()
                .max_connections(5)
                .connect_lazy(&self.connection_string)?,
        );

        Ok(())
    }
}

#[derive(Debug)]
pub struct DatabaseConnectionList {
    pub list: Vec<DatabaseConnection>,
    tx: Sender<SqlThread>,
    rx: Receiver<SqlThread>,
}

impl DatabaseConnectionList {
    pub fn new() -> Self {
        let (tx, rx) = channel::<SqlThread>();
        DatabaseConnectionList {
            list: Vec::new(),
            tx,
            rx,
        }
    }
}
