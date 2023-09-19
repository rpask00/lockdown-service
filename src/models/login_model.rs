use std::fmt::{Display, Formatter};

use rocket::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub note: String,
    pub password: String,
    pub email: String,
    pub linked_websites: Vec<String>,
    pub collections: Vec<String>,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct Login {
    pub id: i32,
    pub used_at: String,
    pub username: String,
    pub password: String,
    pub note: String,
    pub email: String,
    pub linked_websites: Vec<String>,
    pub collections: Vec<String>,
}

impl Display for LoginDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Display for Login {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}


