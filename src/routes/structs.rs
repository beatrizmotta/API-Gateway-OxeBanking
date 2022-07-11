use serde::{Deserialize, Serialize};

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
