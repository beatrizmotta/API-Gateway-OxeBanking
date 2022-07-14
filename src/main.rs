use actix_web::{get, web, App, HttpServer, Responder};
use std::net::SocketAddr;
mod routes;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    let port = std::env::var("PORT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(routes::getuser)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}