use common::UserToken;
use jwt::VerifyWithKey;
use sqlx::MySqlPool;

pub struct JwtKey(hmac::Hmac<sha2::Sha256>);

impl JwtKey {
    pub fn new() -> Result<Self, jwt::Error> {
        use hmac::Mac;
        let envvar = std::env::var("JWT_KEY").unwrap_or("".into());
        let key = envvar.trim();
        let key = if key == "" {
            use rand::{distributions::Alphanumeric, Rng};
            log::warn!("Jwt key is set randomly on every boot. Please set environment variable JWT_KEY to a random token");

            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(39)
                .map(char::from)
                .collect()
        } else {
            log::info!("Jwt key from environment variable is used");
            key.to_owned()
        };
        Ok(Self(hmac::Hmac::new_from_slice(key.as_bytes())?))
    }
}

async fn validate_user_token(
    jwt_key: &JwtKey,
    token: &str,
) -> Result<Option<UserToken>, actix_web::Error> {
    token
        .verify_with_key(&jwt_key.0)
        .map(|user: UserToken| {
            let current_time = chrono::offset::Local::now();
            let in_time = current_time
                .signed_duration_since(user.generation_time())
                .abs()
                < chrono::TimeDelta::days(1);
            in_time.then_some(user)
        })
        .map_err(|err| {
            log::error!("Verification of jwt failed: {err}");
            actix_web::error::ErrorUnauthorized(err)
        })
}
pub async fn valid_user_token(
    jwt_key: &JwtKey,
    token: &str,
) -> Result<UserToken, actix_web::Error> {
    match validate_user_token(jwt_key, token).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(actix_web::error::ErrorUnauthorized("")),
        Err(err) => Err(err),
    }
}

pub async fn get_user_token(
    db: &MySqlPool,
    jwt_key: &JwtKey,
    username: &str,
) -> actix_web::Result<String> {
    async fn set_last_gen_time(
        executor: &MySqlPool,
        name: &str,
        t: chrono::DateTime<chrono::Local>,
    ) {
        if let Err(err) = sqlx::query("UPDATE users SET last_token_generation=? WHERE username=?")
            .bind(t)
            .bind(name)
            .execute(executor)
            .await
        {
            log::error!("Setting of last token generation for '{name}' failed: {err}");
        }
    }

    let user_id: i64 = sqlx::query_scalar("SELECT id FROM users WHERE username=?")
        .bind(username)
        .fetch_optional(db)
        .await
        .map_err(|err| {
            log::error!("No user found for name: '{username}'");
            actix_web::error::ErrorInternalServerError(err)
        })?
        .ok_or(actix_web::error::ErrorInternalServerError("No user found"))?;

    let last_gen: Option<chrono::DateTime<chrono::Local>> =
        sqlx::query_scalar("SELECT last_token_generation FROM users WHERE username=? AND last_token_generation is not null")
            .bind(username)
            .fetch_optional(db)
            .await
            .map_err(|err| {
                log::error!("Creation of token failed: {err:?}");
                actix_web::error::ErrorInternalServerError(err)
            })?;
    let current_time = chrono::offset::Local::now();

    use jwt::SignWithKey;
    let time2use = match last_gen {
        Some(last_gen) => {
            let near_enough =
                current_time.signed_duration_since(last_gen).abs() < chrono::TimeDelta::minutes(10);
            if near_enough {
                last_gen
            } else {
                set_last_gen_time(db, username, current_time).await;
                current_time
            }
        }
        None => {
            set_last_gen_time(db, username, current_time).await;
            current_time
        }
    };
    let claims = common::UserToken::new(user_id, time2use);
    let token = claims
        .sign_with_key(&jwt_key.0)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(token)
}
