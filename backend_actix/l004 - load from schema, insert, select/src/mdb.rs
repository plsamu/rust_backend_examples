use std::{env, fs, ptr::null};
use tokio_postgres::{Client, NoTls};

async fn get_conn() -> Client {
    let dbname: String = env::var("DB_NAME").expect("Please set db name in .env");
    let user: String = env::var("DB_USER").expect("Please set user in .env");
    let password: String = env::var("DB_PWD").expect("Please set password in .env");
    let host: String = env::var("DB_HOST").expect("Please set host in .env");
    let port_str: String = env::var("DB_PORT").expect("Please set port in .env");
    // let port: u16 = port_str.parse::<u16>().unwrap();

    let placeholder =
        "dbname={DB_NAME} user={DB_USER} password={DB_PWD} host={DB_HOST} port={DB_PORT}";

    let conn_string = placeholder
        .replace("{DB_NAME}", &dbname)
        .replace("{DB_USER}", &user)
        .replace("{DB_PWD}", &password)
        .replace("{DB_HOST}", &host)
        .replace("{DB_PORT}", &port_str);

    let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

pub async fn load_from_schema() {
    let schema = fs::read_to_string("./sql/schema.sql").expect("Unable to read file");
    let conn = get_conn().await;
    conn.batch_execute(&schema).await.unwrap();
}

pub fn test() {
    let mut conn = get_conn();
}

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
