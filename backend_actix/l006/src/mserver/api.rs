use actix_web::{
    get,
    web::{self, Json},
    Error, HttpRequest, HttpResponse,
};

use crate::{
    json_parser::{self, event::Event},
    mdatabase::{
        crud::event::{get_events, insert_event},
        mdb,
    },
};

#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Json<Vec<Event>>, Error> {
    insert_event("21 luglio biglietti flixbus").await;
    let rows = get_events().await;
    Ok(json_parser::event::get_json(rows))
}
