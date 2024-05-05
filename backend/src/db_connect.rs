use std::ops::Deref;

use sqlx::{mysql::MySqlConnectOptions, MySql, MySqlPool, Pool};

pub(crate) async fn connect() -> Result<Pool<MySql>, sqlx::Error> {
    // env only works in docker when there are no spaces around '=' in Key=Value
    let db_user = std::env::var("DATABASE_USER").unwrap_or("root".into());
    let db_password = std::env::var("DATABASE_PASS").unwrap_or("password".into());
    let db_host = std::env::var("DATABASE_HOST")
        .expect("Sql db host given. Ensure there are no spaces around '=' with Key=Value for environment variables");
    let db_port = std::env::var("DATABASE_PORT").unwrap_or("3306".into());
    let db_name = std::env::var("DATABASE_NAME").unwrap_or("mysql".into());

    log::info!("Using: user={db_user}, password={db_password}, host={db_host}, port={db_port}, name={db_name}");

    let opts = MySqlConnectOptions::new()
        .host(&db_host)
        .username(&db_user)
        .password(&db_password)
        .port(db_port.parse().expect("Port has to be u16"))
        .database(&db_name);
    return MySqlPool::connect_with(opts).await;
}

pub(crate) struct ReadonlyPool(Pool<MySql>);

impl Deref for ReadonlyPool {
    type Target = Pool<MySql>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) async fn connect_readonly() -> Result<ReadonlyPool, sqlx::Error> {
    // env only works in docker when there are no spaces around '=' in Key=Value
    let db_user = "readonly";
    let db_password = "readonly";
    let db_host = std::env::var("DATABASE_HOST")
        .expect("Sql db host given. Ensure there are no spaces around '=' with Key=Value for environment variables");
    let db_port = std::env::var("DATABASE_PORT").unwrap_or("3306".into());
    let db_name = std::env::var("DATABASE_NAME").unwrap_or("mysql".into());

    log::info!("Using for readonly: user={db_user}, password={db_password}, host={db_host}, port={db_port}, name={db_name}");

    let opts = MySqlConnectOptions::new()
        .host(&db_host)
        .username(&db_user)
        .password(&db_password)
        .port(db_port.parse().expect("Port has to be u16"))
        .database(&db_name);
    return MySqlPool::connect_with(opts).await.map(ReadonlyPool);
}

pub(crate) struct SecretPool(Pool<MySql>);

impl Deref for SecretPool {
    type Target = Pool<MySql>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) async fn secret_connect() -> Result<SecretPool, sqlx::Error> {
    // env only works in docker when there are no spaces around '=' in Key=Value
    let get_var =
        |first_try, second_try| std::env::var(first_try).or_else(|_| std::env::var(second_try));

    let db_user = get_var("SECRET_USER", "DATABASE_USER").unwrap_or("root".into());
    let db_password = get_var("SECRET_PASS", "DATABASE_PASS").unwrap_or("password".into());
    let db_host = get_var("SECRET_HOST", "DATABASE_HOST")
        .expect("Sql secret db host given. Ensure there are no spaces around '=' with Key=Value for environment variables");
    let db_port = get_var("SECRET_PORT", "DATABASE_PORT").unwrap_or("3306".into());
    let db_name = get_var("SECRET_NAME", "DATABASE_NAME").unwrap_or("mysql".into());

    log::info!("Using (for secret db): user={db_user}, password={db_password}, host={db_host}, port={db_port}, name={db_name}");

    let opts = MySqlConnectOptions::new()
        .host(&db_host)
        .username(&db_user)
        .password(&db_password)
        .port(db_port.parse().expect("Port has to be u16"))
        .database(&db_name);
    return MySqlPool::connect_with(opts).await.map(SecretPool);
}
