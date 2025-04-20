use std::net::IpAddr;

use sqlx::PgPool;

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
