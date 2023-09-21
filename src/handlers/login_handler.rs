use rocket::{delete, get, post, put, State};
use rocket::http::hyper::body::HttpBody;
use rocket::serde::json::Json;

use crate::APIError;
use crate::models::login_model::{Login, LoginDto};
use crate::models::user_model::User;
use crate::persistence::login_dao::LoginDao;

#[post("/logins", data = "<login>")]
pub async fn create_login(user: User, login: Json<LoginDto>, login_dao: &State<Box<dyn LoginDao + Sync + Send>>) -> Result<Json<Login>, APIError> {
    return login_dao.create_login(login.0, user.id).await
        .map(|login| Json(login))
        .map_err(|err| APIError::InternalError(err.to_string()));
}

#[get("/logins")]
pub async fn get_logins(user: User, login_dao: &State<Box<dyn LoginDao + Sync + Send>>) -> Result<Json<Vec<Login>>, APIError> {
    return login_dao.get_logins(user.id).await
        .map(|logins| Json(logins))
        .map_err(|err| APIError::InternalError(err.to_string()));
}


#[get("/logins/<id>")]
pub async fn get_login(id: i32, user: User, login_dao: &State<Box<dyn LoginDao + Sync + Send>>) -> Result<Json<Login>, APIError> {
    validate_user_owns_login(user.id, id, login_dao).await?;

    return login_dao.get_login(id).await
        .map(|login| Json(login))
        .map_err(|err| APIError::InternalError(err.to_string()));
}

#[delete("/logins/<id>")]
pub async fn delete_login(id: i32, user: User, login_dao: &State<Box<dyn LoginDao + Sync + Send>>) -> Result<(), APIError> {
    validate_user_owns_login(user.id, id, login_dao).await?;

    login_dao.delete_login(id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    return Ok(());
}

#[put("/logins/<id>", data = "<login>")]
pub async fn update_login(id: i32, login: Json<LoginDto>, user: User, login_dao: &State<Box<dyn LoginDao + Sync + Send>>) -> Result<Json<Login>, APIError> {
    validate_user_owns_login(user.id, id, login_dao).await?;

    let result = login_dao.update_login(id, login.0).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    return Ok(Json(result));
}

async fn validate_user_owns_login(user_id: i32, login_id: i32, login_dao: &State<Box<dyn LoginDao + Sync + Send>>) -> Result<(), APIError> {
    let login_owner = login_dao.get_login_owner(login_id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    if login_owner != user_id {
        return Err(APIError::Unauthorized(String::from("Login doesn't belong to user.")));
    }

    Ok(())
}
