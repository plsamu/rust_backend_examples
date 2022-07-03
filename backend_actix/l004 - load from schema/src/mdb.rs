use postgres::{Client, NoTls};
use std::{env, fs};

fn get_conn() -> Client {
    let dbname: String = env::var("DB_NAME").unwrap();
    let user: String = env::var("DB_USER").unwrap();
    let password: String = env::var("DB_PWD").unwrap();
    let host: String = env::var("DB_HOST").unwrap();
    let port_str: String = env::var("DB_PORT").unwrap();
    let port: u16 = port_str.parse::<u16>().unwrap();
    Client::configure()
        .dbname(&dbname)
        .user(&user)
        .password(&password)
        .host(&host)
        .port(port)
        .connect(NoTls) // the connection
        .unwrap()
}

pub fn load_from_schema() {
    let schema = fs::read_to_string("./sql/schema.sql").expect("Unable to read file");
    let mut conn = get_conn();
    conn.batch_execute(&schema).unwrap();
    conn.close().unwrap();
}
