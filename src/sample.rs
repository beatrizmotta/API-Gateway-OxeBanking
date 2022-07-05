use actix_web::{post, web, HttpResponse, Responder};
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


#[post("/sample")]
pub async fn test(req_body: String) -> impl Responder {
    let r: Request  = serde_json::from_str(&req_body).unwrap();
   
    println!("{}", r.function);
    println!("{:?}", r.arguments);
    
    let mut vec:Vec<Response> = Vec::new();
    vec.push(Response{message: "All ok!".to_string()});

    HttpResponse::Ok().body(serde_json::to_vec(&vec).unwrap())
}