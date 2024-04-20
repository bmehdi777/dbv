use crate::application::StoreAction;
use sqlx::{Any, Pool, Row};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct Database;

impl Database {
    pub fn get_databases(pool: Pool<Any>, sender: UnboundedSender<StoreAction>) {
        spawn(async move {
            let rows = sqlx::query("SHOW databases")
                .fetch_all(&pool)
                .await
                .unwrap();
            sender
                .send(StoreAction::SendDatabaseData(
                    rows.iter()
                        .map(|row| row.try_get("Database").unwrap())
                        .collect(),
                ))
                .unwrap();
        });
    }
}
