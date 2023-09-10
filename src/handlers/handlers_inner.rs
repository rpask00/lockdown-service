use log::error;

use crate::persistence::users_dao::UsersDao;
use crate::models::user_model::{User, UserDto, UserUpdateDto};

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

pub async fn  get_user(id:i32, users_dao: &Box<dyn UsersDao + Sync + Send>,
) -> Result<User, HandlerError> {
    let user = users_dao.get_user(id).await;

    match user {
        Ok(user) => Ok(user),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}

pub async fn update_user(
    user: UserUpdateDto,
    id: i32,
    users_dao: &Box<dyn UsersDao + Sync + Send>,
) -> Result<User, HandlerError> {
    let user = users_dao.update_user(user, id).await;

    match user {
        Ok(user) => Ok(user),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
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

pub async fn delete_user(
    id: i32,
    users_dao: &Box<dyn UsersDao + Sync + Send>,
) -> Result<(), HandlerError> {
    let user = users_dao.delete_user(id).await;

    match user {
        Ok(user) => Ok(user),
        Err(err) => {
            error!("{:?}", err);
            Err(HandlerError::default_internal_error())
        }
    }
}
