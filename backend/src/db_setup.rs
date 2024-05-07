use futures::TryFutureExt;
use sqlx::MySqlPool;
use std::collections::HashMap;

async fn add_user_with_pwd_to_database(
    db: &MySqlPool,
    secret_db: &MySqlPool,
    username: &str,
    password: &str,
    is_admin: bool,
) {
    let password_hash = crate::hash_password(password);

    if let Err(err) =
        sqlx::query("INSERT INTO users(username, password_hash, is_admin) VALUES(?, ?, ?)")
            .bind(username)
            .bind(&password_hash)
            .bind(is_admin)
            .execute(db)
            .await
    {
        match err {
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    log::debug!("Adding of user '{username}' is skipped'")
                } else {
                    log::error!("Adding of user '{username}' failed with db error: {db_err:?}")
                }
            }
            _ => log::error!("Adding of user '{username}' failed: {err:?}"),
        }
    } else {
        if let Err(err) = sqlx::query(
            "INSERT INTO passwords(username, password, password_hash, is_admin) VALUES(?,?,?,?)",
        )
        .bind(username)
        .bind(password)
        .bind(password_hash)
        .bind(is_admin)
        .execute(secret_db)
        .await
        {
            match err {
                sqlx::Error::Database(db_err) => {
                    if db_err.is_unique_violation() {
                        log::debug!("Adding of user '{username}' to secrets is skipped'")
                    } else {
                        log::error!("Adding of user '{username}' to secrets failed with db error: {db_err:?}")
                    }
                }
                _ => log::error!("Adding of user '{username}' to secrets failed: {err:?}"),
            }
        }
    }
}
async fn add_user_to_database(
    db: &MySqlPool,
    secret_db: &MySqlPool,
    username: &str,
    is_admin: bool,
) {
    use rand::{distributions::Alphanumeric, Rng};

    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();
    log::info!("Add user {username} with {password} as admin={is_admin}");

    add_user_with_pwd_to_database(db, secret_db, username, &password, is_admin).await
}
async fn add_pet_to_database(db: &MySqlPool, owner_id: i64, name: &str, pettype: &str) {
    if let Err(err) = sqlx::query("INSERT INTO pets(owner_id, name, pettype) VALUES(?, ?, ?)")
        .bind(owner_id)
        .bind(name)
        .bind(pettype)
        .execute(db)
        .await
    {
        match err {
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    log::debug!("Adding of pet '{name}' skipped (owner={owner_id})")
                } else {
                    log::error!("Adding of pet '{name}' to database failed with db error: owner={owner_id}, {db_err:?}")
                }
            }
            _ => {
                log::error!("Adding of pet '{name}' to database failed: owner={owner_id}, {err:?}")
            }
        }
    }
}
async fn add_user_with_pets_to_database(
    db: &MySqlPool,
    secret_db: &MySqlPool,
    username: &str,
    pets: HashMap<&str, &str>,
    is_admin: bool,
) -> Option<()> {
    add_user_to_database(db, secret_db, username, is_admin).await;

    let owner_id = match sqlx::query_scalar("SELECT id FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(db)
        .await
    {
        Ok(Some(o)) => o,
        Ok(None) => return None,
        Err(err) => {
            log::error!("Reading owner id of '{username}' failed: {err:?}");
            return None;
        }
    };

    let fs = pets
        .iter()
        .map(|(name, type_)| add_pet_to_database(db, owner_id, name, &type_));

    futures::future::join_all(fs).await;
    Some(())
}

pub async fn get_pettable4user(
    db: &MySqlPool,
    owner_id: i64,
    name_pattern: &str,
) -> Result<Vec<common::Pet>, sqlx::Error> {
    use sqlx::Row;
    let name_pattern = if name_pattern == "" {
        "%"
    } else {
        name_pattern
    };

    let pets: Vec<(String, String)> = sqlx::raw_sql(&format!(
        "SELECT name, pettype FROM pets WHERE owner_id={owner_id} AND name LIKE '{name_pattern}'"
    ))
    .fetch_all(db)
    .map_ok(|rows| rows.iter().map(|row| (row.get(0), row.get(1))).collect())
    .await?;
    Ok(pets
        .into_iter()
        .map(|(name, pettype)| common::Pet::new(name, pettype))
        .collect())
}

pub async fn setup_database(pool: &MySqlPool, secret_db: &MySqlPool) {
    let _ = sqlx::query("DROP TABLE users").execute(pool).await;
    let _ = sqlx::query("DROP TABLE pets").execute(pool).await;
    let _ = sqlx::query("DROP TABLE passwords").execute(secret_db).await;

    log::info!("Tables get recreated!");

    log::info!(
        "Creation of table 'users': {:?}",
        sqlx::query(
            "CREATE TABLE users(
            id integer primary key auto_increment,
            username varchar(390) not null unique,
            password_hash varchar(500) not null,
            is_admin boolean not null default false,
            last_token_generation datetime
        )",
        )
        .execute(pool)
        .await
    );
    log::info!(
        "Creation of table 'pets': {:?}",
        sqlx::query(
            "CREATE TABLE pets(
                id integer primary key auto_increment,
                name varchar(390) not null,
                pettype varchar(100) not null,
                owner_id integer not null references users,
                registration_time datetime,
                CONSTRAINT unique_pet_per_owner UNIQUE(name, pettype, owner_id)
            )"
        )
        .execute(pool)
        .await
    );
    log::info!(
        "Creation of table 'secrets.passwords': {:?}",
        sqlx::query(
            "CREATE TABLE passwords(
                id integer primary key auto_increment,
                username varchar(390) not null unique,
                password varchar(1000) not null,
                password_hash varchar(500) not null,
                is_admin bool not null default false
            )"
        )
        .execute(secret_db)
        .await
    );

    log::info!(
        "Creating user with readonly access: {:?}",
        sqlx::raw_sql(
            "CREATE USER 'readonly'@'%' IDENTIFIED BY 'readonly';
         GRANT SELECT ON *.* TO 'readonly'@'%';
         FLUSH PRIVILEGES;"
        )
        .execute(pool)
        .await
    );
    futures::join!(
        add_user_with_pets_to_database(
            pool,
            secret_db,
            "Mia",
            HashMap::from([("Pfote", "Katze")]),
            false,
        ),
        add_user_with_pets_to_database(
            pool,
            secret_db,
            "Andreas",
            HashMap::from([("Cody", "Katze")]),
            false
        ),
        add_user_with_pets_to_database(
            pool,
            secret_db,
            "Arthur",
            HashMap::from([("Cuppy", "Hund"), ("Fluffy", "Hund")]),
            false
        ),
        add_user_with_pets_to_database(pool, secret_db, "Emma", HashMap::from([]), false),
        add_user_with_pets_to_database(
            pool,
            secret_db,
            "Annabeth",
            HashMap::from([("Leo", "Löwe"), ("Boots", "Affe"), ("Kiko", "Kaninchen")]),
            false
        ),
    );
    add_user_with_pets_to_database(
        pool,
        secret_db,
        "Percy",
        HashMap::from([("Mrs. O'Leary", "Hund"), ("Blackjack", "Pegasus")]),
        true,
    )
    .await;
}
