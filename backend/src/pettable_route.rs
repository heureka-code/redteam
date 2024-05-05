use crate::db_connect::ReadonlyPool;

use super::jwt_token::{valid_user_token, JwtKey};
use actix_web::web::{self, Json};

#[actix_web::post("/get-pettable")]
async fn get_pettable(
    data: web::Json<common::RequestPettable>,
    readonly: web::Data<ReadonlyPool>,
    jwt_key: web::Data<JwtKey>,
) -> actix_web::Result<web::Json<common::ResponsePettable>> {
    let readonly = readonly.as_ref();

    let token = valid_user_token(&jwt_key, data.token()).await?;
    let user_id = token.user_id();

    let pets = super::db_setup::get_pettable4user(readonly, *user_id, data.name_pattern())
        .await
        .map_err(|err| {
            log::error!(
                "Pet search failed: owner_id={user_id}, name_pattern={}",
                data.name_pattern()
            );
            actix_web::error::ErrorInternalServerError(err)
        })?;
    log::info!(
        "Name pattern ({}) produced following pets: {pets:?}",
        data.name_pattern()
    );

    Ok(Json(common::ResponsePettable::new(
        data.name_pattern(),
        pets,
    )))
}
