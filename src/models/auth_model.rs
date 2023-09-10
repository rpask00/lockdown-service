use rocket::serde::{Deserialize, Serialize};

use crate::models::user_model::User;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}


#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub exp: usize,
}
