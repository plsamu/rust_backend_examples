use actix_web::{web::ServiceConfig, App, HttpServer};

use crate::{mdatabase::mdb, mutil::constant};

use super::api;

#[actix_rt::main]
pub async fn init() -> std::io::Result<()> {
    // DATABASE
    mdb::load_from_schema().await;

    // SERVER
    HttpServer::new(|| App::new().configure(configure))
        .bind((constant::server::IP, constant::server::PORT))?
        .run()
        .await
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(api::index);
}
