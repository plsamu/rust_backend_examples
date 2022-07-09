use std::fs;

use crate::mdatabase::mdb::get_conn;

pub async fn get_event() -> Vec<tokio_postgres::Row> {
    let get_event_query = fs::read_to_string("./sql/get_event.sql").expect("Unable to read file");
    let conn = get_conn().await;

    let rows = conn.query(&get_event_query, &[]).await.unwrap();

    for row in &rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found event: {} {} {:?}", id, name, data);
    }

    conn.execute(
        "INSERT INTO event (name, data) VALUES ($1, $2)",
        &[&"evento_1", &"".as_bytes()],
    )
    .await
    .unwrap();

    rows
}
