use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RequestLoginUser {
    username: Arc<str>,
    password: Arc<str>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ResponseLoginUser {
    Accepted { username: String, token: String },
    Incorrect { username: String },
}

impl RequestLoginUser {
    pub fn new(username: impl Into<Arc<str>>, password: impl Into<Arc<str>>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl ResponseLoginUser {
    pub fn new_accepted(username: impl Into<String>, token: impl Into<String>) -> Self {
        Self::Accepted {
            username: username.into(),
            token: token.into(),
        }
    }
    pub fn new_invalid(username: impl Into<String>) -> Self {
        Self::Incorrect {
            username: username.into(),
        }
    }
    pub fn username(&self) -> &str {
        match self {
            Self::Accepted { username, .. } => &username,
            Self::Incorrect { username } => &username,
        }
    }
    pub fn token(&self) -> Option<&str> {
        match self {
            Self::Accepted { token, .. } => Some(&token),
            _ => None,
        }
    }
}
