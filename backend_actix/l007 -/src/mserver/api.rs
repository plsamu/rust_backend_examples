use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web::Json, Error, HttpRequest};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    errors::AppError,
    json_parser::{self, event::Event, user::User},
    mdatabase::{crud, mdb},
};

#[get("/")]
pub async fn index(req: HttpRequest) -> Result<Json<Vec<Event>>, Error> {
    crud::event::insert_event("21 luglio biglietti flixbus").await?; // with ?, this will return an AppError
    let rows = crud::event::get_events().await?;
    Ok(json_parser::event::get_json(rows))
}

#[get("/subscriptions")]
pub async fn subscriptions(req: HttpRequest) -> Result<Json<Vec<User>>, Error> {
    let conn = mdb::get_conn().await?;
    let rows = conn
        .query("SELECT * from my_schema.user;", &[])
        .await
        .map_err(|err| AppError::db_error("get_events", err))?;
    Ok(json_parser::user::get_json(rows))
}

#[derive(Serialize, Debug)]
pub struct UserId {
    user_id: String,
}

#[get("/register")]
pub async fn register(req: HttpRequest) -> Result<Json<UserId>, Error> {
    println!("{:?}", chrono::offset::Local::now()); // 2022-07-15T01:05:39.028665800+02:00
    println!("{:?}", chrono::offset::Utc::now()); // 2022-07-14T23:05:39.029221200Z
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let uuid4_tmp = Uuid::new_v4();
    let new_id = now.as_millis().to_string() + "-" + &uuid4_tmp.to_string(); // create user_id
    let output = actix_web::web::Json(User {
        user_id: new_id.to_string(),
    });
    crud::user::insert_user(&output).await?; // insert User object in db
    Ok(actix_web::web::Json(UserId { user_id: new_id }))
}
