use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Store, Default, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct AuthStore {
    pub token: Option<String>,
    pub username: Option<String>,
}
