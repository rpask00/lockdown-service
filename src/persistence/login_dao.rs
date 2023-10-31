use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use error_stack::{Result, ResultExt};
use rocket::FromForm;
use rocket::http::hyper::body::HttpBody;
use sqlx::PgPool;
use thiserror::Error;

use crate::models::DBError;
use crate::models::login_model::{Login, LoginDto};

#[async_trait]
pub trait LoginDao {
    async fn create_login(&self, login: LoginDto, owner_id: i32) -> Result<Login, DBError>;
    async fn get_logins(&self, owner_id: i32) -> Result<Vec<Login>, DBError>;
    async fn get_login(&self, id: i32) -> Result<Login, LoginError>;
    async fn get_login_owner(&self, id: i32) -> Result<i32, DBError>;
    async fn delete_logins(&self, ids: &Vec<i32>) -> Result<(), DBError>;
    async fn update_login(&self, id: i32, login: LoginDto) -> Result<Login, LoginError>;
}

#[derive(FromForm)]
pub struct Collection {
    pub ids: Vec<i32>,
}

pub struct LoginDaoImpl {
    db: PgPool,
}

impl LoginDaoImpl {
    pub fn new(db: PgPool) -> Self {
        LoginDaoImpl { db }
    }
}


#[derive(Debug, Error)]
pub enum LoginError {
    InvalidInput(String),
    Other,
    DatabaseError,
}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::InvalidInput(msg) => f.write_str(msg),
            LoginError::Other => f.write_str("Something went wrong! Try again!"),
            LoginError::DatabaseError => f.write_str("Error in database occurred."),
        }
    }
}

#[async_trait]
impl LoginDao for LoginDaoImpl {
    async fn create_login(&self, login: LoginDto, owner_id: i32) -> Result<Login, DBError> {
        let record = sqlx::query!(
            r#"
                INSERT INTO logins (username, note, password, email, linked_websites, collections, owner_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, used_at, username,note, password, email, linked_websites, collections
            "#,
            login.username,
            login.note,
            login.password,
            login.email,
            login.linked_websites.unwrap_or(vec![]).join(","),
            login.collections.unwrap_or(vec![]).join(","),
            owner_id
        ).fetch_one(&self.db)
            .await
            .change_context(DBError::Other2)?;



        Ok(Login {
            id: record.id,
            used_at: record.used_at.to_string(),
            username: record.username,
            note: record.note,
            password: record.password,
            email: record.email.to_string(),
            linked_websites: record.linked_websites.split(",").map(|s| s.to_string()).collect(),
            collections: record.collections.split(",").map(|s| s.to_string()).collect(),
        })
    }

    async fn get_logins(&self, owner_id: i32) -> Result<Vec<Login>, DBError> {
        let records = sqlx::query!(r#"
            SELECT id, used_at, username, password,note, email, linked_websites, collections
            FROM logins
            WHERE owner_id = $1
        "#,
        owner_id
        ).fetch_all(&self.db).await
            .change_context(DBError::Other2)?;



        Ok(records.iter().map(|record| Login {
            id: record.id,
            used_at: record.used_at.to_string(),
            username: record.username.to_string(),
            note: record.note.to_string(),
            password: record.password.to_string(),
            email: record.email.to_string(),
            linked_websites: record.linked_websites.split(",").map(|s| s.to_string()).collect(),
            collections: record.collections.split(",").map(|s| s.to_string()).collect(),
        }).collect())
    }

    async fn get_login(&self, id: i32) -> Result<Login, LoginError> {
        let record = sqlx::query!(r#" SELECT * FROM logins WHERE id = $1"#, id).fetch_one(&self.db).await
            .change_context(DBError::Other2)
            .change_context(LoginError::DatabaseError)?;

        return Ok(Login {
            id: record.id,
            used_at: record.used_at.to_string(),
            username: record.username.to_string(),
            password: record.password.to_string(),
            note: record.note.to_string(),
            email: record.email.to_string(),
            linked_websites: record.linked_websites.split(",").map(|s| s.to_string()).collect(),
            collections: record.collections.split(",").map(|s| s.to_string()).collect(),
        });
    }

    async fn get_login_owner(&self, id: i32) -> Result<i32, DBError> {
        let record = sqlx::query!(r#" SELECT owner_id FROM logins WHERE id = $1"#, id).fetch_one(&self.db).await
            .change_context(DBError::Other2)?;

        return Ok(record.owner_id.unwrap());
    }

    async fn delete_logins(&self, ids: &Vec<i32>) -> Result<(), DBError> {
        sqlx::query!(r#"DELETE FROM logins WHERE id = ANY($1)"#, ids).execute(&self.db).await.map_err(
            |e| DBError::Other(Box::new(e))
        )?;

        Ok(())
    }

    async fn update_login(&self, id: i32, login_dao: LoginDto) -> Result<Login, LoginError> {
        let mut login = self.get_login(id).await?;

        if let Some(password) = login_dao.password {
            login.password = password;
        }
        if let Some(note) = login_dao.note {
            login.note = note;
        }
        if let Some(email) = login_dao.email {
            login.email = email;
        }
        if let Some(username) = login_dao.username {
            login.username = username;
        }
        if let Some(linked_websites) = login_dao.linked_websites {
            login.linked_websites = linked_websites;
        }
        if let Some(collections) = login_dao.collections {
            login.collections = collections;
        }

        sqlx::query!(r#"
            Update logins
            set username = $1, note = $2, password = $3, email = $4, linked_websites = $5, collections = $6
            where id = $7
        "#,
            login.username,
            login.note,
            login.password,
            login.email,
            login.linked_websites.join(","),
            login.collections.join(","),
            id
        ).execute(&self.db)
            .await
            .change_context(DBError::Other2)
            .change_context(LoginError::DatabaseError)?;

        return Ok(login);
    }
}
