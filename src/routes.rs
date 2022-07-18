use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder, Result};
use std::{collections::HashMap, path};
use serde_json::json;
mod structs;

// Login Route
#[post("/login")]
pub async fn getuser(req_body: String) -> impl Responder {
    let data: structs::UserData = serde_json::from_str(&req_body).unwrap();
    let user_email: String = data.email;
    let user_password: String = data.password;
    
    let mut vec:Vec<structs::ResponseMessage> = Vec::new();
    
    let client = reqwest::Client::new();
    match client.get("https://oxebanking-accounting.herokuapp.com/users")
    .send()
    .await
    .expect("Failed to fetch")
        .json::<Vec<structs::AccountDetails>>()
        .await {
            Ok(res) => {
                let mut found: bool = false;
                let mut correct_password: bool = false;
                for i in res {
                    if i.email == user_email {
                        found = true; 
                        if i.password == user_password {
                            correct_password = true; 
                        }
                        break;
                    } 
                }

                if found == true {
                    if correct_password == true {
                        vec.push(
                            structs::ResponseMessage {
                                message: "Tudo certo.".to_string(), 
                                code: 200, 
                                logged: "true".to_string()
                        });
                    } else {
                        vec.push(
                            structs::ResponseMessage {
                                message: "Senha incorreta.".to_string(), 
                                code: 200, 
                                logged: "false".to_string()
                        });
                    }
                } else {
                    vec.push(
                        structs::ResponseMessage {
                            message: "Não existe usuário cadastrado com essas credenciais.".to_string(),
                            code: 200, 
                            logged: "false".to_string()
                    });
                }
            },
            Err(erro) => {
                vec.push(
                    structs::ResponseMessage {
                        message: erro.to_string(),
                        code: 500, 
                        logged: "false".to_string()
                    }
                )  
            }   
        }
        
        return web::Json(vec);
}

//Transfer Route
pub async fn updateTransferTedDoc(info: web::Json<structs::update_transfer_ted_doc>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("conta", info.conta.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn createTransferTedDoc(info: web::Json<structs::create_transfer_ted_doc>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("agencia", info.agencia.to_string());
    map.insert("conta", info.conta.to_string());
    map.insert("transfer_id", info.transfer_id.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn updateTransferPix(info: web::Json<structs::update_transfer_pix>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());
    map.insert("chave", info.chave.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn createTransferPix(info: web::Json<structs::create_transfer_pix>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("transfer_id", info.transfer_id.to_string());
    map.insert("chave", info.chave.to_string());
    map.insert("tipo", info.tipo.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}

pub async fn deleteTransfer(info: web::Json<structs::update_transfer>) -> Result<impl Responder> {

    let mut map = HashMap::new();

    map.insert("id", info.id.to_string());

    let res = reqwest::Client::new()
    .post("https://127.0.0.1/api/update_transfer/id:4000")
    .json(&map)
    .send()
    .await;

    let obj = structs::Test {
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}


pub async fn updateTransfer(info: web::Json<structs::update_transfer>) -> Result<impl Responder> {

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

    let obj = structs::Test{
        status: "OK".to_string(),
    };
    
    Ok(web::Json(obj))
}