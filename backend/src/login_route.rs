use crate::db_connect::ReadonlyPool;

use super::jwt_token::{get_user_token, JwtKey};
use actix_web::web::{self, Json};
use sqlx::MySqlPool;

#[actix_web::post("/login-user")]
async fn login_user(
    userdata: web::Json<common::RequestLoginUser>,
    db: web::Data<MySqlPool>,
    readonly: web::Data<ReadonlyPool>,
    jwt_key: web::Data<JwtKey>,
) -> actix_web::Result<web::Json<common::ResponseLoginUser>> {
    let userdata = userdata.0;
    let username = userdata.username();
    let password = userdata.password();

    let db: &MySqlPool = db.as_ref();
    let readonly: &MySqlPool = readonly.as_ref();

    let hashed_password = super::hash_password(password);

    use sqlx::Executor;
    use sqlx::Row;

    log::info!("Someone tries: user=['{username}'] query=[{}]", format!("SELECT username FROM users WHERE username='{username}' AND password_hash='{hashed_password}'").as_str());

    let maybe_hacked_username: Option<String> = readonly
            .fetch_optional(
                format!("SELECT username FROM users WHERE username='{username}' AND password_hash='{hashed_password}'").as_str(),
            )
            .await
            .map_err(|err| {
                log::error!(
                    "Unexpected error on searching for '{username}' (non admin): {err:?}"
                );
                actix_web::error::ErrorInternalServerError(err)
            })?
            .map(|row| row.get(0));

    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM users WHERE username=?")
        .bind(&maybe_hacked_username)
        .fetch_optional(readonly)
        .await
        .map_err(|err| {
            log::error!("Unexpected error on deciding whether '{username}' is admin: {err:?}");
            actix_web::error::ErrorInternalServerError(err)
        })?
        .unwrap_or(false);

    if is_admin {
        let found_users: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE username=? AND password_hash=?")
                .bind(username)
                .bind(&hashed_password)
                .fetch_one(readonly)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        if found_users == 0 {
            return Ok(Json(common::ResponseLoginUser::new_invalid(username)));
        }

        let token = get_user_token(db, &jwt_key, username, true).await?;
        return Ok(Json(common::ResponseLoginUser::new_accepted(
            username, token,
        )));
    } else {
        return if let Some(username) = maybe_hacked_username {
            let token = get_user_token(db, &jwt_key, &username, false).await?;
            Ok(Json(common::ResponseLoginUser::new_accepted(
                username, token,
            )))
        } else {
            Ok(Json(common::ResponseLoginUser::new_invalid(username)))
        };
    }
}
