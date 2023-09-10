use jsonwebtoken::EncodingKey;
use rocket::{post, serde::json::Json, State};

use crate::{APIError, models::*, Token};
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

#[post("/logout")]
pub async fn logout(token: Token, auth_dao: &State<Box<dyn AuthDao + Sync + Send>>) -> Result<(), APIError> {
    auth_dao.logout(token).await.map_err(|e| APIError::InternalError(e.to_string()))?;
    Ok(())
}
