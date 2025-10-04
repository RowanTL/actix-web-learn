use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

#[derive(Clone)]
struct ArbitraryBullShit {
    bullshit: usize,
}

async fn index(data: web::Data<ArbitraryBullShit>) -> String {
    format!("Request number: {}\n", data.bullshit) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bs = web::Data::new(ArbitraryBullShit { bullshit: 1 });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(bs.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
