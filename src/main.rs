use actix_web::{post, get, web, Responder, Result, HttpResponse, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Serialize)]
struct Test{
    nome: String,
} 


/// extract `Info` using serde
async fn index(info: web::Json<Info>) -> Result<impl Responder> {

    let obj = Test{
        nome: info.username.to_string(),
    };
    
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}