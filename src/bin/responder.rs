use std::str::FromStr;

use actix_web::{App, HttpServer, Responder, Result, get, http::StatusCode, web};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> (impl Responder, StatusCode) {
    let obj = MyObj {
        name: name.to_string(),
    };
    (obj.name, StatusCode::from_str("500").unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
