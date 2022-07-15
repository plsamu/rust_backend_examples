use actix_files::{Files, NamedFile};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::ServiceConfig,
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

#[get("/test")]
async fn test(req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body("test opened"))
}

#[get("/")]
async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    println!(
        "{}:{}{}",
        req.connection_info().scheme(), // http
        req.connection_info().host(),   // 127.0.0.1:8080
        req.path()                      // /
    );
    Ok(NamedFile::open("./static/index.html").unwrap())
}

// .service(fs::Files::new("/", "./static").show_files_listing()) --> show all files inside the dir
pub fn configure(cfg: &mut ServiceConfig) {
    cfg
    .service(index) // this first
    .service(Files::new("/", "./static"));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("creating server");
    HttpServer::new(|| App::new().configure(configure))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
