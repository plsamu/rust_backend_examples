extern crate tokio_postgres;

mod mdatabase;
mod mserver;
mod mutil;
mod json_parser;
mod errors;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    match mserver::init() {
        Ok(()) => println!("Server closed without errors"),
        Err(e) => println!("{}", e),
    }
}
