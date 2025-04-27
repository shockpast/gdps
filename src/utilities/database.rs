use std::net::IpAddr;

use sqlx::PgPool;

use crate::types::database::{Account, Level, User};

// TODO: rewrite get_user_id function
pub async fn get_user_id(
    db: &PgPool,
    id: &String,
    username: &str,
    ip: &IpAddr,
) -> Result<i32, sqlx::Error> {
    if let Some(user_id) = sqlx::query_scalar!("SELECT user_id FROM users WHERE ext_id = $1", id)
        .fetch_optional(db)
        .await?
    {
        Ok(user_id)
    } else {
        let is_registered = id.parse::<i32>().is_ok();

        let row = sqlx::query!(
            r#"
            INSERT INTO users (is_registered, ext_id, username, last_played, ip)
            VALUES ($1, $2, $3, $4, $5) RETURNING user_id
        "#,
            is_registered as i32,
            id,
            username,
            chrono::Utc::now().timestamp() as i32,
            ip.to_string()
        )
        .fetch_one(db)
        .await?;

        Ok(row.user_id)
    }
}

pub async fn get_account_by_username(db: &PgPool, username: &str) -> Option<Account> {
    sqlx::query_as!(
        Account,
        "SELECT * FROM accounts WHERE username = $1",
        username
    )
    .fetch_optional(db)
    .await
    .unwrap()
}

pub async fn get_account_by_id(db: &PgPool, id: i32) -> Option<Account> {
    sqlx::query_as!(Account, "SELECT * FROM accounts WHERE account_id = $1", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn get_user_by_id(db: &PgPool, id: i32) -> Option<User> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE ext_id = $1 OR user_id = $1::INT",
        id.to_string()
    )
    .fetch_optional(db)
    .await
    .unwrap()
}

pub async fn get_level_by_id(db: &PgPool, id: i32) -> Option<Level> {
    sqlx::query_as!(
        Level,
        "SELECT * FROM levels WHERE level_id = $1 AND is_deleted = 0",
        id
    )
    .fetch_optional(db)
    .await
    .unwrap()
}
