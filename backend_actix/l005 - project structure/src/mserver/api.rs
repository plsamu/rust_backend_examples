use actix_web::{get, Error, HttpRequest, HttpResponse};

use crate::mdatabase::{mdb, crud::event::get_event};

#[get("/")]
pub async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    let rows = get_event().await;
    Ok(HttpResponse::Ok().body("Hello world!"))
}
