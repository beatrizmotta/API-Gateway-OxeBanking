use serde::{Deserialize, Serialize};

// Login Route

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountDetails {
    pub client_type: i64,
    pub address: String,
    pub email: String,
    pub password: String, 
    pub phone: String,
    pub status: i64,
    pub id: i64,
    pub mounthly_income: i64,
    pub official_document: i64,
    pub client_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Response {
    Results(Vec<AccountDetails>),
    Error(Vec<reqwest::Error>),
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub code: i32,
    pub message: String,
    pub logged: String,
}

// Transfer Route

#[derive(Deserialize, Serialize)]
pub struct create_transfer {
    pub docClienteOrigem: i32,
    pub docClienteDestino: i32,
    pub nomeClienteOrigem: String,
    pub nomeClienteDestino: String,
    pub bancoOrigem: String,
    pub bancoDestino: String,
    pub ValorTransf: i32,
    pub DataHora: String,
}

#[derive(Deserialize, Serialize)]
pub struct update_transfer {
    pub id: i32,
    pub bancoDestino: String,
    pub ValorTransf: i32,
    pub DataHora: String,
}

#[derive(Deserialize, Serialize)]
pub struct delete_transfer {
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct create_transfer_pix {
    pub chave: String,
    pub tipo: String,
    pub transfer_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct update_transfer_pix {
    pub id: i32,
    pub chave: String,
}

#[derive(Deserialize, Serialize)]
pub struct create_transfer_ted_doc {
    pub agencia: String,
    pub conta: String,
    pub transfer_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct update_transfer_ted_doc {
    pub id: i32,
    pub conta: String,
}

#[derive(Serialize)]
pub struct Test{
    pub status: String,
} 
