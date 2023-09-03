use log::error;
use crate::{
    models::{User},
    persistence::{users_dao::UsersDao},
};
use crate::models::UserDto;

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    pub fn default_internal_error() -> Self {
        HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
    }
}

pub async fn create_user(
    user: UserDto,
    users_dao: &Box<dyn UsersDao + Sync + Send>,
) -> Result<User, HandlerError> {
    let user = users_dao.create_user(user).await;

    match user {
        Ok(user) => Ok(user),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}
