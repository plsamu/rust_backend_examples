use std::fs;

use crate::mdatabase::mdb::get_conn;

pub async fn insert_event(name: &str) -> u64 {
    let insert_event_query =
        fs::read_to_string("./sql/event/insert_event.sql").expect("Unable to read file");
    let conn = get_conn().await;
    conn.execute(&insert_event_query, &[&name, &"".as_bytes()])
        .await
        .unwrap()
}

pub async fn get_events() -> Vec<tokio_postgres::Row> {
    let get_events_query =
        fs::read_to_string("./sql/event/get_events.sql").expect("Unable to read file");
    let conn = get_conn().await;

    let rows = conn.query(&get_events_query, &[]).await.unwrap();

    for row in &rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found event: {} {} {:?}", id, name, data);
    }
    rows
}
