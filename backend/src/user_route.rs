use super::jwt_token::{valid_user_token, JwtKey};
use actix_web::web::{self, Json};
use sqlx::MySqlPool;

#[actix_web::post("/get-username")]
async fn get_username(
    userdata: web::Json<common::RequestUsername>,
    db: web::Data<MySqlPool>,
    jwt_key: web::Data<JwtKey>,
) -> actix_web::Result<web::Json<common::ResponseUsername>> {
    let db = db.as_ref();

    let userdata = valid_user_token(&jwt_key, userdata.token()).await?;
    let user_id = userdata.user_id();

    let username: String = sqlx::query_scalar("SELECT username FROM users WHERE id=?")
        .bind(user_id)
        .fetch_optional(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound("No user with id found"))?;

    Ok(Json(common::ResponseUsername::new(username)))
}
