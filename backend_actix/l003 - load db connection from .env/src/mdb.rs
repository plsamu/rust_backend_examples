use postgres::{Client, NoTls};
use std::{env};

pub fn init_client() -> Client {
    let dbname = env::var("DB_NAME").unwrap();
    let user = env::var("DB_USER").unwrap();
    let password = env::var("DB_PWD").unwrap();
    let host = env::var("DB_HOST").unwrap();
    let port = env::var("DB_PORT").unwrap();

    let placeholder =
        "dbname={DB_NAME} user={DB_USER} password={DB_PWD} host={DB_HOST} port={DB_PORT}";

    let conn_string = placeholder
        .replace("{DB_NAME}", &dbname)
        .replace("{DB_USER}", &user)
        .replace("{DB_PWD}", &password)
        .replace("{DB_HOST}", &host)
        .replace("{DB_PORT}", &port);

    //DATABASE
    // let client = Client::connect("postgresql://postgres@127.0.0.1:6543", NoTls).unwrap();

    Client::connect(&conn_string, NoTls).unwrap()
}
