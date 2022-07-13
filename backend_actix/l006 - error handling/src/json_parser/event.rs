use actix_web::web::{self, Json};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    id: i32,
    name: String,
    data: Vec<u8>,
}

pub fn get_json(event_rows: Vec<Row>) -> Json<Vec<Event>> {
    let mut output = Vec::with_capacity(event_rows.len());
    for i in 0..event_rows.len() {
        let row = event_rows.get(i).unwrap();
        output.insert(
            output.len(),
            Event {
                id: row.get(0),
                name: row.get(1),
                data: row.get(2),
            },
        );
    }
    actix_web::web::Json(output)
}
