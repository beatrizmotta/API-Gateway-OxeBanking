use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder};
mod structs;


#[post("/login")]
pub async fn getuser(req_body: String) -> impl Responder {
    let data: structs::UserData = serde_json::from_str(&req_body).unwrap();
    let user_email: String = data.email;

    // httpresponse.insert("status", "200");
    
    let mut vec:Vec<structs::ResponseMessage> = Vec::new();
    
    let client = reqwest::Client::new();
    match client.get("http://127.0.0.1:3000/users")
    .send()
    .await
    .expect("Failed to fetch")
        .json::<Vec<structs::AccountDetails>>()
        .await {
            Ok(res) => {
                let mut found: bool = false;
                for i in res {
                    if i.email == user_email {
                        found = true; 
                        break;
                    } 
                }

                if found == true {
                    vec.push(
                        structs::ResponseMessage {
                            message: "Usuário encontrado.".to_string(), 
                            code: 200, 
                            logged: "true".to_string()
                        });
                } else {
                    vec.push(
                        structs::ResponseMessage {
                            message: "As credenciais estão erradas. Não existe usuário com esse nome.".to_string(),
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