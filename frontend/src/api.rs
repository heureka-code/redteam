mod config;
use config::get_database_url;
use serde::de::DeserializeOwned;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum ApiLoginError {
    #[error("Conversion of request to json failed: {0}")]
    ReqConv(serde_json::Error),
    #[error("Response failed {0}")]
    RespError(#[from] reqwasm::Error),
    #[error("Credentials invalid")]
    Credentials,
}

impl From<ApiError> for ApiLoginError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::ReqConv(e) => Self::ReqConv(e),
            ApiError::RespError(e) => Self::RespError(e),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Conversion of request to json failed: {0}")]
    ReqConv(#[from] serde_json::Error),
    #[error("Response failed {0}")]
    RespError(#[from] reqwasm::Error),
}

async fn send_json<R: DeserializeOwned>(
    suffix: &str,
    body: impl serde::Serialize,
) -> Result<R, ApiError> {
    let body = serde_json::to_string(&body)?;

    let response = reqwasm::http::Request::post(&format!("{}/{}", get_database_url(), suffix))
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await?
        .json::<R>()
        .await?;
    Ok(response)
}

pub async fn api_login(
    username: String,
    password: String,
) -> Result<common::ResponseLoginUser, ApiLoginError> {
    let body = common::RequestLoginUser::new(username, password);
    send_json::<Option<_>>("login-user", body)
        .await?
        .ok_or(ApiLoginError::Credentials)
}

pub async fn api_get_username(token: String) -> Option<common::ResponseUsername> {
    let body = common::RequestUsername::new(token);
    send_json("get-username", body).await.ok()
}

pub async fn api_get_pets(
    token: impl Into<std::sync::Arc<str>>,
    name_pattern: impl Into<std::sync::Arc<str>>,
) -> Option<common::ResponsePettable> {
    let body = common::RequestPettable::new(token, name_pattern);
    send_json("get-pettable", body).await.ok()
}

pub async fn api_crack_password(
    username: impl Into<Arc<str>>,
    password_hash: impl Into<Arc<str>>,
) -> Option<common::ResponseHashcracking> {
    let body = common::RequestHashcracking::new(username, password_hash);
    send_json("get-cracked", body).await.ok()
}
