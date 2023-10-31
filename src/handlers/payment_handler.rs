use rocket::{delete, get, post, put, State};
use rocket::serde::json::Json;

use crate::APIError;
use crate::models::payment_model::{Payment, PaymentDto};
use crate::models::user_model::User;
use crate::persistence::payment_dao::PaymentDao;

#[post("/payments", data = "<payment>")]
pub async fn create_payment(user: User, payment: Json<PaymentDto>, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<Json<Payment>, APIError> {
    return payment_dao.create_payment(payment.0, user.id).await
        .map(|payment| Json(payment))
        .map_err(|err| APIError::BadRequest(err.to_string()));
}


#[get("/payments/<id>")]
pub async fn get_payment(user: User, id: i32, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<Json<Payment>, APIError> {
    return payment_dao.get_payment(id).await
        .map(|payment| Json(payment))
        .map_err(|_| APIError::NotFound(format!("Payment with id {} not found", id)));
}


#[get("/payments")]
pub async fn get_payments(user: User, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<Json<Vec<Payment>>, APIError> {
    return payment_dao.get_payments(user.id).await
        .map(|payment| Json(payment))
        .map_err(|err| APIError::InternalError(err.to_string()));
}

#[delete("/payments/<id>")]
pub async fn delete_payment(user: User, id: i32, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<(), APIError> {
    return payment_dao.delete_payment(id).await
        .map_err(|err| APIError::InternalError(err.to_string()));
}


#[put("/payments/<id>", data = "<payment>")]
pub async fn update_payment(user: User, id: i32, payment: Json<PaymentDto>, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<Json<Payment>, APIError> {
    return payment_dao.update_payment(id, payment.0).await
        .map(|payment| Ok(Json(payment)))
        .map_err(|_| APIError::NotFound(format!("Payment with id {} not found", id)))?;
}
