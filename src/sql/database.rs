use crate::application::StoreAction;
use sqlx::{Any, Pool, Row};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct Database;

impl Database {
    pub fn get_databases(pool: Pool<Any>, sender: UnboundedSender<StoreAction>) {
        spawn(async move {
            let query = sqlx::query("SHOW databases").fetch_all(&pool).await;

            match query {
                Ok(rows) => {
                    let res: Vec<_> = rows
                        .iter()
                        .map(|row| row.try_get("Database").unwrap())
                        .collect();
                    sender
                        .send(StoreAction::SendDatabaseData(res.clone()))
                        .unwrap();
                }
                Err(e) => {
                    sender
                        .send(StoreAction::SendError(format!("{:?}", e)))
                        .unwrap();
                }
            };
        });
    }
}
