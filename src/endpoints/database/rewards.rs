use axum::{Extension, Router, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use serde::Deserialize;
use sqlx::PgPool;

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
    reward_type: Option<i32>, // TODO: enum
    secret: Option<String>,
    chk: Option<String>, // checksum? used for security purposes or whatever
    r1: Option<i32>,
    r2: Option<i32>, // beep boop, star wars mf
}

async fn get_rewards(
    Extension(_db): Extension<PgPool>,
    Form(_data): Form<RewardsRequest>,
) -> impl IntoResponse {
    "-1"
}

pub fn init() -> Router {
    Router::new().route("/database/getGJRewards.php", post(get_rewards))
}
