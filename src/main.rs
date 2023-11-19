use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/", web::get().to(health)))
        .bind("localhost:8000")?
        .run()
        .await
}
