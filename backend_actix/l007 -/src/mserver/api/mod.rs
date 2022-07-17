pub mod event;
pub mod user;

use crate::{
    json_parser::{self, event::Event},
    mdatabase::crud,
};
use actix_web::{get, web::Json, Error, HttpRequest};

#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Json<Vec<Event>>, Error> {
    crud::event::insert_event("21 luglio biglietti flixbus").await?; // with ?, this will return an AppError
    let rows = crud::event::get_events().await?;
    Ok(json_parser::event::get_json(rows))
}
