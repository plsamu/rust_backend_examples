extern crate tokio_postgres;

mod mdatabase;
mod mserver;
mod mutil;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    match mserver::mserver::init() {
        Ok(()) => println!("Server closed without errors"),
        Err(e) => println!("{}", e),
    }
}
