use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct UserToken {
    user_id: i64,
    generation_time: Arc<chrono::DateTime<chrono::Local>>,
}

impl UserToken {
    pub fn new(
        user_id: impl Into<i64>,
        t: impl Into<Arc<chrono::DateTime<chrono::Local>>>,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            generation_time: t.into(),
        }
    }
    pub fn generation_time(&self) -> &chrono::DateTime<chrono::Local> {
        &self.generation_time
    }
    pub fn user_id(&self) -> &i64 {
        &self.user_id
    }
}
