use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use bcrypt::{DEFAULT_COST, hash_with_salt};
use rand::random;
use thiserror::Error;
use sqlx::{Execute, PgPool};

use crate::models::{DBError, User, UserDto};

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
    async fn create_user(&self, user: UserDto) -> Result<User, DBError>;
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
    async fn create_user(&self, user: UserDto) -> Result<User, DBError> {
        let salt = random();
        // Concatenate the password and salt, then hash it
        let hashed_password = match hash_with_salt(&user.password, DEFAULT_COST, salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => return Err(DBError::Other(Box::new(UserError::Other))),
        };

        let r = sqlx::query!(
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
            id: r.id,
            username: r.username,
            first_name: r.first_name,
            last_name: r.last_name,
            email: r.email,
            created_at: r.created_at.unwrap().to_string(),
        })
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), DBError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }
}


