use rocket::{delete, get, post, put, State};
use rocket::serde::json::Json;

use crate::APIError;
use crate::models::secured_note::{SecuredNote, SecuredNoteDto};
use crate::models::user_model::User;
use crate::persistence::secured_note_dao::SecuredNoteDao;

#[post("/secured_notes", data = "<secured_note>")]
pub async fn create_secured_note(user: User, secured_note: Json<SecuredNoteDto>, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<SecuredNote>, APIError> {
    let secured_note = secured_notes_dao.create_secured_note(secured_note.0, user.id).await
        .map(|result| Json(result))
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(secured_note)
}


#[get("/secured_notes/<id>")]
pub async fn get_secured_note(user: User, id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<SecuredNote>, APIError> {
    validate_user_owns_secured_note(user.id, id, secured_notes_dao).await?;

    let secured_note = secured_notes_dao.get_secured_note(id).await.map(Json)
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(secured_note)
}

#[get("/secured_notes")]
pub async fn get_secured_notes(user: User, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<Vec<SecuredNote>>, APIError> {
    let secured_notes = secured_notes_dao.get_secured_notes(user.id).await.map(Json)
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(secured_notes)
}

#[put("/secured_notes/<id>", data = "<secured_note>")]
pub async fn update_secured_note(user: User, id: i32, secured_note: Json<SecuredNoteDto>, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<SecuredNote>, APIError> {
    validate_user_owns_secured_note(user.id, id, secured_notes_dao).await?;
    let secured_notes = secured_notes_dao.update_secured_notes(id, secured_note.0).await.map(Json)
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(secured_notes)
}


#[delete("/secured_notes/<id>")]
pub async fn delete_secured_note(user: User, id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<(), APIError> {
    validate_user_owns_secured_note(user.id, id, secured_notes_dao).await?;
    secured_notes_dao.delete_secured_note(id).await.map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(())
}


async fn validate_user_owns_secured_note(user_id: i32, note_id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<(), APIError> {
    let note = secured_notes_dao.get_secured_note(note_id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    if note.id != user_id {
        return Err(APIError::Unauthorized(String::from("Note doesn't belong to user.")));
    }

    Ok(())
}
