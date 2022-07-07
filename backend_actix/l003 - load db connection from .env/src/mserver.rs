use actix_web::{
    get,
    web::ServiceConfig,
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use crate::constant;

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

#[actix_rt::main]
pub async fn init() -> std::io::Result<()> {
    // SERVER
    HttpServer::new(|| App::new().configure(configure))
        .bind((constant::server::IP, constant::server::PORT))?
        .run()
        .await
}

#[allow(dead_code)]
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index);
}
