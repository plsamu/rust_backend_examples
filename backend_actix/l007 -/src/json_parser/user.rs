use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
}

pub fn get_json(rows: Vec<Row>) -> Json<Vec<User>> {
    let mut output = Vec::with_capacity(rows.len());
    for i in 0..rows.len() {
        let row = rows.get(i).unwrap();
        output.insert(
            output.len(),
            User {
                user_id: row.get(0),
            },
        );
    }
    actix_web::web::Json(output)
}