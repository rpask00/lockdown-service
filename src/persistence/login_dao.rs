use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::DBError;
use crate::models::login_model::{Login, LoginDto};

#[async_trait]
pub trait LoginDao {
    async fn create_login(&self, login: LoginDto, owner_id: i32) -> Result<Login, DBError>;
    async fn get_logins(&self, owner_id: i32) -> Result<Vec<Login>, DBError>;
    async fn get_login(&self, id: i32) -> Result<Login, DBError>;
}

pub struct LoginDaoImpl {
    db: PgPool,
}

impl LoginDaoImpl {
    pub fn new(db: PgPool) -> Self {
        LoginDaoImpl { db }
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
            login.linked_websites.join(","),
            login.collections.join(","),
            owner_id
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;


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
            .map_err(|e| DBError::Other(Box::new(e)))?;


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

    async fn get_login(&self, id: i32) -> Result<Login, DBError> {
        let record = sqlx::query!(r#" SELECT * FROM logins WHERE id = $1"#, id).fetch_one(&self.db).await.map_err(
            |e| DBError::Other(Box::new(e))
        )?;

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
}
