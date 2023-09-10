use rocket::{delete, get, post, put, serde::json::Json, State};

use crate::{APIError, persistence::users_dao::UsersDao};
use crate::handlers::handlers_inner;
use crate::models::user_model::{User, UserDto, UserUpdateDto};

#[get("/user/<id>")]
pub async fn get_user(
    _user: User,
    id: i32,
    users_dao: &State<Box<dyn UsersDao + Sync + Send>>,
) -> Result<Json<User>, APIError> {
    match handlers_inner::get_user(id, users_dao.inner()).await {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(err.into()),
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

#[put("/user/<id>", data = "<user>")]
pub async fn update_user(
    user: Json<UserUpdateDto>,
    id: i32,
    users_dao: &State<Box<dyn UsersDao + Sync + Send>>,
) -> Result<Json<User>, APIError> {
    match handlers_inner::update_user(user.0, id, users_dao.inner()).await {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(err.into()),
    }
}


#[delete("/user/<id>")]
pub async fn delete_user(
    id: i32,
    users_dao: &State<Box<dyn UsersDao + Sync + Send>>,
) -> Result<(), APIError> {
    match handlers_inner::delete_user(id, users_dao.inner()).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
