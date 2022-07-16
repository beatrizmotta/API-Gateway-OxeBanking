use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder};
mod structs;


#[post("/login")]
pub async fn getuser(req_body: String) -> impl Responder {
    let data: structs::UserData = serde_json::from_str(&req_body).unwrap();
    let user_email: String = data.email;
    let user_password: String = data.password;

    // httpresponse.insert("status", "200");
    
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