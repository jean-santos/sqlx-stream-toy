use std::time::Instant;

use crate::{
    stream::get_stream,
    transaction::{DatabaseTransaction, Statement},
};
use futures::StreamExt;
use sqlx::{Row, SqlitePool};

pub async fn async_query(query: &str, pool: &SqlitePool) -> anyhow::Result<()> {
    let stat = Statement { sql: query.into() };

    let start = Instant::now();
    let mut first_read = false;

    let database_transaction = DatabaseTransaction::new(pool).await;

    let mut stream = get_stream(&database_transaction, stat).await.unwrap();

    while let Some(item) = stream.next().await {
        match item {
            Ok(row) => {
                let id: i32 = row.try_get("id")?;
                if !first_read {
                    first_read = true;
                    println!("first read at {:?} with id {:?}", start.elapsed(), id);
                }
            }
            Err(_e) => {}
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}
