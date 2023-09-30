extern crate pretty_env_logger;

use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::launch;
use sqlx::postgres::PgPoolOptions;

pub use handlers::*;

use crate::cors::CORS;
use crate::persistence::auth_dao::{AuthDao, AuthDaoImpl};
use crate::persistence::login_dao::{LoginDao, LoginDaoImpl};
use crate::persistence::payment_dao::{PaymentDao, PaymentDaoImpl};
use crate::persistence::users_dao::{UsersDao, UsersDaoImpl};

mod cors;
mod models;
mod handlers;
mod persistence;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let secret_key = &std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found");
    let jwt_encoding_key = EncodingKey::from_secret(secret_key.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(secret_key.as_bytes());


    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let users_dao = UsersDaoImpl::new(pool.clone());
    let auth_dao = AuthDaoImpl::new(pool.clone());
    let login_dao = LoginDaoImpl::new(pool.clone());
    let payment_dao = PaymentDaoImpl::new(pool.clone());

    rocket::build()
        .mount(
            "/api",
            app_routes(),
        )
        .attach(CORS)
        .manage(Box::new(users_dao) as Box<dyn UsersDao + Send + Sync>)
        .manage(Box::new(auth_dao) as Box<dyn AuthDao + Send + Sync>)
        .manage(Box::new(login_dao) as Box<dyn LoginDao + Send + Sync>)
        .manage(Box::new(payment_dao) as Box<dyn PaymentDao + Send + Sync>)
        .manage(jwt_encoding_key)
        .manage(jwt_decoding_key)
}
