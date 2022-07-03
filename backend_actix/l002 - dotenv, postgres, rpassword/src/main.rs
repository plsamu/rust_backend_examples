/**
 * dotenv
 * postgres
 */
extern crate postgres;
mod constant;
use std::{env, io::Write};

use postgres::{Client, NoTls};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::ServiceConfig,
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use rpassword::read_password;

#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!(
        "{}:{}{}",
        req.connection_info().scheme(), // http
        req.connection_info().host(),   // 127.0.0.1:8080
        req.path()                      // /
    );
    Ok(HttpResponse::Ok().body("Hello world!"))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    // ENV
    dotenv().ok();
    println!("{}", env::var("CIAO").unwrap());

    print!("Type password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    //DATABASE
    let client = Client::connect("postgresql://postgres@127.0.0.1:6543", NoTls)
            .unwrap();

    // SERVER
    HttpServer::new(|| App::new().configure(configure))
        .bind((constant::server::IP, constant::server::PORT))?
        .run()
        .await
}
