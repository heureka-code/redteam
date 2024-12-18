mod app_port;
mod db_connect;
mod db_setup;
mod hashcrack_route;
mod jwt_token;
mod login_route;
mod password_hashing;
mod pettable_route;
mod user_route;

use std::collections::BTreeSet;

use actix_web::{http, middleware::Logger, web, App, HttpServer};
use async_std::task;
use jwt_token::JwtKey;

use app_port::get_app_port;
use db_connect::{connect, secret_connect};
use hashcrack_route::get_cracked;
use login_route::login_user;
use password_hashing::hash_password;
use pettable_route::get_pettable;
use user_route::get_username;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    name: String,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();
    let port = get_app_port();

    let database = web::Data::new(
        task::block_on(connect()).expect("Url isn't valid for init user, no db found"),
    );

    let secret_database = web::Data::new(
        task::block_on(secret_connect()).expect("Url isn't valid, no secret db found"),
    );

    let jwt_key = web::Data::new(JwtKey::new().expect("Valid hmac"));

    let allowed_origin_suffixes = std::env::var("ALLOWED_ORIGIN_SUFFIXES").unwrap_or("".into());
    let allowed_origin_suffixes = allowed_origin_suffixes
        .split('|')
        .filter_map(|s| (s.trim() != "").then_some(s.to_owned()))
        .collect::<BTreeSet<_>>();
    let allowed_origin_suffixes = std::sync::Arc::new(allowed_origin_suffixes);

    log::info!("Allowed origin suffixes are: {allowed_origin_suffixes:?}");

    db_setup::setup_database(&database, &secret_database).await;

    let readonly_database = web::Data::new(
        task::block_on(crate::db_connect::connect_readonly())
            .expect("Url isn't valid for readonly user, no db found"),
    );

    HttpServer::new(move || {
        let logger = Logger::default();
        let allowed_origin_suffixes = allowed_origin_suffixes.clone();

        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                let org = origin.as_bytes();
                let allowed = allowed_origin_suffixes
                    .iter()
                    .any(|origin| org.ends_with(origin.as_bytes()));
                log::info!("(CORS check) tried origin is: {origin:?}, allowed={allowed}");
                allowed
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(logger)
            .wrap(cors)
            .service(login_user)
            .service(get_username)
            .service(get_pettable)
            .service(get_cracked)
            .app_data(database.clone())
            .app_data(jwt_key.clone())
            .app_data(secret_database.clone())
            .app_data(readonly_database.clone())
    })
    .bind(("0.0.0.0", port))?
    .workers(4)
    .run()
    .await
}
