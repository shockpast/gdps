use std::fmt::Write;
use std::net::SocketAddr;

use axum::{Extension, Router, extract::ConnectInfo, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use chrono::Utc;
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;

use crate::utilities::{self, crypto};

// https://github.com/tokio-rs/axum/discussions/2380#discussioncomment-7705720
// luv luv!
pub fn take_first<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec: Vec<T> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().next())
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct UpdateRequest {
    #[serde(rename = "accountID")]
    account_id: Option<i32>,
    #[serde(rename = "userName")]
    username: Option<String>,
    secret: Option<String>,
    stars: Option<i32>,
    demons: Option<i32>,
    icon: Option<i32>,
    color1: Option<i32>,
    color2: Option<i32>,
    coins: Option<i32>,
    #[serde(rename = "iconType")]
    icon_type: Option<i32>,
    #[serde(rename = "userCoins")]
    user_coins: Option<i32>,
    special: Option<i32>,
    #[serde(rename = "accIcon")]
    accessory_icon: Option<i32>,
    #[serde(rename = "accShip")]
    accessory_ship: Option<i32>,
    #[serde(rename = "accBall")]
    accessory_ball: Option<i32>,
    #[serde(rename = "accBird")]
    accessory_bird: Option<i32>,
    #[serde(rename = "accDart")]
    accessory_dart: Option<i32>,
    #[serde(rename = "accRobot")]
    accessory_robot: Option<i32>,
    #[serde(rename = "accGlow")]
    accessory_glow: Option<i32>,
    #[serde(rename = "accSpider")]
    accessory_spider: Option<i32>,
    #[serde(rename = "accExplosion")]
    accessory_explosion: Option<i32>,
    #[serde(rename = "accSwing")]
    accessory_swing: Option<i32>,
    #[serde(rename = "accJetpack")]
    accessory_jetpack: Option<i32>,
    diamonds: Option<i32>,
    moons: Option<i32>,
    color3: Option<i32>,
    dinfo: Option<String>,
    dinfow: Option<i32>,
    dinfog: Option<i32>,
    sinfo: Option<String>,
    sinfod: Option<i32>,
    sinfog: Option<i32>,
    #[serde(default, rename = "gameVersion", deserialize_with = "take_first")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i32>,
    #[serde(rename = "udid")]
    id: Option<String>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct UserInfoRequest {
    #[serde(rename = "gameVersion")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i32>,
    #[serde(rename = "udid")]
    id: Option<String>,
    #[serde(rename = "accountID")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
    #[serde(rename = "targetAccountID")]
    target_account_id: Option<i32>,
    #[serde(rename = "secret")]
    secret: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct UserCommentsRequest {
    #[serde(rename = "gameVersion")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i32>,
    #[serde(rename = "udid")]
    id: Option<String>,
    #[serde(rename = "uuid")]
    uuid: Option<i32>,
    #[serde(default, rename = "accountID", deserialize_with = "take_first")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
    page: Option<i32>,
    total: Option<i32>,
    secret: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct PostCommentRequest {
    #[serde(rename = "gameVersion")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i32>,
    #[serde(rename = "udid")]
    id: Option<String>,
    uuid: Option<i32>,
    #[serde(default, rename = "accountID", deserialize_with = "take_first")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
    #[serde(rename = "userName")]
    username: Option<String>,
    comment: Option<String>,
    secret: Option<String>,
    #[serde(rename = "cType")]
    comment_type: Option<i32>,
    #[serde(rename = "chk")]
    checksum: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct DeleteCommentRequest {
    #[serde(rename = "gameVersion")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i32>,
    #[serde(rename = "udid")]
    id: Option<String>,
    uuid: Option<i32>,
    #[serde(default, rename = "accountID", deserialize_with = "take_first")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
    #[serde(rename = "commentID")]
    comment_id: Option<i32>,
    secret: Option<String>,
    #[serde(rename = "cType")]
    comment_type: Option<i32>,
    #[serde(rename = "targetAccountID")]
    target_account_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct UpdateUserSettingsRequest {
    #[serde(default, rename = "accountID", deserialize_with = "take_first")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: Option<String>,
    #[serde(rename = "mS")]
    // Allow Messages From:
    // ALL, FRIENDS, NONE
    allow_messages: Option<i32>,
    #[serde(rename = "frS")]
    // Allow Friend Requests From:
    // ALL, NONE
    allow_friend_requests: Option<i32>,
    #[serde(rename = "cS")]
    // Show Comment History To:
    // ALL, FRIENDS, ME
    show_comments_history: Option<i32>,
    #[serde(rename = "yt")]
    youtube: Option<String>,
    twitter: Option<String>,
    twitch: Option<String>,
    secret: Option<String>,
}

async fn get_friend_requests_count(db: &PgPool, account_id: i32) -> i64 {
    let query = "SELECT count(*) FROM friendreqs WHERE toAccountID = $1";
    sqlx::query_scalar(query)
        .bind(account_id)
        .fetch_one(db)
        .await
        .unwrap_or(0)
}

async fn get_messages_count(db: &PgPool, account_id: i32) -> i64 {
    let query = "SELECT count(*) FROM messages WHERE toAccountID = $1 AND isNew = 0";
    sqlx::query_scalar(query)
        .bind(account_id)
        .fetch_one(db)
        .await
        .unwrap_or(0)
}

async fn get_friends_count(db: &PgPool, account_id: i32) -> i64 {
    sqlx::query_scalar!(
        r#"
        SELECT count(*)
        FROM friendships
        WHERE (person1 = $1 AND is_new2 = '1') OR (person2 = $1 AND is_new1 = '1')
    "#,
        account_id
    )
    .fetch_one(db)
    .await
    .unwrap_or_default()
    .unwrap()
}

async fn get_friend_state(db: &PgPool, account_id: i32, target_account_id: i32) -> i32 {
    let incoming_req = sqlx::query_scalar!(
        r#"
        SELECT ID FROM friend_requests WHERE account_id = $1 AND to_account_id = $2
    "#,
        account_id,
        target_account_id
    )
    .fetch_optional(db)
    .await
    .unwrap_or(None);

    if incoming_req.is_some() {
        return 3;
    }

    let outgoing_req = sqlx::query_scalar!(
        r#"
        SELECT count(*) FROM friend_requests WHERE to_account_id = $1 AND account_id = $2
    "#,
        target_account_id,
        account_id
    )
    .fetch_one(db)
    .await
    .unwrap_or_default()
    .unwrap();

    if outgoing_req > 0 {
        return 4;
    }

    let is_friend = sqlx::query_scalar!(
        r#"
        SELECT count(*)
        FROM friendships
        WHERE (person1 = $1 AND person2 = $2) OR (person2 = $1 AND person1 = $2)
    "#,
        account_id,
        target_account_id
    )
    .fetch_one(db)
    .await
    .unwrap_or_default()
    .unwrap();

    if is_friend > 0 {
        return 1;
    }

    0
}

fn sanitize_youtube(youtube: &str) -> String {
    if youtube.starts_with('@') {
        let sanitized = youtube
            .chars()
            .enumerate()
            .filter(|(i, c)| *i == 0 || c.is_alphanumeric() || *c == '_')
            .map(|(_, c)| c)
            .collect::<String>();

        return format!("../{}", sanitized);
    }

    youtube
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

fn sanitize_social(handle: &str) -> String {
    handle
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

async fn update_user_scores(
    Extension(db): Extension<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(data): Form<UpdateRequest>,
) -> impl IntoResponse {
    let upload_date = chrono::Utc::now().timestamp();

    let old_stats = sqlx::query!("SELECT * FROM users WHERE user_id = $1", data.account_id)
        .fetch_optional(&db)
        .await
        .unwrap();

    if let Some(_old) = old_stats {
        let _ = sqlx::query!(
            r#"UPDATE users SET
                game_version = $1, username = $2, coins = $3, secret = $4, stars = $5, demons = $6, icon = $7,
                color1 = $8, color2 = $9, icon_type = $10, user_coins = $11, special = $12, acc_icon = $13, acc_ship = $14,
                acc_ball = $15, acc_bird = $16, acc_dart = $17, acc_robot = $18, acc_glow = $19, ip = $20, last_played = $21,
                acc_spider = $22, acc_explosion = $23, diamonds = $24, moons = $25, color3 = $26, acc_swing = $27,
                acc_jetpack = $28, dinfo = $29, sinfo = $30
             WHERE user_id = $31"#,
            data.game_version,
            data.username,
            data.coins,
            data.secret,
            data.stars,
            data.demons,
            data.icon,
            data.color1,
            data.color2,
            data.icon_type,
            data.user_coins,
            data.special,
            data.accessory_icon,
            data.accessory_ship,
            data.accessory_ball,
            data.accessory_bird,
            data.accessory_dart,
            data.accessory_robot,
            data.accessory_glow,
            addr.ip().to_string(),
            upload_date as i32,
            data.accessory_spider,
            data.accessory_explosion,
            data.diamonds,
            data.moons,
            data.color3,
            data.accessory_swing,
            data.accessory_jetpack,
            data.dinfo.clone(),
            data.sinfo.clone(),
            data.account_id
        )
        .execute(&db)
        .await;

        return data.account_id.unwrap().to_string();
    }

    "-1".to_string()
}

async fn get_user_info(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UserInfoRequest>,
) -> impl IntoResponse {
    let target = data.target_account_id.unwrap_or_default();
    let me = data.account_id.unwrap_or_default();

    let is_me = me == target;

    let is_blocked = sqlx::query_scalar!(r#"
        SELECT count(*) FROM blocks WHERE (person1 = $1 AND person2 = $2) OR (person2 = $1 AND person1 = $2)
    "#, me, target)
        .fetch_one(&db)
        .await
        .unwrap_or_default();

    if is_blocked.unwrap() != 0 {
        return "-1".into_response();
    }

    let user = sqlx::query!(
        r#"
        SELECT * FROM users WHERE user_id = $1
    "#,
        target
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if user.is_none() {
        return "-1".into_response();
    }

    let user = user.unwrap();

    let rank = sqlx::query_scalar!(
        r#"
        SELECT count(*) FROM users WHERE (stars + moons) > ($1::INT + $2::INT)
    "#,
        user.stars,
        user.moons
    )
    .fetch_one(&db)
    .await
    .unwrap_or_default()
    .unwrap();
    let rank = rank + 1;

    let account_info = sqlx::query!(
        r#"
        SELECT youtube_url, twitter, twitch, fr_s, ms, cs FROM accounts WHERE account_id = $1
    "#,
        target
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if account_info.is_none() {
        return "-1".into_response();
    }

    let account_info = account_info.unwrap();

    let youtube_url = sanitize_youtube(&account_info.youtube_url);
    let twitter = sanitize_social(&account_info.twitter);
    let twitch = sanitize_social(&account_info.twitch);

    let mut response = format!(
        "1:{}:2:{}:13:{}:17:{}:10:{}:11:{}:51:{}:3:{}:46:{}:52:{}:4:{}:8:{}:18:{}:19:{}:50:{}:20:{}:21:{}:22:{}:23:{}:24:{}:25:{}:26:{}:28:{}:43:{}:48:{}:53:{}:54:{}:30:{}:16:{}:31:{}:44:{}:45:{}:49:{}:55:{}:56:{}:57:{}",
        user.username,
        user.user_id,
        user.coins,
        user.user_coins,
        user.color1,
        user.color2,
        user.color3,
        user.stars,
        user.diamonds,
        user.moons,
        user.demons,
        user.creator_points.round(),
        account_info.ms,
        account_info.fr_s,
        account_info.cs,
        youtube_url,
        user.acc_icon,
        user.acc_ship,
        user.acc_ball,
        user.acc_bird,
        user.acc_dart,
        user.acc_robot.unwrap(),
        user.acc_glow,
        user.acc_spider,
        user.acc_explosion,
        user.acc_swing,
        user.acc_jetpack,
        rank,
        user.ext_id,
        if is_me { 1 } else { 0 },
        twitter,
        twitch,
        0,
        user.dinfo.unwrap_or_default(),
        user.sinfo.unwrap_or_default(),
        user.pinfo.unwrap_or_default()
    );

    if is_me {
        let friend_requests = get_friend_requests_count(&db, me).await;
        let messages = get_messages_count(&db, me).await;
        let friends = get_friends_count(&db, me).await;
        response.push_str(&format!(
            ":38:{}:39:{}:40:{}",
            messages, friend_requests, friends
        ));
    } else {
        let friend_state = get_friend_state(&db, me, target).await;
        response.push_str(&format!(":31:{}", friend_state));
    }

    response.push_str("29:1");
    response.into_response()
}

async fn get_user_comments(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UserCommentsRequest>,
) -> impl IntoResponse {
    let account_id = data.account_id.unwrap_or_default();
    let page = data.page.unwrap_or_default();
    let offset = (page * 10) as i64;

    let user_id = match sqlx::query_scalar!(
        r#"SELECT user_id FROM users WHERE ext_id = $1"#,
        account_id.to_string()
    )
    .fetch_optional(&db)
    .await
    .unwrap()
    {
        Some(id) => id,
        None => return "#0:0:0".into_response(),
    };

    let comments = sqlx::query!(r#"
        SELECT comment, user_id, likes, is_spam, comment_id, timestamp FROM acc_comments WHERE user_id = $1 ORDER BY timestamp DESC LIMIT 10 OFFSET $2
    "#, user_id, offset)
        .fetch_all(&db)
        .await
        .unwrap();

    if comments.is_empty() {
        return "#0:0:0".into_response();
    }

    let mut comment_string = String::new();

    let comment_count: Option<i64> = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM acc_comments WHERE user_id = $1"#,
        user_id
    )
    .fetch_one(&db)
    .await
    .unwrap_or_default();

    for comment in comments {
        let comment_date = utilities::make_time(comment.timestamp as i64);

        let _ = write!(
            comment_string,
            "2~{}~3~{}~4~{}~5~0~7~{}~9~{}~6~{}|",
            comment.comment,
            comment.user_id,
            comment.likes,
            comment.is_spam,
            comment_date,
            comment.comment_id
        );
    }

    if comment_string.ends_with('|') {
        comment_string.pop();
    }

    format!(
        "{comment_string}\n#{}:{}:10",
        comment_count.unwrap_or_default(),
        offset
    )
    .into_response()
}

async fn add_user_comment(
    Extension(db): Extension<PgPool>,
    Form(data): Form<PostCommentRequest>,
) -> impl IntoResponse {
    let Some(account_id) = data.account_id else {
        return "-1".into_response();
    };
    let Some(comment) = data.comment.as_ref() else {
        return "-1".into_response();
    };

    let decoded_comment = crypto::decode_base64(comment);

    if decoded_comment.len() > 140 {
        return format!(
            "temp_0_You cannot post account comments above 140 characters! (your's {})",
            decoded_comment.len()
        )
        .into_response();
    }

    let username = data.username.clone().unwrap_or_default();
    let user_id = match sqlx::query_scalar!(
        r#"SELECT user_id FROM users WHERE ext_id = $1"#,
        account_id.to_string()
    )
    .fetch_optional(&db)
    .await
    .unwrap()
    {
        Some(id) => id,
        None => return "#0:0:0".into_response(),
    };

    let timestamp = Utc::now().timestamp();
    sqlx::query!(
        r#"
        INSERT INTO acc_comments (username, comment, user_id, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        username,
        comment,
        user_id,
        timestamp as i64
    )
    .execute(&db)
    .await
    .unwrap();

    "1".into_response()
}

async fn delete_user_comment(
    Extension(db): Extension<PgPool>,
    Form(data): Form<DeleteCommentRequest>,
) -> impl IntoResponse {
    let Some(account_id) = data.account_id else {
        return "-1".into_response();
    };
    let Some(comment_id) = data.comment_id else {
        return "-1".into_response();
    };

    let user_id = match sqlx::query_scalar!(
        r#"
        SELECT user_id FROM users WHERE ext_id = $1
    "#,
        account_id.to_string()
    )
    .fetch_optional(&db)
    .await
    .unwrap()
    {
        Some(id) => id,
        None => return "-1".into_response(),
    };

    let comment = sqlx::query!(
        r#"SELECT * FROM acc_comments WHERE comment_id = $1"#,
        comment_id
    )
    .fetch_one(&db)
    .await;

    if comment.is_err() {
        return "-1".into_response();
    }

    sqlx::query!(
        "DELETE FROM acc_comments WHERE comment_id = $1 AND user_id = $2",
        comment_id,
        user_id
    )
    .execute(&db)
    .await
    .unwrap();

    "1".into_response()
}

async fn update_user_settings(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UpdateUserSettingsRequest>,
) -> impl IntoResponse {
    let account_id = data.account_id.unwrap();
    let youtube = data.youtube.unwrap_or_default();
    let twitter = data.twitter.unwrap_or_default();
    let twitch = data.twitch.unwrap_or_default();
    let allow_messages = data.allow_messages.unwrap_or_default();
    let allow_friend_requests = data.allow_friend_requests.unwrap_or_default();
    let show_comments_history = data.show_comments_history.unwrap_or_default();

    sqlx::query!(
        r#"
        UPDATE accounts 
        SET youtube_url = $1, twitter = $2, twitch = $3,
            ms = $4, fr_s = $5, cs = $6
        WHERE account_id = $7
    "#,
        youtube,
        twitter,
        twitch,
        allow_messages,
        allow_friend_requests,
        show_comments_history,
        account_id
    )
    .execute(&db)
    .await
    .unwrap();

    "1".into_response()
}

pub fn init() -> Router {
    Router::new()
        .route(
            "/database/getGJAccountComments20.php",
            post(get_user_comments),
        )
        .route("/database/uploadGJAccComment20.php", post(add_user_comment))
        .route("/database/getGJUserInfo20.php", post(get_user_info))
        .route(
            "/database/updateGJAccSettings20.php",
            post(update_user_settings),
        )
        .route(
            "/database/deleteGJAccComment20.php",
            post(delete_user_comment),
        )
        .route(
            "/database/updateGJUserScore22.php",
            post(update_user_scores),
        )
}
