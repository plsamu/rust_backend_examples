use std::{env, fs, ptr::null};
use tokio_postgres::{Client, NoTls};

use crate::errors::{AppError, AppErrorType};

pub async fn get_conn() -> Result<Client, AppError> {
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

    let (client, connection) = tokio_postgres::connect(&conn_string, NoTls)
        .await
        .map_err(|err| AppError::internal_error("get_conn", err))?; // ? will return the AppError

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

/**
 * ========================================
 * BAD example of AppError use
 *  - this is not an API request
 *  - this code come before the server is ON
 *  - just use a normal Result<(), Error>
 * ========================================
 */
pub async fn load_from_schema() {
    let schema = fs::read_to_string("./sql/schema.sql").map_err(|err| AppError {
        message: Some("unable to read file".to_string()),
        cause: Some(err.to_string()),
        error_type: AppErrorType::DbError,
    });

    match schema {
        Ok(schema) => {
            let conn = get_conn().await.unwrap();
            conn.batch_execute(&schema).await.unwrap();
        }
        Err(err) => println!("{}", err),
    }
}

pub fn test() {
    let mut conn = get_conn();
}
