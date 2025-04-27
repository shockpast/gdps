use axum::{Extension, Router, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use serde::Deserialize;
use sqlx::PgPool;

use crate::types::{database::User, response::CommonResponse};

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
struct GetScoresRequest {
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
    #[serde(rename = "type")]
    search_type: String,
    count: i32,
    secret: String,
}

async fn get_scores(
    Extension(db): Extension<PgPool>,
    Form(data): Form<GetScoresRequest>,
) -> impl IntoResponse {
    let mut user_results = Vec::new();

    match data.search_type.as_str() {
        "top" => {
            let users = sqlx::query_as!(
                User,
                r#"
                SELECT * FROM users
                    WHERE stars >= 1
                      AND is_banned = 0
                      AND ip NOT IN (SELECT ip FROM banned_ips)
                ORDER BY stars + moons DESC
                LIMIT 100
            "#
            )
            .fetch_all(&db)
            .await
            .unwrap();

            user_results = users;
        }
        "creators" => {
            let creators = sqlx::query_as!(
                User,
                r#"
                SELECT * FROM users
                    WHERE creator_points > 0
                      AND is_creator_banned = 0
                      AND ip NOT IN (SELECT ip FROM banned_ips)
                ORDER BY creator_points DESC
                LIMIT 100
            "#
            )
            .fetch_all(&db)
            .await
            .unwrap();

            user_results = creators;
        }
        "relative" => {
            let relatives = sqlx::query_as!(
                User,
                r#"
                WITH current_star AS (
                    SELECT stars, moons
                    FROM users
                    WHERE ext_id = $1
                )

                SELECT * FROM users
                    WHERE (stars BETWEEN (SELECT stars FROM current_user) - 1000 AND (SELECT stars FROM current_user) + 1000)
                      AND (moons BETWEEN (SELECT moons FROM current_user) - 500 AND (SELECT moons FROM current_user) + 500)
                ORDER BY (ABS(stars - (SELECT stars FROM current_user)) + ABS(moons - (SELECT moons FROM current_user)))
                LIMIT 50
            "#, data.account_id.to_string())
                .fetch_all(&db)
                .await
                .unwrap();

            user_results = relatives;
        }
        "friends" => {
            let user_friends = sqlx::query!(
                "SELECT * FROM friendships WHERE person1 = $1 OR person2 = $1 LIMIT 50",
                data.account_id
            )
            .fetch_all(&db)
            .await
            .unwrap();

            let mut friend_ids = Vec::new();
            for friend in user_friends {
                if friend.person2 == data.account_id {
                    friend_ids.push(friend.person1.to_string());
                } else {
                    friend_ids.push(friend.person2.to_string());
                }
            }

            let friends = sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE ext_id IN ($1, $2) ORDER BY stars DESC",
                &data.account_id.to_string(),
                friend_ids.join(", ")
            )
            .fetch_all(&db)
            .await
            .unwrap();

            user_results = friends;
        }
        &_ => (),
    }

    let mut leaderboard_string = String::new();
    let mut leaderboard_place = 0;

    for user in user_results {
        leaderboard_place += 1;

        let user_string = format!(
            "1:{}:2:{}:13:{}:17:{}:6:{}:9:{}:10:{}:11:{}:51:{}:14:{}:15:{}:16:{}:3:{}:8:{}:4:{}:7:{}:46:{}:52:{}|",
            user.username,
            user.user_id,
            user.coins,
            user.user_coins,
            leaderboard_place,
            user.icon,
            user.color1,
            user.color2,
            user.color3,
            user.icon_type,
            user.special,
            data.account_id,
            user.stars,
            user.creator_points.round(),
            user.demons,
            data.account_id,
            user.diamonds,
            user.moons
        );
        leaderboard_string.push_str(&user_string);
    }

    if leaderboard_string.is_empty() {
        return CommonResponse::InvalidRequest.into_response();
    }

    leaderboard_string.into_response()
}

pub fn init() -> Router {
    Router::new().route("/database/getGJScores20.php", post(get_scores))
}
