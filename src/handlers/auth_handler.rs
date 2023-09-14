use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::{get, post, Request, State};
use rocket::http::{Cookie, CookieJar};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;

use crate::APIError;
use crate::models::auth_model::{Credentials, TokenClaims};
use crate::models::user_model::User;
use crate::persistence::auth_dao::AuthDao;

pub struct Token(pub String);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.cookies().get("Authorization") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, TokenError::Missing))
        };

        Outcome::Success(Token(token.to_string()))
    }
}

#[post("/login", data = "<credentials>")]
pub async fn login<'a>(
    credentials: Json<Credentials>,
    auth_dao: &State<Box<dyn AuthDao + Sync + Send>>,
    jwt_encoding_key: &State<EncodingKey>,
    jar: &'a CookieJar<'_>,
) -> Result<Json<User>, APIError> {
    match auth_dao.login(credentials.0, jwt_encoding_key.inner()).await {
        Ok(u) => {
            jar.add_private(Cookie::new("Authorization", u.1.0.clone()));
            Ok(Json(u.0))
        }
        Err(err) => Err(APIError::InvalidCredentials(err.to_string())),
    }
}

#[get("/logout")]
pub async fn logout<'a>(_user: User, token: Token, auth_dao: &State<Box<dyn AuthDao + Sync + Send>>, jar: &'a CookieJar<'_>) -> Result<(), APIError> {
    jar.remove_private(Cookie::named("Authorization"));
    auth_dao.logout(token).await.map_err(|e| APIError::InternalError(e.to_string()))?;
    Ok(())
}


#[get("/status")]
pub async fn status<'a>(jar: &'a CookieJar<'_>, decoding_key: &State<DecodingKey>, auth_dao: &State<Box<dyn AuthDao + Send + Sync>>) -> Json<bool> {
    if let Some(authorization) = jar.get_private("Authorization") {
        let authorization = authorization.to_string();
        let authorization = authorization.split("=").skip(1).next().unwrap();

        let claims = jsonwebtoken::decode::<TokenClaims>(authorization, decoding_key, &jsonwebtoken::Validation::default());

        if let Ok(blacklisted) = auth_dao.token_blacklisted(Token(authorization.to_owned())).await {
            return Json(!blacklisted && claims.is_ok());
        }
    }

    return Json(false);
}


