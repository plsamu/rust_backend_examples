extern crate tokio_postgres;

mod constant;
mod mdb;
mod mserver;
mod api;
mod mutils;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    match mserver::init() {
        Ok(()) => println!("Server closed without errors"),
        Err(e) => println!("{}", e),
    }
}
