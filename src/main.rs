use actix_web::{post, get, web, Responder, Result, HttpResponse, App, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use reqwest::{self};


#[derive(Deserialize, Serialize)]
struct create_transfer {
    docClienteOrigem: i32,
    docClienteDestino: i32,
    nomeClienteOrigem: String,
    nomeClienteDestino: String,
    bancoOrigem: String,
    bancoDestino: String,
    ValorTransf: i32,
    DataHora: String,
}

#[derive(Serialize)]
struct Test{
    nome: String,
} 


/// extract `Info` using serde
async fn createTransfer(info: web::Json<create_transfer>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("docClienteOrigem", info.docClienteOrigem.to_string());
    map.insert("docClienteDestino", info.docClienteDestino.to_string());
    map.insert("nomeClienteOrigem", info.nomeClienteOrigem.to_string());
    map.insert("nomeClienteDestino", info.nomeClienteDestino.to_string());
    map.insert("bancoOrigem", info.bancoOrigem.to_string());
    map.insert("bancoDestino", info.bancoDestino.to_string());
    map.insert("ValorTransf", info.ValorTransf.to_string());
    map.insert("DataHora", info.DataHora.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/create_transfer:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        nome: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/createTransfer", web::post().to(createTransfer)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}