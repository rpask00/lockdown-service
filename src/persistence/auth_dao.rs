use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use bcrypt::{DEFAULT_COST, hash_with_salt};
use jsonwebtoken::EncodingKey;
use sqlx::PgPool;
use thiserror::Error;

use crate::models::auth_model::{Credentials, LoginResponse, TokenClaims};
use crate::models::DBError;
use crate::models::user_model::User;
use crate::Token;

#[async_trait]
pub trait AuthDao {
    async fn login(&self, credentials: Credentials, jwt_encoding_key: &EncodingKey) -> Result<LoginResponse, DBError>;
    async fn logout(&self, token: Token) -> Result<(), DBError>;
    async fn token_blacklisted(&self, token: Token) -> Result<bool, DBError>;
}


pub struct AuthDaoImpl {
    db: PgPool,
}

impl AuthDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AuthDaoImpl { db }
    }
}


#[derive(Debug, Error)]
enum AuthError {
    InvalidInput(String),
    Other,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidInput(msg) => f.write_str(msg),
            AuthError::Other => f.write_str("Something went wrong! Try again!"),
        }
    }
}


#[async_trait]
impl AuthDao for AuthDaoImpl {
    async fn login(&self, credentials: Credentials, jwt_encoding_key: &EncodingKey) -> Result<LoginResponse, DBError> {
        let record = sqlx::query!(
            r#"
                SELECT * FROM users WHERE username = $1
            "#,
            credentials.username
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let mut salt = [0u8; 16];
        salt.copy_from_slice(&*general_purpose::STANDARD.decode(&record.salt.as_bytes()).unwrap());

        let hashed_password = match hash_with_salt(&credentials.password, DEFAULT_COST, salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => return Err(DBError::Other(Box::new(AuthError::Other))),
        };

        let record = sqlx::query!(
            r#"
                SELECT * FROM users WHERE username = $1 AND password = $2
            "#,
            credentials.username,
            hashed_password
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let user = User {
            id: record.id,
            username: record.username,
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            created_at: record.created_at.unwrap().to_string(),
        };
        let claims = TokenClaims {
            sub: user.id,
            exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &jwt_encoding_key,
        ).unwrap();

        Ok(LoginResponse {
            user,
            token,
        })
    }

    async fn logout(&self, token: Token) -> Result<(), DBError> {
        sqlx::query("INSERT INTO token_blacklist (token) VALUES ($1)")
            .bind(token.0)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn token_blacklisted(&self, token: Token) -> Result<bool, DBError> {
        let record = sqlx::query!(
            r#"
               SELECT COUNT(*) as c FROM token_blacklist WHERE token = $1
            "#,
            token.0
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(record.c.unwrap() > 0)
    }
}
