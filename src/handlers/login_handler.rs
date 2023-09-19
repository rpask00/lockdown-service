use rocket::{post, State};
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
