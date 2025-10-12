use std::str::FromStr;

use actix_files::NamedFile;
use actix_web::{App, HttpServer, Responder, Result, get, http::StatusCode, web};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

// To experiment with the different responders
#[get("/a/{name}")]
async fn a_name(name: web::Path<String>) -> (impl Responder, StatusCode) {
    let obj = MyObj {
        name: name.to_string(),
    };
    (obj.name, StatusCode::from_str("500").unwrap())
}

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await // impl Responder can take a io::Result and return it :)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
