use std::net::IpAddr;

use serde::Deserialize;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Deserialize, Debug, Default)]
#[allow(unused)]
pub struct Level {
    pub game_version: i32,
    pub binary_version: i32,
    pub username: String,
    pub level_id: i32,
    pub level_name: String,
    pub level_desc: String,
    pub level_version: i32,
    pub level_length: i32,
    pub audio_track: i32,
    pub auto: i32,
    pub password: i32,
    pub original: i32,
    pub two_player: i32,
    pub song_id: i32,
    pub song_ids: Option<String>,
    pub sfx_ids: Option<String>,
    pub objects: i32,
    pub coins: i32,
    pub requested_stars: i32,
    pub extra_string: String,
    pub level_string: Option<String>,
    pub level_info: String,
    pub secret: String,
    pub star_difficulty: i32,
    pub downloads: i32,
    pub likes: i32,
    pub star_demon: i32,
    pub star_auto: i16,
    pub star_stars: i32,
    pub upload_date: i64,
    pub update_date: i64,
    pub rate_date: i64,
    pub star_coins: i32,
    pub star_featured: i32,
    pub star_hall: i32,
    pub star_epic: i32,
    pub star_demon_diff: i32,
    pub user_id: i32,
    pub ext_id: String,
    pub unlisted: i32,
    pub original_reup: i32,
    pub hostname: String,
    pub is_cp_shared: i32,
    pub is_deleted: i32,
    pub is_ldm: i32,
    pub unlisted2: i32,
    pub wt: i32,
    pub wt2: i32,
    pub ts: i32,
    pub settings_string: String,
}

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

pub async fn get_level_by_id(db: &PgPool, id: i32) -> Result<Level, sqlx::Error> {
    sqlx::query_as!(
        Level,
        "SELECT * FROM levels WHERE level_id = $1 AND is_deleted = 0",
        id
    )
    .fetch_one(db)
    .await
}
