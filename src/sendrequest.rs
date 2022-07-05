use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
// use std::error::Error;
use std::collections::HashMap;
use reqwest::{StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    function: String,
    arguments: Vec<String>,
}

#[derive(Serialize)]
pub struct Response {
    message: String,
}

async fn callapi(data: Request) {

    let mut map = HashMap::new();
    map.insert("function", data.function.to_string());
    map.insert("arguments", format!("{:?}", data.arguments));

    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3000/bia")
        .json(&map)
        .send()
        .await; 

        // println!("{:?}", res)
}

#[post("/sample")]
pub async fn sample(req_body: String) -> impl Responder {
    let r: Request = serde_json::from_str(&req_body).unwrap();
   
    println!("{}", r.function);
    println!("{:?}", r.arguments);
    
        
    let mut vec:Vec<Response> = Vec::new(); 
    vec.push(Response{message: "Tudo ok".to_string()});

    callapi(r).await;

    HttpResponse::Ok().body(serde_json::to_vec(&vec).unwrap())
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(sample)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await 
}