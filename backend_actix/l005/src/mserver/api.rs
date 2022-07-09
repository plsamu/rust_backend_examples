use actix_web::{get, Error, HttpRequest, HttpResponse};

use crate::mdb;

#[get("/")]
pub async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    let rows = mdb::get_event().await;
    Ok(HttpResponse::Ok().body("Hello world!"))
}
