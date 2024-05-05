use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RequestUsername {
    token: Arc<str>,
}

impl RequestUsername {
    pub fn new(token: impl Into<Arc<str>>) -> Self {
        Self {
            token: token.into(),
        }
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ResponseUsername {
    username: Arc<str>,
}

impl ResponseUsername {
    pub fn new(username: impl Into<Arc<str>>) -> Self {
        Self {
            username: username.into(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
}
