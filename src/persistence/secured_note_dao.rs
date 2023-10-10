use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::DBError;
use crate::models::secured_note::{SecuredNote, SecuredNoteDto};

#[async_trait]
pub trait SecuredNoteDao {
    async fn create_secured_note(&self, secured_note: SecuredNoteDto, owner_id: i32) -> Result<SecuredNote, DBError>;
}

pub struct SecuredNoteDaoImpl {
    db: PgPool,
}

impl SecuredNoteDaoImpl {
    pub fn new(db: PgPool) -> Self {
        SecuredNoteDaoImpl { db }
    }
}

#[async_trait]
impl SecuredNoteDao for SecuredNoteDaoImpl {
    async fn create_secured_note(&self, secured_note: SecuredNoteDto, owner_id: i32) -> Result<SecuredNote, DBError> {
        let record = sqlx::query!(r#"
           INSERT INTO secured_notes (name, content, color, owner_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, content, created_at, modified_at, color, owner_id
        "#,
            secured_note.name,
            secured_note.content,
            secured_note.color,
            owner_id,

        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(SecuredNote{
            id: 0,
            name: record.name,
            content: record.content,
            created_at: record.created_at.unwrap().to_string(),
            modified_at: record.modified_at.unwrap().to_string(),
            color: record.color,
        })
    }
}



