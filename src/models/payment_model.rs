use std::fmt::{Display, Formatter};

use rocket::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct PaymentDto {
    pub card_holder: Option<String>,
    pub card_number: Option<String>,
    pub security_code: Option<i16>,
    pub expiration_month: Option<i16>,
    pub expiration_year: Option<i16>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub note: Option<String>,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: i32,
    pub card_holder: String,
    pub card_number: String,
    pub security_code: i16,
    pub expiration_month: i16,
    pub expiration_year: i16,
    pub name: String,
    pub color: String,
    pub note: String,
}

impl Display for PaymentDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Display for Payment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}
