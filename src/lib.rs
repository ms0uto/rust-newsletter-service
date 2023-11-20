use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route(
            "/health_check",
            web::get().to(|| async { HttpResponse::Ok() }),
        )
    })
    .bind("127.0.0.1:8000")?
    .run();
    Ok(server)
}
