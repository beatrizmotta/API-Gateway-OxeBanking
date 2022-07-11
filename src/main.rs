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

#[derive(Debug, Serialize, Deserialize)]
struct Todo{
    userId: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}


/// extract `Info` using serde
async fn index(info: web::Json<Info>) -> Result<impl Responder> {

    let todos= reqwest::Client::new()
    .get("https://jsonplaceholder.typicode.com/todos?userId=1")
    .send()
    .await
    .expect("Failed to fetch")
    .json::<Vec<Todo>>()
    .await;

    println!("{:#?}", todos);

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