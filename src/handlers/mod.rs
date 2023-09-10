use rocket::{Request, Responder, routes};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use handlers_inner::*;

mod handlers_inner;
mod user_handler;
mod auth_handler;

pub struct Token(pub(crate) String);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("token") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, TokenError::Missing))
        };

        Outcome::Success(Token(token.to_string()))
    }
}


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
