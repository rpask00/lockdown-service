use rocket::{delete, get, post, put, Responder, routes, serde::json::Json, State};

use handlers_inner::*;

use crate::{
    models::*,
    persistence::users_dao::UsersDao,
};

mod handlers_inner;

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

#[get("/user/<id>")]
pub async fn get_user(
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


pub fn app_routes() -> Vec<rocket::Route> {
    routes![
        get_user,
        create_user,
        update_user,
        delete_user
    ]
}
