use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use bcrypt::{DEFAULT_COST, hash_with_salt};
use jsonwebtoken::EncodingKey;
use rand::random;
use thiserror::Error;
use sqlx::{Execute, PgPool};

use crate::models::{Credentials, DBError, LoginResponse, User, UserDto, UserUpdateDto};

#[derive(Debug, Error)]
enum UserError {
    InvalidInput(String),
    Other,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::InvalidInput(msg) => f.write_str(msg),
            UserError::Other => f.write_str("Something went wrong! Try again!"),
        }
    }
}

#[async_trait]
pub trait UsersDao {
    async fn login(&self, credentials: Credentials, jwt_encoding_key: &EncodingKey) -> Result<LoginResponse, DBError>;
    async fn get_user(&self, id: i32) -> Result<User, DBError>;
    async fn create_user(&self, user: UserDto) -> Result<User, DBError>;
    async fn update_user(&self, user: UserUpdateDto, user_id: i32) -> Result<User, DBError>;
    async fn delete_user(&self, user_id: i32) -> Result<(), DBError>;
    // async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct UsersDaoImpl {
    db: PgPool,
}

impl UsersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        UsersDaoImpl { db }
    }
}

#[async_trait]
impl UsersDao for UsersDaoImpl {
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
            Err(_) => return Err(DBError::Other(Box::new(UserError::Other))),
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

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &user,
            &jwt_encoding_key,
        ).unwrap();

        Ok(LoginResponse {
            user,
            token,
        })
    }

    async fn get_user(&self, id: i32) -> Result<User, DBError> {
        let record = sqlx::query!(
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            id
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(User {
            id: record.id,
            username: record.username,
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            created_at: record.created_at.unwrap().to_string(),
        })
    }

    async fn create_user(&self, user: UserDto) -> Result<User, DBError> {
        let salt = random();
        // Concatenate the password and salt, then hash it
        let hashed_password = match hash_with_salt(&user.password, DEFAULT_COST, salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => return Err(DBError::Other(Box::new(UserError::Other))),
        };

        let record = sqlx::query!(
            r#"
                INSERT INTO users ( username, first_name, last_name,password, email, salt )
                VALUES ( $1, $2, $3, $4, $5, $6 )
                RETURNING *
            "#,
            user.username,
            user.first_name,
            user.last_name,
            hashed_password,
            user.email,
            general_purpose::STANDARD.encode(&salt)
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;


        Ok(User {
            id: record.id,
            username: record.username,
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            created_at: record.created_at.unwrap().to_string(),
        })
    }

    async fn update_user(&self, user_update: UserUpdateDto, user_id: i32) -> Result<User, DBError> {
        let mut user = self.get_user(user_id).await?;

        if let Some(username) = user_update.username {
            user.username = username;
        }
        if let Some(first_name) = user_update.first_name {
            user.first_name = first_name;
        }
        if let Some(last_name) = user_update.last_name {
            user.last_name = last_name;
        }
        if let Some(email) = user_update.email {
            user.email = email;
        }

        sqlx::query!(
            r#"
                UPDATE users
                SET username = $1, first_name = $2, last_name = $3, email = $4
                WHERE id = $5
                RETURNING *
            "#,
            user.username,
            user.first_name,
            user.last_name,
            user.email,
            user_id
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), DBError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }
}


