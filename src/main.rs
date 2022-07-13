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

#[derive(Deserialize, Serialize)]
struct update_transfer {
    id: i32,
    bancoDestino: String,
    ValorTransf: i32,
    DataHora: String,
}

#[derive(Deserialize, Serialize)]
struct delete_transfer {
    id: i32,
}

#[derive(Deserialize, Serialize)]
struct create_transfer_pix {
    chave: String,
    tipo: String,
    transfer_id: i32,
}

#[derive(Deserialize, Serialize)]
struct update_transfer_pix {
    id: i32,
    chave: String,
}

#[derive(Deserialize, Serialize)]
struct create_transfer_ted_doc {
    agencia: String,
    conta: String,
    transfer_id: i32,
}

#[derive(Deserialize, Serialize)]
struct update_transfer_ted_doc {
    id: i32,
    conta: String,
}

#[derive(Serialize)]
struct Test{
    status: String,
} 

async fn updateTransferTedDoc(info: web::Json<update_transfer_ted_doc>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("conta", info.conta.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

async fn createTransferTedDoc(info: web::Json<create_transfer_ted_doc>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("agencia", info.agencia.to_string());
    map.insert("conta", info.conta.to_string());
    map.insert("transfer_id", info.transfer_id.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

async fn updateTransferPix(info: web::Json<update_transfer_pix>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("chave", info.chave.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

async fn createTransferPix(info: web::Json<create_transfer_pix>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("transfer_id", info.transfer_id.to_string());
    map.insert("chave", info.chave.to_string());
    map.insert("tipo", info.tipo.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

async fn deleteTransfer(info: web::Json<update_transfer>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

async fn updateTransfer(info: web::Json<update_transfer>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("bancoDestino", info.bancoDestino.to_string());
    map.insert("ValorTransf", info.ValorTransf.to_string());
    map.insert("DataHora", info.DataHora.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

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
        status: "OK".to_string(),
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