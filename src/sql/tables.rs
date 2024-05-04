use crate::application::StoreAction;
use sqlx::{Any, Pool, Row};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct Tables;

impl Tables {
    pub fn get_tables(pool: Pool<Any>, sender: UnboundedSender<StoreAction>, db_name: String) {
        spawn(async move {
            let query = sqlx::query(&format!(
                "select table_name from information_schema.tables where table_schema='{}'",
                db_name
            ))
            .fetch_all(&pool)
            .await;

            match query {
                Ok(rows) => {
                    sender
                        .send(StoreAction::SendTablesData(
                            rows.iter()
                                .map(|row| row.try_get("table_name").unwrap())
                                .collect(),
                        ))
                        .unwrap();
                }
                Err(e) => {
                    sender
                        .send(StoreAction::SendError(format!("{:?}", e)))
                        .unwrap();
                }
            }
        });
    }
}
