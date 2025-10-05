use actix_web::{
    App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result, body::BoxBody,
    error, get, http::header::ContentType, web,
};
use derive_more::derive::{Display, Error};
use futures::{future::ok, stream::once};
use serde::Serialize;

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

#[derive(Debug, Display, Error)]
#[display("my error: {name}")]
struct MyError {
    name: &'static str,
}

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

#[get("/either/{boolean}")]
async fn either_index(path: web::Path<bool>) -> RegisterResult {
    let boolean: bool = path.into_inner();
    if !boolean {
        // choose Left variant
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        Either::Right(Ok("Hello!"))
    }
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[get("/error")]
async fn error_index() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
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
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(stream)
            .service(either_index)
            .service(error_index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
