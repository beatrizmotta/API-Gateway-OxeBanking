use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
mod routes;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    // let HOST = env::var("HOST").expect("Host not set");
    // let PORT = env::var("PORT").expect("Port not set");
    
    let HOST = "127.0.0.1";
    let PORT = "8080";

    println!("Project running!");

    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(routes::getuser)
            // .route("/createTransfer", web::post().to(routes::createTransfer))
            .route("/updateTransfer", web::post().to(routes::updateTransfer))
            .route("/deleteTransfer", web::post().to(routes::deleteTransfer))
            .route("/createTransferPix", web::post().to(routes::createTransferPix))
            .route("/updateTransferPix", web::post().to(routes::updateTransferPix))
            .route("/createTransferTedDoc", web::post().to(routes::createTransferTedDoc))
            .route("/updateTransferTedDoc", web::post().to(routes::updateTransferTedDoc))
    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}