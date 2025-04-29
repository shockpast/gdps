use axum::{Extension, Router, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use rand::prelude::*;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{types::response::CommonResponse, utilities};

use super::COMMON_SECRET;

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RewardsRequest {
    #[serde(rename = "gameVersion")]
    game_version: i32,
    #[serde(rename = "binaryVersion")]
    binary_version: i32,
    #[serde(rename = "udid")]
    id: String,
    #[serde(rename = "uuid")]
    user_id: i32,
    #[serde(rename = "accountID")]
    account_id: i32,
    #[serde(rename = "gjp2")]
    hash: String,
    #[serde(rename = "rewardType")]
    reward_type: Option<i32>, // TODO: enum
    secret: String,
    #[serde(rename = "chk")]
    checksum: Option<String>, // checksum? used for security purposes or whatever
    r1: Option<i32>,
    r2: Option<i32>, // beep boop, star wars mf
}

async fn get_rewards(
    Extension(db): Extension<PgPool>,
    Form(data): Form<RewardsRequest>,
) -> impl IntoResponse {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }

    let checksum = data.checksum.unwrap_or_default();
    let reward_type = data.reward_type.unwrap_or_default();
    let hash = data.hash;

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != hash {
        return "-1".into_response();
    }

    let user = match utilities::database::get_user_by_id(&db, data.account_id).await {
        Some(user) => user,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    let mut rng = StdRng::from_os_rng();
    let current_time = chrono::Utc::now().timestamp();
    let current_time = current_time + 100;

    let mut chest1_count = user.chest1_count;
    let mut chest2_count = user.chest2_count;

    let chest1_difference = current_time - user.chest1_time as i64;
    let chest2_difference = current_time - user.chest2_time as i64;

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
            user.user_id
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

    let decoded_checksum = utilities::crypto::decode_base64_url_raw(&checksum[5..]);
    let decoded_checksum =
        utilities::crypto::cyclic_xor(&String::from_utf8_lossy(&decoded_checksum), "59182");

    let response = format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        utilities::rand_ascii(5),
        data.user_id,
        decoded_checksum,
        data.id,
        data.account_id,
        chest1_left,
        chest1_stuff,
        chest1_count,
        chest2_left,
        chest2_stuff,
        chest2_count,
        reward_type
    );

    let xor_reward = utilities::crypto::cyclic_xor(&response, "59182");
    let b64_reward = utilities::crypto::encode_base64_url(&xor_reward);
    let hash_reward = utilities::crypto::sha1_salt(&response, "pC26fpYaQCtg");

    format!("{}|{}", b64_reward, hash_reward).into_response()
}

pub fn init() -> Router {
    Router::new().route("/database/getGJRewards.php", post(get_rewards))
}
