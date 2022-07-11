use actix_web::{post, get, web, Responder, Result, HttpResponse, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};


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

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        // go check out her latest album. It's 🔥
        query = "Little Simz"
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        // confirm the request using send()
        .header(AUTHORIZATION, "Bearer BQA9BfYoX_yElgUrqI1V3RU404_z2FC-Fbf7J7eBD6GUVEvGRRaDYg3UWkoP6JaNaceydh60v0ZFirm7GTC0HOI5i-sVKOmLeH1s0p1bcutTuQmv6023scqYvzrd7jJ9lkq2vxfvYSODjpQZtB7dEfTAJPPFvxK8wFul")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);

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