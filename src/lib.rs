use std::net::TcpListener;
use actix_web::{get, post, App, HttpServer, Responder, HttpResponse, web};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
        .listen(listener)?
        .run();

    Ok(server)
}