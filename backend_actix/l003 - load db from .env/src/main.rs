extern crate postgres;

mod constant;
mod mdb;
mod mserver;
mod mutils;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // ENV
    let client = mdb::init_client();
}

/*
fn hello_world(name: Option<&str>) -> String {
    match name {
        Some(n) => format!("Hello, World {n}"),
        None => format!("Who are you?"),
    }
}
*/
