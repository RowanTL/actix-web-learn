use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, body::BoxBody, get,
    http::header::ContentType, web,
};
use futures::{future::ok, stream::once};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/")]
async fn index() -> impl Responder {
    MyObj { name: "user" }
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(stream))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
