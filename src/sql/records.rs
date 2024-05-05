use crate::application::{UpdateAction, StoreAction};
use sqlx::{Any, Pool};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct Records;

impl Records {
    pub fn get_all(pool: Pool<Any>, sender: UnboundedSender<UpdateAction>,db_name: String, table_name: String) {
        spawn(async move {
            let query = sqlx::query(&format!("select * from {}.{}", db_name, table_name))
                .fetch_all(&pool)
                .await;

            match query {
                Ok(rows) => {
                    sender.send(UpdateAction::SendStoreAction(StoreAction::SendRecordsData(rows))).unwrap();
                }
                Err(e) => {
                    sender
                        .send(UpdateAction::SendStoreAction(StoreAction::SendError(format!("{:?}", e))))
                        .unwrap();
                }
            }
        });
    }
}
