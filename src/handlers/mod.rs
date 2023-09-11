use rocket::{Responder, routes};
use rocket::request::FromRequest;

use handlers_inner::*;

mod handlers_inner;
mod user_handler;
pub mod auth_handler;




#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
    #[response(status = 401)]
    InvalidCredentials(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}


pub fn app_routes() -> Vec<rocket::Route> {
    routes![
        // AUTH
        auth_handler::login,
        auth_handler::logout,
        // USER
        user_handler::get_user,
        user_handler::create_user,
        user_handler::update_user,
        user_handler::delete_user
    ]
}
