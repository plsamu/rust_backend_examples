use std::fs;

use crate::{errors::AppError, mdatabase::mdb::get_conn};

pub async fn insert_event(name: &str) -> Result<u64, AppError> {
    let insert_event_query = fs::read_to_string("./sql/event/insert_event.sql")
        .map_err(|err| AppError::internal_error("insert_event", err))?; // ? will return the AppError

    let conn = get_conn().await?;
    Ok(conn
        .execute(&insert_event_query, &[&name, &"".as_bytes()])
        .await
        .map_err(|err| AppError::db_error("insert_event", err))?)
}

pub async fn get_events() -> Result<Vec<tokio_postgres::Row>, AppError> {
    let get_events_query = fs::read_to_string("./sql/event/get_events.sql")
        .map_err(|err| AppError::internal_error("get_events", err))?; // ? will return the AppError

    let conn = get_conn().await?;
    let rows = conn
        .query(&get_events_query, &[])
        .await
        .map_err(|err| AppError::db_error("get_events", err))?;

    for row in &rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found event: {} {} {:?}", id, name, data);
    }
    Ok(rows)
}
