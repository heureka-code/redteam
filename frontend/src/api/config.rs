const RT_HOSTNAME: &'static str = std::env!(
    "RT_HOSTNAME",
    "please provide the env var 'RT_HOSTNAME' at build time"
);

pub fn get_database_url() -> std::sync::Arc<str> {
    format!("https://{RT_HOSTNAME}/redteam-backend").into()
}
