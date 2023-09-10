use jsonwebtoken::{DecodingKey, Validation};
use rocket::{Request, Responder, routes};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use handlers_inner::*;

use crate::{
    models::*,
    persistence::users_dao::UsersDao,
};

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


#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let decoding_key = request.rocket().state::<DecodingKey>().unwrap();
        let user_dao = request.rocket().state::<Box<dyn UsersDao + Sync + Send>>().unwrap();

        let token = match request.headers().get_one("token") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, TokenError::Missing))
        };

        let decoded_claims = jsonwebtoken::decode::<TokenClamis>(token, decoding_key, &Validation::default());

        let user_id = match decoded_claims {
            Ok(token_claims) => token_claims.claims.sub,
            Err(e) => return Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
        };


        return match user_dao.get_user(user_id).await {
            Ok(user) => Outcome::Success(user),
            Err(e) => Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
        };
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
