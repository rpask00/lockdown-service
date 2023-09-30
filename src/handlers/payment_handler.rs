use rocket::{get, post, State};
use rocket::serde::json::Json;

use crate::APIError;
use crate::models::payment_model::{Payment, PaymentDto};
use crate::models::user_model::User;
use crate::persistence::payment_dao::PaymentDao;

#[post("/payments", data = "<payment>")]
pub async fn create_payment(user: User, payment: Json<PaymentDto>, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<Json<Payment>, APIError> {
    return payment_dao.create_payment(payment.0).await
        .map(|payment| Json(payment))
        .map_err(|err| APIError::InternalError(err.to_string()));
}


#[get("/payments/<id>")]
pub async fn get_payment(user: User, id: i32, payment_dao: &State<Box<dyn PaymentDao + Sync + Send>>) -> Result<Json<Payment>, APIError> {
    return payment_dao.get_payment(id).await
        .map(|payment| Json(payment))
        .map_err(|err| APIError::InternalError(err.to_string()));
}
