use crate::application::{AppAction, StoreAction, UpdateAction};
use sqlx::{Any, Pool, Row};
use tokio::{spawn, sync::mpsc::UnboundedSender};

pub struct Records;

impl Records {
    pub fn get_all(
        pool: Pool<Any>,
        sender: UnboundedSender<UpdateAction>,
        db_name: String,
        table_name: String,
    ) {
        spawn(async move {
            let query = sqlx::query(&format!(
                "select * from {}.{} LIMIT 100",
                db_name, table_name
            ))
            .fetch_all(&pool)
            .await;

            let count = sqlx::query(&format!("select COUNT(*) from {}.{}", db_name, table_name))
                .fetch_one(&pool)
                .await;
            match query {
                Ok(rows) => {
                    if let Ok(c) = count {
                        sender
                            .send(UpdateAction::SendAppAction(AppAction::SendRecords((
                                rows,
                                Some(c.get::<i64, _>(0)),
                            ))))
                            .unwrap();
                    } else {
                        sender
                            .send(UpdateAction::SendAppAction(AppAction::SendRecords((
                                rows, None,
                            ))))
                            .unwrap();
                    }
                }
                Err(e) => {
                    sender
                        .send(UpdateAction::SendStoreAction(StoreAction::SendError(
                            format!("{:?}", e),
                        )))
                        .unwrap();
                }
            }
        });
    }
}
