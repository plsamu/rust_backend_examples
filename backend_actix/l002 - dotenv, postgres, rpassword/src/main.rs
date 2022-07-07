extern crate postgres;

mod constant;
mod mdb;
mod mserver;
mod mutils;

use std::env;

use dotenv::dotenv;

fn main() {
    // ENV
    dotenv().ok();
    println!("{}", env::var("CIAO").unwrap());

    let _client = mdb::init_client();
}