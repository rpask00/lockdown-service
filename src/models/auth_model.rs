use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}




#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub exp: usize,
}
