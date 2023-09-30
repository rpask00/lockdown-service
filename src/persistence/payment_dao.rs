use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::DBError;
use crate::models::payment_model::{Payment, PaymentDto};

#[async_trait]
pub trait PaymentDao {
    async fn create_payment(&self, payment: PaymentDto) -> Result<Payment, DBError>;
    async fn get_payment(&self, id: i32) -> Result<Payment, DBError>;
}


pub struct PaymentDaoImpl {
    db: PgPool,
}

impl PaymentDaoImpl {
    pub fn new(db: PgPool) -> Self {
        PaymentDaoImpl { db }
    }
}


#[async_trait]
impl PaymentDao for PaymentDaoImpl {
    async fn create_payment(&self, payment: PaymentDto) -> Result<Payment, DBError> {
        let record = sqlx::query!(
            r#"
                  INSERT INTO payments (card_holder, card_number, security_code, expiration_month, expiration_year, name, color, note)
                  VALUES ($1, $2, $3, $4, $5, $6, $7,$8 )
                  RETURNING id, card_holder, card_number, security_code, expiration_month, expiration_year, name, color, note
            "#,
            payment.card_holder,
            payment.card_number,
            payment.security_code,
            payment.expiration_month,
            payment.expiration_year,
            payment.name,
            payment.color,
            payment.note,
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

    async fn get_payment(&self, id: i32) -> Result<Payment, DBError> {
        println!("{id}");
        let record = sqlx::query!(r#"    SELECT * FROM payments WHERE id = $1"#, id).fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;


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
}
