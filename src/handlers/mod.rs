use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rocket::{delete, get, post, put, Request, Responder, routes, serde::json::Json, State};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use handlers_inner::*;

use crate::{
    models::*,
    persistence::users_dao::UsersDao,
};

mod handlers_inner;


pub struct Token(pub(crate) String);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("token") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, TokenError::Missing))
        };

        Outcome::Success(Token(token.to_string()))
    }
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TokenError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let decoding_key = request.rocket().state::<DecodingKey>().unwrap();
        let user_dao = request.rocket().state::<Box<dyn UsersDao + Sync + Send>>().unwrap();

        let token = match request.headers().get_one("token") {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, TokenError::Missing))
        };

        let decoded_claims = jsonwebtoken::decode::<TokenClamis>(token, decoding_key, &Validation::default());

        let user_id = match decoded_claims {
            Ok(token_claims) => token_claims.claims.sub,
            Err(e) => return Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
        };

        println!("user_id: {}", user_id);

        return match user_dao.get_user(user_id).await {
            Ok(user) => Outcome::Success(user),
            Err(e) => Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
        };
    }
}

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
    #[response(status = 401)]
    InvalidCredentials(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

#[post("/login", data = "<credentials>")]
pub async fn login(
    credentials: Json<Credentials>,
    users_dao: &State<Box<dyn UsersDao + Sync + Send>>,
    jwt_encoding_key: &State<EncodingKey>,
) -> Result<Json<LoginResponse>, APIError> {
    match users_dao.login(credentials.0, jwt_encoding_key.inner()).await {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(APIError::InvalidCredentials(err.to_string())),
    }
}

#[post("/logout")]
pub async fn logout(token: Token, users_dao: &State<Box<dyn UsersDao + Sync + Send>>) -> Result<(), APIError> {
    users_dao.logout(token).await.map_err(|e| APIError::InternalError(e.to_string()))?;
    Ok(())
}


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


pub fn app_routes() -> Vec<rocket::Route> {
    routes![
        login,
        logout,
        get_user,
        create_user,
        update_user,
        delete_user
    ]
}
