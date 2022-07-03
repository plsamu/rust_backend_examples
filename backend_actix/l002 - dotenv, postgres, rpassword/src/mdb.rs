use postgres::{Client, NoTls};
use rpassword::read_password;
use std::{env, io::Write};

use crate::mutils::concat_and_consume;

pub fn init_client() -> Client {
    print!("Type password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    let placeholder = "host=127.0.0.1 user=postgres port=6543 dbname=postgres password=".to_owned();
    let conn_string = concat_and_consume(placeholder, password);

    //DATABASE
    // let client = Client::connect("postgresql://postgres@127.0.0.1:6543", NoTls).unwrap();

    Client::connect(&conn_string, NoTls).unwrap()
}
