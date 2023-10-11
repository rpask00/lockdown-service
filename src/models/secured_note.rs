use std::fmt::{Debug, Display, Formatter};

use rocket::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub struct SecuredNote {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub created_at: String,
    pub modified_at: String,
    pub color: String,
}


#[derive(Debug, Error, Serialize, Deserialize)]
pub struct SecuredNoteDto {
    pub name: Option<String>,
    pub content: Option<String>,
    pub color: Option<String>,
}


impl Display for SecuredNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}


impl Display for SecuredNoteDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}
