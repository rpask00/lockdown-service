use jsonwebtoken::EncodingKey;
use rocket::{get, post, serde::json::Json, State};

use crate::{APIError, Token};
use crate::models::auth_model::{Credentials, LoginResponse};
use crate::models::user_model::User;
use crate::persistence::auth_dao::AuthDao;

#[post("/login", data = "<credentials>")]
pub async fn login(
    credentials: Json<Credentials>,
    auth_dao: &State<Box<dyn AuthDao + Sync + Send>>,
    jwt_encoding_key: &State<EncodingKey>,
) -> Result<Json<LoginResponse>, APIError> {
    match auth_dao.login(credentials.0, jwt_encoding_key.inner()).await {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(APIError::InvalidCredentials(err.to_string())),
    }
}

#[get("/logout")]
pub async fn logout(_user: User, token: Token, auth_dao: &State<Box<dyn AuthDao + Sync + Send>>) -> Result<(), APIError> {
    auth_dao.logout(token).await.map_err(|e| APIError::InternalError(e.to_string()))?;
    Ok(())
}
