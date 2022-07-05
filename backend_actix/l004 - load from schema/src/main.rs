extern crate postgres;

mod constant;
mod mdb;
mod mserver;
mod mutils;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // ENV
    mdb::load_from_schema();
    match mserver::init() {
        Ok(()) => println!("Server closed without errors"),
        Err(e) => println!("{}", e)
    }
}
