use axum::{Extension, Router, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use rand::prelude::*;
use serde::Deserialize;
use sqlx::PgPool;

use crate::utilities::crypto;

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RewardsRequest {
    #[serde(rename = "gameVersion")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i32>,
    #[serde(rename = "udid")]
    id: Option<String>,
    #[serde(rename = "uuid")]
    user_id: Option<i32>,
    #[serde(rename = "accountID")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
    #[serde(rename = "rewardType")]
    reward_type: Option<i32>, // TODO: enum
    secret: Option<String>,
    chk: Option<String>, // checksum? used for security purposes or whatever
    r1: Option<i32>,
    r2: Option<i32>, // beep boop, star wars mf
}

async fn get_rewards(
    Extension(db): Extension<PgPool>,
    Form(data): Form<RewardsRequest>,
) -> impl IntoResponse {
    if data.account_id.is_none() {
        return "-1".into_response();
    }

    // let checksum = data.chk.unwrap();
    let account_id = data.account_id.unwrap();
    let reward_type = data.reward_type.unwrap();
    let hash = data.hash.unwrap();

    let account = sqlx::query!("SELECT * FROM accounts WHERE account_id = $1", account_id)
        .fetch_one(&db)
        .await;

    if account.is_err() {
        return "-1".into_response();
    }

    let account = account.unwrap();
    if account.gjp2.unwrap_or_default() != hash {
        return "-1".into_response();
    }

    let user = sqlx::query!("SELECT * FROM users WHERE username = $1", account.username)
        .fetch_one(&db)
        .await
        .unwrap();

    let chests_data = sqlx::query!(
        "SELECT chest1_time, chest2_time, chest1_count, chest2_count FROM users WHERE ext_id = $1",
        account_id.to_string()
    )
    .fetch_one(&db)
    .await
    .map_err(|_| "-1".into_response())
    .unwrap();

    let mut rng = StdRng::from_os_rng();
    let current_time = chrono::Utc::now().timestamp();
    let current_time = current_time + 100;

    let mut chest1_count = chests_data.chest1_count;
    let mut chest2_count = chests_data.chest2_count;

    let chest1_difference = current_time - chests_data.chest1_time as i64;
    let chest2_difference = current_time - chests_data.chest2_time as i64;

    let chest1_items = [1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 14];
    let chest2_items = [1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 14];

    // TODO: config/rewards.rs (or something similar)
    // Orbs, Diamonds, Items, Keys
    let chest1_stuff = format!(
        "{},{},{},{}",
        rng.random_range(200..=400),
        rng.random_range(2..=10),
        chest1_items.choose(&mut rng).unwrap(),
        rng.random_range(1..=6)
    );
    // Orbs, Diamonds, Items, Keys
    let chest2_stuff = format!(
        "{},{},{},{}",
        rng.random_range(200..=400),
        rng.random_range(2..=10),
        chest2_items.choose(&mut rng).unwrap(),
        rng.random_range(1..=6)
    );

    // 3600 = Wooden Chest, 14400 = Golden Chest
    let chest1_left = std::cmp::max(0, 3600 - chest1_difference);
    let chest2_left = std::cmp::max(0, 14400 - chest2_difference);

    if reward_type == 1 {
        if chest1_left != 0 {
            return "-1".into_response();
        }

        chest1_count += 1;

        sqlx::query!(
            r#"
            UPDATE users SET chest1_count = chest1_count + 1, chest1_time = $1 WHERE user_id = $2
        "#,
            current_time as i32,
            account_id
        )
        .execute(&db)
        .await
        .unwrap();
    } else if reward_type == 2 {
        if chest2_left != 0 {
            return "-1".into_response();
        }

        chest2_count += 1;

        sqlx::query!(
            r#"
            UPDATE users SET chest2_count = chest2_count + 1, chest2_time = $1 WHERE user_id = $2
        "#,
            current_time as i32,
            user.user_id
        )
        .execute(&db)
        .await
        .unwrap();
    }

    let reward = format!(
        "1:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        user.user_id,
        "",
        data.id.unwrap_or_default(),
        account_id,
        chest1_left,
        chest1_stuff,
        chest1_count,
        chest2_left,
        chest2_stuff,
        chest2_count,
        reward_type
    );

    let xor_reward = crypto::xor_cipher(reward.as_bytes(), b"59182");
    let b64_reward = crypto::encode_base64_url(&xor_reward);
    let hash_reward = crypto::sha1_salt(&b64_reward, "pC26fpYaQCtg");

    format!("{}|{}", b64_reward, hash_reward).into_response()
}

pub fn init() -> Router {
    Router::new().route("/database/getGJRewards.php", post(get_rewards))
}
