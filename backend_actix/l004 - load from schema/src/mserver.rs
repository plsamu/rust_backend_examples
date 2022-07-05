use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::ServiceConfig,
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use crate::{mdb, constant};


#[get("/")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!(
        "{}:{}{}",
        req.connection_info().scheme(), // http
        req.connection_info().host(),   // 127.0.0.1:8080
        req.path()                      // /
    );

    mdb::test();
    // let events = mdb::get_event();

   /*  for event in events {
        let id: i32 = event.get(0);
        let name: &str = event.get(1);
        let data: Option<&[u8]> = event.get(2);

        print!("{}", id);
        print!("{}", name);
    } */

    Ok(HttpResponse::Ok().body("Hello world!"))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[actix_rt::main]
pub async fn init() -> std::io::Result<()> {
    // SERVER
    HttpServer::new(|| App::new().configure(configure))
        .bind((constant::server::IP, constant::server::PORT))?
        .run()
        .await
}
