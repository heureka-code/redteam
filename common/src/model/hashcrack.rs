use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RequestHashcracking {
    username: Arc<str>,
    password_hash: Arc<str>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ResponseHashcracking {
    Accepted {
        username: String,
        password: String,
        password_hash: String,
    },
    Incorrect {
        username: String,
    },
}

impl RequestHashcracking {
    pub fn new(username: impl Into<Arc<str>>, password_hash: impl Into<Arc<str>>) -> Self {
        Self {
            username: username.into(),
            password_hash: password_hash.into(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

impl ResponseHashcracking {
    pub fn new_accepted(
        username: impl Into<String>,
        password: impl Into<String>,
        password_hash: impl Into<String>,
    ) -> Self {
        Self::Accepted {
            username: username.into(),
            password: password.into(),
            password_hash: password_hash.into(),
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
    pub fn password(&self) -> Option<&str> {
        match self {
            Self::Accepted { password, .. } => Some(&password),
            _ => None,
        }
    }
}
