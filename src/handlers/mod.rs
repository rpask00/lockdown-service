use rocket::{options, Responder, routes};

use handlers_inner::*;

mod handlers_inner;
mod user_handler;
pub mod auth_handler;
mod login_handler;
mod payment_handler;
mod secured_note_handler;


#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
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

#[options("/<_..>")]
pub async fn allow_options() -> &'static str {
    "ok"
}


pub fn app_routes() -> Vec<rocket::Route> {
    routes![
        self::allow_options,
        // AUTH
        auth_handler::status,
        auth_handler::login,
        auth_handler::logout,
        // USER
        user_handler::get_user,
        user_handler::create_user,
        user_handler::update_user,
        user_handler::delete_user,
         // LOGIN
        login_handler::create_login,
        login_handler::get_logins,
        login_handler::get_login,
        login_handler::delete_login,
        login_handler::update_login,
        // PAYMENT
        payment_handler::create_payment,
        payment_handler::get_payment,
        payment_handler::get_payments,
        payment_handler::delete_payment,
        payment_handler::update_payment,
    ]
}
