use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use bcrypt::{DEFAULT_COST, hash_with_salt};
use error_stack::ResultExt;
use rand::random;
use sqlx::PgPool;
use thiserror::Error;

use crate::models::DBError;
use crate::models::user_model::{User, UserDto, UserUpdateDto};

#[derive(Debug, Error)]
pub enum UserError {
    InvalidInput(String),
    Other,
    DatabaseError,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::InvalidInput(msg) => f.write_str(msg),
            UserError::Other => f.write_str("Something went wrong! Try again!"),
            UserError::DatabaseError => f.write_str("Error in database occurred."),
        }
    }
}

#[async_trait]
pub trait UsersDao {
    async fn get_user(&self, id: i32) -> error_stack::Result<User, DBError>;
    async fn create_user(&self, user: UserDto) -> error_stack::Result<User, UserError>;
    async fn update_user(&self, user: UserUpdateDto, user_id: i32) -> error_stack::Result<User, UserError>;
    async fn delete_user(&self, user_id: i32) -> error_stack::Result<(), UserError>;
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
    async fn get_user(&self, id: i32) -> error_stack::Result<User, DBError> {
        let record = sqlx::query!(
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            id
        ).fetch_one(&self.db)
            .await
            .change_context(DBError::Other2)?;

        Ok(User {
            id: record.id,
            username: record.username,
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            created_at: record.created_at.unwrap().to_string(),
        })
    }

    async fn create_user(&self, user: UserDto) -> error_stack::Result<User, UserError> {
        let salt = random();
        // Concatenate the password and salt, then hash it

        let hashed_password = hash_with_salt(&user.password, DEFAULT_COST, salt).change_context(
            UserError::InvalidInput("Hashing password failed.".to_string())
        )?.to_string();

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
            .change_context(DBError::Other2)
            .attach_printable(format!("Inserting user with username {} failed", user.username))
            .change_context(UserError::DatabaseError)?;


        Ok(User {
            id: record.id,
            username: record.username,
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            created_at: record.created_at.unwrap().to_string(),
        })
    }

    async fn update_user(&self, user_update: UserUpdateDto, user_id: i32) -> error_stack::Result<User, UserError> {
        let mut user = self.get_user(user_id).await.change_context(UserError::DatabaseError)?;

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
            .change_context(DBError::Other2)
            .attach_printable(format!("Updating user with username {} failed", user.username))
            .change_context(UserError::DatabaseError)?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: i32) -> error_stack::Result<(), UserError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.db)
            .await
            .change_context(DBError::Other2)
            .attach_printable(format!("Deleting user with id {} failed", user_id))
            .change_context(UserError::DatabaseError)?;


        Ok(())
    }
}


