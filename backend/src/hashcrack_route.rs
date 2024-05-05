use super::db_connect::SecretPool;
use actix_web::web::{self, Json};
use sqlx::MySqlPool;

#[actix_web::post("/get-cracked")]
async fn get_cracked(
    data: web::Json<common::RequestHashcracking>,
    db: web::Data<SecretPool>,
) -> actix_web::Result<web::Json<common::ResponseHashcracking>> {
    let db: &MySqlPool = &db;

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let password: Option<String> = sqlx::query_scalar(
        "SELECT password FROM passwords WHERE username=? AND password_hash=? AND is_admin=true",
    )
    .bind(data.username())
    .bind(data.password_hash())
    .fetch_optional(db)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Json(if let Some(password) = password {
        common::ResponseHashcracking::new_accepted(data.username(), password, data.password_hash())
    } else {
        common::ResponseHashcracking::new_invalid(data.username())
    }))
}
