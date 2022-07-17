use std::fs;

use crate::{errors::AppError, json_parser::user::User, mdatabase};

/*
pub struct User {
    user_id: String,
}
*/

pub async fn insert_user(user: &User) -> Result<u64, AppError> {
    let insert = fs::read_to_string("./sql/user/insert_user.sql")
        .map_err(|err| AppError::internal_error("insert_user", err))?; // ? will return the AppError

    let conn = mdatabase::mdb::get_conn().await?;

    Ok(conn
        .execute(&insert, &[&user.user_id])
        .await
        .map_err(|err| AppError::db_error("insert_user", err))?)
}
