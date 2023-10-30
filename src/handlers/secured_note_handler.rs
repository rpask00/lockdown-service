use std::path::Path;

use rocket::{Data, delete, get, post, put, State};
use rocket::data::{FromData, ToByteUnit};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};
use tokio::fs;

use crate::APIError;
use crate::models::secured_note::{File, FileDto, SecuredNote, SecuredNoteDto};
use crate::models::user_model::User;
use crate::persistence::secured_note_dao::SecuredNoteDao;

#[post("/secured_notes", data = "<secured_note>")]
pub async fn create_secured_note(user: User, secured_note: Json<SecuredNoteDto>, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<SecuredNote>, APIError> {
    let secured_note = secured_notes_dao.create_secured_note(secured_note.0, user.id).await
        .map(Json)
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
    let note_attachments = secured_notes_dao.get_secured_note_attachments(id).await.map_err(|err| APIError::InternalError(err.to_string()))?;


    for attachment in note_attachments {
        fs::remove_file(format!("upload/{}", attachment.id)).await.unwrap();
    }

    secured_notes_dao.delete_secured_note(id).await.map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(())
}


#[post("/secured_notes/<id>/attachments", data = "<paste>")]
pub async fn upload<'r>(user: User, id: i32, ct: &ContentType, paste: Data<'r>, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<Vec<File>>, APIError> {
    validate_user_owns_secured_note(user.id, id, secured_notes_dao).await?;

    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("file").size_limit(100.megabytes().as_u64()),
    ]);

    let multipart_form = MultipartFormData::parse(ct, paste, options).await.unwrap();
    let file_data = multipart_form.files.get("file").unwrap();


    let mut response_files = vec![];


    for file in file_data {
        let file_name = file.file_name.as_ref().unwrap();


        let file_dto = FileDto {
            name: file_name.to_owned(),
            file_type: file.content_type.as_ref().unwrap().to_string(),
            size: fs::metadata(&file.path).await.unwrap().len() as i32,
        };

        let _file = secured_notes_dao.save_file(user.id, file_dto, id).await.unwrap();
        let target_path = format!("upload/{}", _file.id);
        fs::copy(&file.path, Path::new(&target_path)).await.expect("Error");
        response_files.push(_file);
    }

    Ok(Json(response_files))
}

#[get("/secured_notes/<id>/attachments")]
pub async fn secured_note_attachments(user: User, id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<Json<Vec<File>>, APIError> {
    validate_user_owns_secured_note(user.id, id, secured_notes_dao).await?;

    let files = secured_notes_dao.get_secured_note_attachments(id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;


    Ok(Json(files))
}

#[get("/attachments/<id>")]
pub async fn download_attachment(user: User, id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<fs::File, APIError> {
    let file = secured_notes_dao.get_secured_note_attachment(id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    validate_user_owns_secured_note(user.id, file.note_id, secured_notes_dao).await?;

    fs::File::open(format!("upload/{}", file.id)).await
        .map_err(|err| APIError::InternalError(err.to_string()))
}

#[delete("/attachments/<id>")]
pub async fn delete_attachment(user: User, id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<(), APIError> {
    let file = secured_notes_dao.get_secured_note_attachment(id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    validate_user_owns_secured_note(user.id, file.note_id, secured_notes_dao).await?;

    secured_notes_dao.delete_secured_note_attachment(id).await.map_err(|err| APIError::InternalError(err.to_string()))?;

    fs::remove_file(format!("upload/{}", id)).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;

    Ok(())
}


async fn validate_user_owns_secured_note(user_id: i32, note_id: i32, secured_notes_dao: &State<Box<dyn SecuredNoteDao + Sync + Send>>) -> Result<(), APIError> {
    let note_owner_id = secured_notes_dao.get_secured_note_owner(note_id).await
        .map_err(|err| APIError::InternalError(err.to_string()))?;


    if note_owner_id != user_id {
        return Err(APIError::Unauthorized(String::from("Note doesn't belong to user.")));
    }

    Ok(())
}
