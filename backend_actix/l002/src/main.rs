/**
 * dotenv
 * postgres
 */
mod constant;
use std::env;

use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::ServiceConfig,
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use dotenv::dotenv;

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
    dotenv().ok();
    println!("{}", env::var("CIAO").unwrap());
    HttpServer::new(|| App::new().configure(configure))
        .bind((constant::server::IP, constant::server::PORT))?
        .run()
        .await
}
