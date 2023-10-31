use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use error_stack::Result;
use error_stack::ResultExt;
use sqlx::PgPool;
use thiserror::Error;

use crate::models::DBError;
use crate::models::payment_model::{Payment, PaymentDto};

#[async_trait]
pub trait PaymentDao {
    async fn create_payment(&self, payment: PaymentDto, owner_id: i32) -> Result<Payment, DBError>;
    async fn update_payment(&self, id: i32, payment_dto: PaymentDto) -> Result<Payment, PaymentError>;
    async fn get_payment(&self, id: i32) -> Result<Payment, PaymentError>;
    async fn get_payments(&self, owner_id: i32) -> Result<Vec<Payment>, PaymentError>;
    async fn delete_payment(&self, id: i32) -> Result<(), PaymentError>;
}


pub struct PaymentDaoImpl {
    db: PgPool,
}

impl PaymentDaoImpl {
    pub fn new(db: PgPool) -> Self {
        PaymentDaoImpl { db }
    }
}

#[derive(Error, Debug)]
pub enum PaymentError {
    InvalidInput(String),
    Other,
    DatabaseError,
}

impl Display for PaymentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentError::InvalidInput(msg) => f.write_str(msg),
            PaymentError::Other => f.write_str("Something went wrong! Try again!"),
            PaymentError::DatabaseError => f.write_str("Error in database occurred."),
        }
    }
}


#[async_trait]
impl PaymentDao for PaymentDaoImpl {
    async fn create_payment(&self, payment: PaymentDto, owner_id: i32) -> Result<Payment, DBError> {
        let record = sqlx::query!(
            r#"
                  INSERT INTO payments (card_holder, card_number, security_code, expiration_month, expiration_year, name, color, note, owner_id)
                  VALUES ($1, $2, $3, $4, $5, $6, $7,$8, $9 )
                  RETURNING id, card_holder, card_number, security_code, expiration_month, expiration_year, name, color, note, owner_id
            "#,
            payment.card_holder,
            payment.card_number,
            payment.security_code,
            payment.expiration_month,
            payment.expiration_year,
            payment.name,
            payment.color,
            payment.note,
            owner_id
        ).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        return Ok(
            Payment {
                id: record.id,
                card_holder: record.card_holder.to_string(),
                card_number: record.card_number.to_string(),
                security_code: record.security_code,
                expiration_month: record.expiration_month,
                expiration_year: record.expiration_year,
                name: record.name.to_string(),
                color: record.color.to_string(),
                note: record.note.unwrap_or("".to_string()),
            }
        );
    }


    async fn update_payment(&self, id: i32, payment_dto: PaymentDto) -> Result<Payment, PaymentError> {
        let mut payment = self.get_payment(id).await?;

        if let Some(card_holder) = payment_dto.card_holder {
            payment.card_holder = card_holder;
        }
        if let Some(card_number) = payment_dto.card_number {
            payment.card_number = card_number;
        }
        if let Some(security_code) = payment_dto.security_code {
            payment.security_code = security_code;
        }
        if let Some(expiration_month) = payment_dto.expiration_month {
            payment.expiration_month = expiration_month;
        }
        if let Some(expiration_year) = payment_dto.expiration_year {
            payment.expiration_year = expiration_year;
        }
        if let Some(name) = payment_dto.name {
            payment.name = name;
        }
        if let Some(color) = payment_dto.color {
            payment.color = color;
        }
        if let Some(note) = payment_dto.note {
            payment.note = note;
        }

        sqlx::query!(r#"
            Update payments set card_holder = $1, card_number = $2, security_code = $3, expiration_month = $4, expiration_year = $5, name = $6,color = $7, note = $8
            where id = $9
        "#,
            payment.card_holder,
            payment.card_number,
            payment.security_code,
            payment.expiration_month,
            payment.expiration_year,
            payment.name,
            payment.color,
            payment.note,
            id
        ).execute(&self.db)
            .await
            .change_context(DBError::Other2)
            .change_context(PaymentError::DatabaseError)?;


        return Ok(payment);
    }


    async fn get_payment(&self, id: i32) -> Result<Payment, PaymentError> {
        let record = sqlx::query!(r#"SELECT * FROM payments WHERE id = $1"#, id).fetch_one(&self.db)
            .await
            .change_context(DBError::Other2)
            .change_context(PaymentError::DatabaseError)?;


        return Ok(Payment {
            id: record.id,
            card_holder: record.card_holder.to_string(),
            card_number: record.card_number.to_string(),
            security_code: record.security_code,
            expiration_month: record.expiration_month,
            expiration_year: record.expiration_year,
            name: record.name.to_string(),
            color: record.color.to_string(),
            note: record.note.unwrap_or("".to_string()),
        });
    }

    async fn get_payments(&self, owner_id: i32) -> Result<Vec<Payment>, PaymentError> {
        let record = sqlx::query!(r#"
                SELECT * FROM payments WHERE owner_id = $1
            "#, owner_id ).fetch_all(&self.db)
            .await
            .change_context(DBError::Other2)
            .change_context(PaymentError::DatabaseError)?;

        return Ok(record.iter().map(|r| Payment {
            id: r.id,
            card_holder: r.card_holder.to_string(),
            card_number: r.card_number.to_string(),
            security_code: r.security_code,
            expiration_month: r.expiration_month,
            expiration_year: r.expiration_year,
            name: r.name.to_string(),
            color: r.color.to_string(),
            note: r.note.to_owned().unwrap_or("".to_string()),
        }).collect());
    }

    async fn delete_payment(&self, payment_id: i32) -> Result<(), PaymentError> {
        sqlx::query!(r#"DELETE FROM logins WHERE id = $1"#, payment_id).execute(&self.db).await
            .change_context(DBError::Other2)
            .attach_printable(format!("Deleting Payment method with id {} failed", payment_id))
            .change_context(PaymentError::DatabaseError)?;

        Ok(())
    }
}
