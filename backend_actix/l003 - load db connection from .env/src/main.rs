extern crate postgres;

mod constant;
mod mdb;
mod mserver;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // ENV
    let _client = mdb::init_client();
}

