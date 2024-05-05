use crate::application::{UpdateAction, StoreAction};
use sqlx::{Any, Pool, Row};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct DatabaseList {
    pub list: Vec<String>,
    pub current_database: Option<usize>,
}

impl DatabaseList {
    pub fn new() -> Self {
        DatabaseList {
            list: Vec::new(),
            current_database: None,
        }
    }
}

pub struct Database;

impl Database {
    pub fn get_databases(pool: Pool<Any>, sender: UnboundedSender<UpdateAction>) {
        spawn(async move {
            let query = sqlx::query("SHOW databases").fetch_all(&pool).await;

            match query {
                Ok(rows) => {
                    let res: Vec<_> = rows
                        .iter()
                        .map(|row| row.try_get("Database").unwrap())
                        .collect();
                    sender
                        .send(UpdateAction::SendStoreAction(StoreAction::SendDatabaseData(res.clone())))
                        .unwrap();
                }
                Err(e) => {
                    sender
                        .send(UpdateAction::SendStoreAction(StoreAction::SendError(format!("{:?}", e))))
                        .unwrap();
                }
            };
        });
    }
}
