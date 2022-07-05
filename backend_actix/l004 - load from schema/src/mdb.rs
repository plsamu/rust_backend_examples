use postgres::{Client, NoTls};
use std::{env, fs};

fn get_conn() -> Client {
    let dbname: String = env::var("DB_NAME").expect("Please set db name in .env");
    let user: String = env::var("DB_USER").expect("Please set user in .env");
    let password: String = env::var("DB_PWD").expect("Please set password in .env");
    let host: String = env::var("DB_HOST").expect("Please set host in .env");
    let port_str: String = env::var("DB_PORT").expect("Please set port in .env");
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

pub fn test() {
    let mut conn = get_conn();
    conn.close();
}

pub fn get_event() -> Vec<postgres::Row>{
    let mut conn = get_conn();

    let name = "Ferris";
    let data = None::<&[u8]>;
    conn.execute(
        "INSERT INTO event (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).unwrap();
    
    /* for row in conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);
    
        println!("found event: {} {} {:?}", id, name, data);
    } */

    let rows = conn.query("SELECT id, name, data FROM person", &[]).unwrap();
    conn.close().unwrap();
    rows
}