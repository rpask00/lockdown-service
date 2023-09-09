use rocket::{launch, routes};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::cors::CORS;
use crate::persistence::users_dao::{UsersDao, UsersDaoImpl};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
mod cors;
mod models;
mod handlers;
mod persistence;

extern crate pretty_env_logger;
pub use handlers::*;


#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let secret_key = &std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found");
    let jwt_secret = secret_key.as_bytes();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret);
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret);


    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let users_dao = UsersDaoImpl::new(pool.clone());

    rocket::build()
        .mount(
            "/",
            app_routes(),
        )
        .attach(CORS)
        .manage(Box::new(users_dao) as Box<dyn UsersDao + Send + Sync>)
        .manage(jwt_encoding_key)
        .manage(jwt_decoding_key)
}
