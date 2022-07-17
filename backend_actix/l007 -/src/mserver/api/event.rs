use actix_web::{post, Error, HttpRequest};

use crate::{json_parser::user::User};

/*

    curl --location --request POST 'localhost:8080/event'

*/

#[post("/event")]
pub async fn post_event(req: HttpRequest) -> Result<String, Error> {
    warn!("/event");
    actix_web::web::Json(User {
        user_id: "".to_string(),
    });
    Ok("".to_string())
}
