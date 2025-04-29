use std::fmt::Write;
use std::net::SocketAddr;

use axum::{
    Extension, Router,
    extract::ConnectInfo,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::Form;
use chrono::Utc;
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;

use crate::{
    types::{
        database::{FriendRequest, Message},
        response::CommonResponse,
    },
    utilities::{self, crypto, database::get_user_by_id},
};

use super::COMMON_SECRET;

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
    account_id: i32,
    #[serde(rename = "userName")]
    username: String,
    secret: String,
    stars: i32,
    demons: i32,
    icon: i32,
    color1: i32,
    color2: i32,
    coins: i32,
    #[serde(rename = "iconType")]
    icon_type: i32,
    #[serde(rename = "userCoins")]
    user_coins: i32,
    special: i32,
    #[serde(rename = "accIcon")]
    accessory_icon: i32,
    #[serde(rename = "accShip")]
    accessory_ship: i32,
    #[serde(rename = "accBall")]
    accessory_ball: i32,
    #[serde(rename = "accBird")]
    accessory_bird: i32,
    #[serde(rename = "accDart")]
    accessory_dart: i32,
    #[serde(rename = "accRobot")]
    accessory_robot: i32,
    #[serde(rename = "accGlow")]
    accessory_glow: i32,
    #[serde(rename = "accSpider")]
    accessory_spider: i32,
    #[serde(rename = "accExplosion")]
    accessory_explosion: i32,
    #[serde(rename = "accSwing")]
    accessory_swing: i32,
    #[serde(rename = "accJetpack")]
    accessory_jetpack: i32,
    diamonds: i32,
    moons: i32,
    color3: i32,
    dinfo: Option<String>,
    dinfow: Option<i32>,
    dinfog: Option<i32>,
    sinfo: Option<String>,
    sinfod: Option<i32>,
    sinfog: Option<i32>,
    #[serde(default, rename = "gameVersion", deserialize_with = "take_first")]
    game_version: Option<i32>,
    #[serde(rename = "binaryVersion")]
    binary_version: i32,
    #[serde(rename = "udid")]
    id: String,
    #[serde(rename = "gjp2")]
    hash: String,
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
    game_version: i32,
    #[serde(rename = "binaryVersion")]
    binary_version: i32,
    #[serde(rename = "udid")]
    id: String,
    uuid: i32,
    #[serde(default, rename = "accountID", deserialize_with = "take_first")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: String,
    #[serde(rename = "commentID")]
    comment_id: i32,
    secret: String,
    #[serde(rename = "cType")]
    comment_type: i32,
    #[serde(rename = "targetAccountID")]
    target_account_id: i32,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct UpdateUserSettingsRequest {
    #[serde(default, rename = "accountID", deserialize_with = "take_first")]
    account_id: Option<i32>,
    #[serde(rename = "gjp2")]
    hash: String,
    // Allow Messages From:
    // ALL(0), FRIENDS(1), NONE(2)
    #[serde(rename = "mS")]
    allow_messages: i32,
    // Allow Friend Requests From:
    // ALL(0), NONE(1)
    #[serde(rename = "frS")]
    allow_friend_requests: i32,
    // Show Comment History To:
    // ALL(0), FRIENDS(1), ME(2)
    #[serde(rename = "cS")]
    show_comments_history: i32,
    #[serde(rename = "yt")]
    youtube: String,
    twitter: String,
    twitch: String,
    secret: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct GetUsersRequest {
    #[serde(rename = "gameVersion")]
    game_version: i32,
    #[serde(rename = "binaryVersion")]
    binary_version: i32,
    #[serde(rename = "str")]
    query: String,
    secret: String,
    page: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct UploadMessageRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(rename = "udid")]
    pub id: String,
    pub uuid: i32,
    #[serde(rename = "accountID")]
    pub account_id: i32,
    #[serde(rename = "gjp2")]
    pub hash: String,
    #[serde(rename = "toAccountID")]
    pub to_account_id: i32,
    pub subject: String,
    pub body: String,
    pub secret: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct GetFriendsRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(rename = "udid")]
    pub id: String,
    pub uuid: i32,
    #[serde(rename = "accountID")]
    pub account_id: i32,
    #[serde(rename = "gjp2")]
    pub hash: String,
    pub page: i32,
    pub total: i32,
    pub secret: String,
    pub get_sent: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct GetMessagesRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(rename = "udid")]
    pub id: String,
    pub uuid: i32,
    #[serde(rename = "accountID")]
    pub account_id: i32,
    #[serde(rename = "gjp2")]
    pub hash: String,
    pub page: i32,
    pub total: i32,
    pub secret: String,
    #[serde(rename = "getSent")]
    pub get_sent: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct DownloadMessageRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(rename = "udid")]
    pub id: String,
    pub uuid: i32,
    #[serde(rename = "accountID")]
    pub account_id: i32,
    #[serde(rename = "gjp2")]
    pub hash: String,
    #[serde(rename = "messageID")]
    pub message_id: i32,
    pub secret: String,
    #[serde(rename = "isSender")]
    pub is_sender: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct DeleteMessageRequest {
    #[serde(rename = "gameVersion")]
    pub game_version: i32,
    #[serde(rename = "binaryVersion")]
    pub binary_version: i32,
    #[serde(rename = "udid")]
    pub id: String,
    pub uuid: i32,
    #[serde(rename = "accountID")]
    pub account_id: i32,
    #[serde(rename = "gjp2")]
    pub hash: String,
    #[serde(rename = "messageID")]
    pub message_id: Option<i32>,
    pub messages: Option<String>,
    pub secret: String,
    #[serde(rename = "isSender")]
    pub is_sender: Option<i32>,
}

async fn get_friend_requests_count(db: &PgPool, account_id: i32) -> Option<i64> {
    sqlx::query_scalar!(
        "SELECT count(*) FROM friend_requests WHERE to_account_id = $1",
        account_id
    )
    .fetch_one(db)
    .await
    .unwrap_or_default()
}

async fn get_messages_count(db: &PgPool, account_id: i32) -> Option<i64> {
    sqlx::query_scalar!(
        "SELECT count(*) FROM messages WHERE to_account_id = $1 AND is_new = 0",
        account_id
    )
    .fetch_one(db)
    .await
    .unwrap_or_default()
}

async fn get_friends_count(db: &PgPool, account_id: i32) -> Option<i64> {
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
) -> Response {
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

        return data.account_id.to_string().into_response();
    }

    "-1".into_response()
}

async fn get_user_info(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UserInfoRequest>,
) -> Response {
    let Some(target) = data.target_account_id else {
        return CommonResponse::InvalidRequest.into_response();
    };
    let Some(me) = data.account_id else {
        return CommonResponse::InvalidRequest.into_response();
    };

    let is_me = me == target;

    let is_blocked = sqlx::query_scalar!(
        r#"
        SELECT count(*) FROM blocks WHERE (person1 = $1 AND person2 = $2) OR (person2 = $1 AND person1 = $2)
    "#,
        me,
        target
    )
    .fetch_one(&db)
    .await
    .unwrap_or_default();

    if is_blocked.unwrap() != 0 {
        return CommonResponse::InvalidRequest.into_response();
    }

    let user =
        match utilities::database::get_user_by_id(&db, data.account_id.unwrap_or_default()).await {
            Some(user) => user,
            None => {
                return CommonResponse::InvalidRequest.into_response();
            }
        };

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
        return CommonResponse::InvalidRequest.into_response();
    }

    let account_info = account_info.unwrap();

    let account_role_assign = sqlx::query!(
        "SELECT role_id FROM role_assign WHERE account_id = $1",
        data.account_id.unwrap_or_default() as i64
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if account_role_assign.is_none() {
        return CommonResponse::InvalidRequest.into_response();
    }

    let account_role = sqlx::query!(
        "SELECT action_request_mod, mod_badge_level FROM roles WHERE role_id = $1",
        account_role_assign.unwrap().role_id
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if account_role.is_none() {
        return "-1".into_response();
    }

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
        account_role.unwrap().mod_badge_level,
        user.dinfo.unwrap_or_default(),
        user.sinfo.unwrap_or_default(),
        user.pinfo.unwrap_or_default()
    );

    if is_me {
        let friend_requests = get_friend_requests_count(&db, me).await.unwrap_or_default();
        let messages = get_messages_count(&db, me).await.unwrap_or_default();
        let friends = get_friends_count(&db, me).await.unwrap_or_default();
        response.push_str(&format!(
            ":38:{}:39:{}:40:{}:",
            messages, friend_requests, friends
        ));
    } else {
        let friend_state = get_friend_state(&db, me, target).await;
        response.push_str(&format!(":31:{}:", friend_state));
    }

    response.push_str("29:1");
    response.into_response()
}

async fn get_user_comments(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UserCommentsRequest>,
) -> Response {
    let Some(account_id) = data.account_id else {
        return CommonResponse::InvalidRequest.into_response();
    };
    let Some(page) = data.page else {
        return CommonResponse::InvalidRequest.into_response();
    };

    let offset = (page * 10) as i64;

    let user = match utilities::database::get_user_by_id(&db, account_id).await {
        Some(user) => user,
        None => {
            return "#0:0:0".into_response();
        }
    };

    let comments = sqlx::query!(r#"
        SELECT comment, user_id, likes, is_spam, comment_id, timestamp FROM acc_comments WHERE user_id = $1 ORDER BY timestamp DESC LIMIT 10 OFFSET $2
    "#, user.user_id, offset)
        .fetch_all(&db)
        .await
        .unwrap();

    if comments.is_empty() {
        return "#0:0:0".into_response();
    }

    let mut comment_string = String::new();

    let comment_count: Option<i64> = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM acc_comments WHERE user_id = $1"#,
        user.user_id
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
) -> Response {
    let Some(account_id) = data.account_id else {
        return CommonResponse::InvalidRequest.into_response();
    };
    let Some(comment) = data.comment.as_ref() else {
        return CommonResponse::InvalidRequest.into_response();
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
    let user = match utilities::database::get_user_by_id(&db, account_id).await {
        Some(user) => user,
        None => {
            return "#0:0:0".into_response();
        }
    };

    let timestamp = Utc::now().timestamp();
    sqlx::query!(
        r#"
        INSERT INTO acc_comments (username, comment, user_id, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        username,
        comment,
        user.user_id,
        timestamp as i64
    )
    .execute(&db)
    .await
    .unwrap();

    CommonResponse::Success.into_response()
}

async fn delete_user_comment(
    Extension(db): Extension<PgPool>,
    Form(data): Form<DeleteCommentRequest>,
) -> Response {
    let Some(account_id) = data.account_id else {
        return CommonResponse::InvalidRequest.into_response();
    };

    let user = match utilities::database::get_user_by_id(&db, account_id).await {
        Some(user) => user,
        None => {
            return "#0:0:0".into_response();
        }
    };

    let comment = sqlx::query!(
        r#"SELECT * FROM acc_comments WHERE comment_id = $1"#,
        data.comment_id
    )
    .fetch_one(&db)
    .await;

    if comment.is_err() {
        return CommonResponse::InvalidRequest.into_response();
    }

    sqlx::query!(
        "DELETE FROM acc_comments WHERE comment_id = $1 AND user_id = $2",
        data.comment_id,
        user.user_id
    )
    .execute(&db)
    .await
    .unwrap();

    CommonResponse::Success.into_response()
}

async fn update_user_settings(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UpdateUserSettingsRequest>,
) -> Response {
    let account_id = data.account_id.unwrap();
    let youtube = data.youtube;
    let twitter = data.twitter;
    let twitch = data.twitch;
    let allow_messages = data.allow_messages;
    let allow_friend_requests = data.allow_friend_requests;
    let show_comments_history = data.show_comments_history;

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

    CommonResponse::Success.into_response()
}

async fn get_users(
    Extension(db): Extension<PgPool>,
    Form(data): Form<GetUsersRequest>,
) -> Response {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }

    let users = utilities::database::search_user_by_username(&db, &data.query, 10).await;
    let mut response = String::new();

    if users.is_empty() {
        return CommonResponse::InvalidRequest.into_response();
    }

    for user in &users {
        response.push_str(&format!(
            "1:{}:2:{}:13:{}:17:{}:9:{}:10:{}:11:{}:51:{}:14:{}:15:{}:16:{}:3:{}:8:{}:4:{}:46:{}:52:{}|",
            user.username,
            user.user_id,
            user.coins,
            user.user_coins,
            user.icon,
            user.color1,
            user.color2,
            user.color3,
            user.icon_type,
            user.special,
            user.ext_id,
            user.stars,
            user.creator_points.floor() as i32,
            user.demons,
            user.diamonds,
            user.moons,
        ));
    }

    let response = response.trim_end_matches("|").to_string();

    format!(
        "{}\n#{}:{}:10",
        response,
        users.len(),
        data.page.unwrap_or_default() * 10
    )
    .into_response()
}

async fn upload_message(
    Extension(db): Extension<PgPool>,
    Form(data): Form<UploadMessageRequest>,
) -> Response {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }
    if data.account_id == data.to_account_id {
        return CommonResponse::InvalidRequest.into_response();
    }

    let sender_account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if sender_account.gjp2.unwrap_or_default() != data.hash {
        return CommonResponse::InvalidRequest.into_response();
    }
    if !sender_account.is_active {
        return CommonResponse::InvalidRequest.into_response();
    }

    let sender_user = match utilities::database::get_user_by_id(&db, data.account_id).await {
        Some(user) => user,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if sender_user.is_banned == 1 {
        return CommonResponse::InvalidRequest.into_response();
    }

    let is_blocked = sqlx::query!(
        "SELECT id FROM blocks WHERE person1 = $1 AND person2 = $2",
        data.to_account_id,
        data.account_id
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if sender_account.ms == 2 || is_blocked.is_some() {
        return CommonResponse::InvalidRequest.into_response();
    }

    sqlx::query!(
        r#"
        INSERT INTO messages (subject, body, acc_id, user_id, username, to_account_id, secret, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        data.subject, data.body, data.account_id, sender_user.user_id, sender_user.username, data.to_account_id, data.secret, chrono::Utc::now().timestamp() as i32
    ).execute(&db).await.unwrap();

    CommonResponse::Success.into_response()
}

async fn get_messages(
    Extension(db): Extension<PgPool>,
    Form(data): Form<GetMessagesRequest>,
) -> Response {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return CommonResponse::InvalidRequest.into_response();
    }

    let get_sent = data.get_sent.unwrap_or_default();
    let offset = data.page * 10;

    let messages = match get_sent {
        0 => {
            sqlx::query_as!(
                Message,
                "SELECT * FROM messages WHERE to_account_id = $1 LIMIT 10 OFFSET $2",
                data.account_id,
                offset as i64
            )
            .fetch_all(&db)
            .await
            .unwrap()
        }
        1 => {
            sqlx::query_as!(
                Message,
                "SELECT * FROM messages WHERE acc_id = $1 LIMIT 10 OFFSET $2",
                data.account_id,
                offset as i64
            )
            .fetch_all(&db)
            .await
            .unwrap()
        }
        _ => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if messages.is_empty() {
        return "-2".into_response();
    }

    let mut response = String::new();

    for message in &messages {
        let id = if get_sent == 0 {
            message.acc_id
        } else {
            message.to_account_id
        };

        let user = match get_user_by_id(&db, id).await {
            Some(user) => user,
            None => {
                tracing::error!(
                    "{} couldn't be found in 'users' when fetching friend requests.",
                    id
                );
                continue;
            }
        };

        response.push_str(&format!(
            "6:{}:3:{}:2:{}:1:{}:4:{}:8:{}:9:{}:7:{}|",
            user.username,
            user.user_id,
            user.ext_id,
            message.message_id,
            message.subject,
            message.is_new,
            get_sent,
            utilities::make_time(message.timestamp as i64)
        ));
    }

    let _ = response.trim_end_matches("|");
    format!("{}#{}:{}:10", response, messages.len(), offset).into_response()
}

async fn download_message(
    Extension(db): Extension<PgPool>,
    Form(data): Form<DownloadMessageRequest>,
) -> Response {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return CommonResponse::InvalidRequest.into_response();
    }

    let is_sender = data.is_sender.unwrap_or_default();

    let message = sqlx::query!(
        "SELECT * FROM messages WHERE message_id = $1 AND (acc_id = $2 OR to_account_id = $2) LIMIT 1",
        data.message_id, data.account_id
    ).fetch_one(&db).await.unwrap();

    if is_sender != 1 {
        sqlx::query!(
            "UPDATE messages SET is_new = 1 WHERE message_id = $1 AND to_account_id = $2",
            data.message_id,
            data.account_id
        )
        .execute(&db)
        .await
        .unwrap();
    }

    format!(
        "6:{}:3:{}:2:{}:1:{}:4:{}:8:{}:9:{}:5:{}:7:{}",
        message.username,
        message.acc_id,
        message.acc_id,
        message.message_id,
        message.subject,
        message.is_new,
        is_sender,
        message.body,
        utilities::make_time(message.timestamp as i64)
    )
    .into_response()
}

// TODO: rewrite this if-else condition ladder
async fn delete_message(
    Extension(db): Extension<PgPool>,
    Form(data): Form<DeleteMessageRequest>,
) -> Response {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return CommonResponse::InvalidRequest.into_response();
    }

    let messages = data.messages.unwrap_or_default();
    let message_id = data.message_id.unwrap_or_default();

    if !messages.is_empty() {
        let message_ids = messages
            .split_terminator(",")
            .collect::<Vec<_>>()
            .iter()
            .map(|m| m.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        sqlx::query!(
            "DELETE FROM messages WHERE message_id = ANY($1) AND acc_id = $2",
            &message_ids,
            data.account_id
        )
        .execute(&db)
        .await
        .unwrap();

        sqlx::query!(
            "DELETE FROM messages WHERE message_id = ANY($1) AND to_account_id = $2",
            &message_ids,
            data.account_id
        )
        .execute(&db)
        .await
        .unwrap();
    } else {
        sqlx::query!(
            "DELETE FROM messages WHERE message_id = $1 AND acc_id = $2",
            message_id,
            data.account_id
        )
        .execute(&db)
        .await
        .unwrap();

        sqlx::query!(
            "DELETE FROM messages WHERE message_id = $1 AND to_account_id = $2",
            message_id,
            data.account_id
        )
        .execute(&db)
        .await
        .unwrap();
    }

    CommonResponse::Success.into_response()
}

async fn get_friends(
    Extension(db): Extension<PgPool>,
    Form(data): Form<GetFriendsRequest>,
) -> Response {
    if data.secret != COMMON_SECRET {
        return CommonResponse::InvalidRequest.into_response();
    }

    let get_sent = data.get_sent.unwrap_or_default();
    let offset = data.page * 10;

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return CommonResponse::InvalidRequest.into_response();
    }

    let friend_requests = match data.get_sent.unwrap_or_default() {
        0 => {
            sqlx::query_as!(
                FriendRequest,
                "SELECT * FROM friend_requests WHERE to_account_id = $1 LIMIT 10 OFFSET $2",
                data.account_id,
                offset as i64
            )
            .fetch_all(&db)
            .await
            .unwrap()
        }
        1 => {
            sqlx::query_as!(
                FriendRequest,
                "SELECT * FROM friend_requests WHERE account_id = $1 LIMIT 10 OFFSET $2",
                data.account_id,
                offset as i64
            )
            .fetch_all(&db)
            .await
            .unwrap()
        }
        _ => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    let mut response = String::new();

    for friend_request in &friend_requests {
        let sender_id = if get_sent == 0 {
            friend_request.account_id
        } else {
            friend_request.to_account_id
        };

        let sender_user = match get_user_by_id(&db, sender_id).await {
            Some(user) => user,
            None => {
                tracing::error!(
                    "{} couldn't be found in 'users' when fetching friend requests.",
                    sender_id
                );
                continue;
            }
        };

        response.push_str(&format!(
            "1:{}:2:{}:9:{}:10:{}:11:{}:14:{}:15:{}:16:{}:32:{}:35:{}:41:{}:37:{}|",
            sender_user.username,
            sender_user.user_id,
            sender_user.icon,
            sender_user.color1,
            sender_user.color2,
            sender_user.icon_type,
            sender_user.special,
            sender_user.ext_id,
            friend_request.id,
            friend_request.comment,
            friend_request.is_new,
            utilities::make_time(friend_request.upload_date as i64)
        ))
    }

    let _ = response.trim_end_matches("|");
    format!("{}#{}:{}:10", response, friend_requests.len(), offset).into_response()
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
        .route("/database/getGJUsers20.php", post(get_users))
        .route("/database/getGJFriendRequests20.php", post(get_friends))
        .route("/database/uploadGJMessage20.php", post(upload_message))
        .route("/database/getGJMessages20.php", post(get_messages))
        .route("/database/downloadGJMessage20.php", post(download_message))
        .route("/database/deleteGJMessages20.php", post(delete_message))
}
