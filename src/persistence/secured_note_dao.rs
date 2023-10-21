use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::DBError;
use crate::models::secured_note::{File, FileDto, SecuredNote, SecuredNoteDto};

#[async_trait]
pub trait SecuredNoteDao {
    async fn create_secured_note(&self, secured_note: SecuredNoteDto, owner_id: i32) -> Result<SecuredNote, DBError>;
    async fn get_secured_note(&self, id: i32) -> Result<SecuredNote, DBError>;
    async fn get_secured_notes(&self, owner_id: i32) -> Result<Vec<SecuredNote>, DBError>;
    async fn update_secured_notes(&self, id: i32, secured_note: SecuredNoteDto) -> Result<SecuredNote, DBError>;
    async fn delete_secured_note(&self, id: i32) -> Result<(), DBError>;
    async fn get_secured_note_owner(&self, id: i32) -> Result<i32, DBError>;
    async fn save_file(&self, owner_id: i32, file: FileDto, note_id: i32) -> Result<File, DBError>;
    async fn get_secured_note_attachments(&self, user_id: i32) -> Result<Vec<File>, DBError>;
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

        Ok(SecuredNote {
            id: record.id,
            name: record.name,
            content: record.content,
            created_at: record.created_at.to_string(),
            modified_at: record.modified_at.to_string(),
            color: record.color,
        })
    }

    async fn get_secured_note(&self, id: i32) -> Result<SecuredNote, DBError> {
        let record = sqlx::query!(r#"
           Select * FROM secured_notes
            WHERE id = $1
        "#,
            id
        ).fetch_one(&self.db).await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(SecuredNote {
            id: record.id,
            name: record.name,
            content: record.content,
            created_at: record.created_at.to_string(),
            modified_at: record.modified_at.to_string(),
            color: record.color,
        })
    }

    async fn get_secured_notes(&self, owner_id: i32) -> Result<Vec<SecuredNote>, DBError> {
        let record = sqlx::query!(r#"
           Select * FROM secured_notes
            WHERE owner_id = $1
            ORDER BY id
        "#,
            owner_id
        ).fetch_all(&self.db).await
            .map_err(|e| DBError::Other(Box::new(e)))?;


        Ok(record.iter().map(|r| SecuredNote {
            id: r.id,
            name: r.name.to_string(),
            content: r.content.to_string(),
            created_at: r.created_at.to_string(),
            modified_at: r.modified_at.to_string(),
            color: r.color.to_string(),
        }).collect())
    }

    async fn update_secured_notes(&self, id: i32, secured_note: SecuredNoteDto) -> Result<SecuredNote, DBError> {
        let mut _secured_note = self.get_secured_note(id).await?;

        if let Some(name) = secured_note.name {
            _secured_note.name = name;
        }

        if let Some(color) = secured_note.color {
            _secured_note.color = color;
        }

        if let Some(content) = secured_note.content {
            _secured_note.content = content;
        }

        let record = sqlx::query!(r#"
            UPDATE secured_notes set name = $1, content = $2, color = $3
            WHERE id = $4
            RETURNING *
        "#,
            _secured_note.name,
            _secured_note.content,
            _secured_note.color,
            id
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(SecuredNote {
            id: record.id,
            name: record.name,
            content: record.content,
            created_at: record.created_at.to_string(),
            modified_at: record.modified_at.to_string(),
            color: record.color,
        })
    }

    async fn delete_secured_note(&self, id: i32) -> Result<(), DBError> {
        sqlx::query!(r#"
            DELETE FROM secured_notes where id = $1
        "#, id).execute(&self.db).await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_secured_note_owner(&self, id: i32) -> Result<i32, DBError> {
        let record = sqlx::query!(r#" SELECT owner_id FROM secured_notes WHERE id = $1"#, id).fetch_one(&self.db).await.map_err(
            |e| DBError::Other(Box::new(e))
        )?;

        return Ok(record.owner_id.unwrap());
    }

    async fn get_secured_note_attachments(&self, note_id: i32) -> Result<Vec<File>, DBError> {
        let records = sqlx::query!(r#"
            SELECT type as file_type, * from note_attachments
            where note_id = $1
        "#, note_id).fetch_all(&self.db).await
            .map_err(|e| DBError::Other(Box::new(e)))?;


        Ok(records.iter().map(|record| File {
            id: record.id,
            name: record.name.to_string(),
            created_at: record.created_at.unwrap().to_string(),
            size: record.size,
            file_type: record.file_type.to_string(),
            note_id: record.note_id,
            owner_id: record.owner_id.unwrap(),
        }).collect())
    }

    async fn save_file(&self, owner_id: i32, file: FileDto, note_id: i32) -> Result<File, DBError> {
        let record = sqlx::query!(r#"
            INSERT INTO note_attachments (name, size, type, note_id, owner_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, created_at, size, type, note_id, type as file_type, owner_id
        "#,
        file.name, file.size, file.file_type, note_id, owner_id
        ).fetch_one(&self.db).await
            .map_err(|err| DBError::Other(Box::new(err)))?;


        Ok(File {
            id: record.id,
            name: record.name,
            created_at: record.created_at.unwrap().to_string(),
            size: record.size,
            file_type: record.file_type,
            note_id: record.note_id,
            owner_id: record.owner_id.unwrap(),
        })
    }
}



