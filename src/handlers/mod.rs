use rocket::{post, Responder, serde::json::Json, State};

use crate::{
    models::*,
    persistence::{users_dao::UsersDao},
};

mod handlers_inner;

use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

#[post("/user", data = "<user>")]
pub async fn create_user(
    user: Json<UserDto>,
    users_dao: &State<Box<dyn UsersDao + Sync + Send>>,
) -> Result<Json<User>, APIError> {
    match handlers_inner::create_user(user.0, users_dao.inner()).await {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(err.into()),
    }
}
