use jsonwebtoken::{DecodingKey, Validation};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

use crate::{models::*, persistence::users_dao::UsersDao};
use crate::handlers::auth_handler::{Token, TokenError};
use crate::models::auth_model::TokenClaims;
use crate::persistence::auth_dao::AuthDao;

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
        let auth_dao = request.rocket().state::<Box<dyn AuthDao + Sync + Send>>().unwrap();


        let authorization = match request.cookies().get_private("Authorization") {
            Some(token) => token.to_string(),
            None => return Outcome::Failure((Status::ImATeapot, TokenError::Missing))
        };

        let authorization = authorization.split("=").skip(1).next().unwrap();

        let token_blacklisted = auth_dao.token_blacklisted(Token(authorization.to_string())).await.map_err(|_| true).unwrap();

        if token_blacklisted {
            return Outcome::Failure((Status::Unauthorized, TokenError::Invalid));
        }

        let decoded_claims = jsonwebtoken::decode::<TokenClaims>(authorization, decoding_key, &Validation::default());

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
