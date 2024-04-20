use crate::application::StoreAction;
use sqlx::{Any, Pool, Row};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct Tables;

impl Tables {
    pub fn get_tables(pool: Pool<Any>, sender: UnboundedSender<StoreAction>, db_name: String) {
        spawn(async move {
            let rows = sqlx::query(&format!(
                "select table_name from information_schema.tables where table_schema='{}'",
                db_name
            ))
            .fetch_all(&pool)
            .await
            .unwrap();
            sender
                .send(StoreAction::SendTablesData(
                    rows.iter()
                        .map(|row| row.try_get("table_name").unwrap())
                        .collect(),
                ))
                .unwrap();
        });
    }
}
