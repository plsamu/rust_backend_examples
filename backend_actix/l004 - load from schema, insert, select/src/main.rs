extern crate tokio_postgres;

mod constant;
mod mdb;
mod mserver;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // ENV
    match mserver::init() {
        Ok(()) => println!("Server closed without errors"),
        Err(e) => println!("{}", e),
    }
}
