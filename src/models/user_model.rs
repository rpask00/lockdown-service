use jsonwebtoken::{DecodingKey, Validation};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::{models::*, persistence::users_dao::UsersDao, TokenError};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateDto {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
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


        return match user_dao.get_user(user_id).await {
            Ok(user) => Outcome::Success(user),
            Err(e) => Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
        };
    }
}
