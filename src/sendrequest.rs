use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use std::collections::HashMap;
use futures;
use reqwest::{StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    function: String,
    arguments: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Upper {
    results: String,
}



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



async fn callapi(data: Request) {

    // Aqui estou pegando os dados 
    let mut map = HashMap::new();
    map.insert("function", data.function.to_string());
    map.insert("arguments", format!("{:?}", data.arguments));

    let client = reqwest::Client::new();

    // let res = client.post("http://127.0.0.1:3000/bia")
    //     .json(&map)
    //     .send()
    //     .await
    //     .expect("failed")
    //     .json::<String>()
    //     .await;

        match client.post("http://127.0.0.1:3000/bia")
        .json(&map)
        .send()
        .await
        .expect("failed")
        .json::<String>()
        .await {
            Ok(res) => {
                println!("aaaaa");
                println!("{}", res);
            },
            Err(e) => {
                println!("There has been an error");
            }
        }

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