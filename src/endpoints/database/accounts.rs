use std::{io::Read, net::SocketAddr};

use axum::{
    Extension, Form, Router,
    extract::ConnectInfo,
    response::{IntoResponse, Response},
    routing::post,
};
use flate2::read::GzDecoder;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use tokio::fs::{read_to_string, write};
use tracing::error;

use crate::{
    types::response::{BackupResponse, CommonResponse, LoginResponse, RegisterResponse},
    utilities,
};

#[derive(Deserialize)]
#[allow(unused)]
struct LoginRequest {
    #[serde(rename = "udid")]
    id: String,
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "gjp2")]
    hash: String,
    #[serde(rename = "sID")]
    steam_id: String,
    secret: String,
}

#[derive(Deserialize)]
#[allow(unused)]
struct RegisterRequest {
    #[serde(rename = "userName")]
    username: String,
    password: String,
    email: String,
    secret: String,
}

#[derive(Deserialize)]
#[allow(unused)]
struct GetAccountURLRequest {
    #[serde(rename = "accountID")]
    account_id: i32,
    #[serde(rename = "type")]
    button_type: ButtonType,
    secret: String,
}

#[derive(Deserialize)]
enum ButtonType {
    #[serde(rename = "1")]
    Save,
    #[serde(rename = "2")]
    Load,
}

#[derive(Deserialize)]
#[allow(unused)]
struct BackupRequest {
    #[serde(rename = "gameVersion")]
    game_version: i32,
    #[serde(rename = "binaryVersion")]
    binary_version: i32,
    #[serde(rename = "udid")]
    id: String,
    #[serde(rename = "uuid")]
    user_id: String,
    #[serde(rename = "accountID")]
    account_id: i32,
    #[serde(rename = "gjp2")]
    hash: String,
    #[serde(rename = "saveData")]
    save_data: String,
    secret: String,
}

#[derive(Deserialize)]
#[allow(unused)]
struct SyncRequest {
    #[serde(rename = "gameVersion")]
    game_version: i32,
    #[serde(rename = "binaryVersion")]
    binary_version: i32,
    #[serde(rename = "udid")]
    id: String,
    #[serde(rename = "uuid")]
    user_id: String,
    #[serde(rename = "accountID")]
    account_id: i32,
    #[serde(rename = "gjp2")]
    hash: String,
    secret: String,
}

async fn register_account(
    Extension(db): Extension<PgPool>,
    Form(data): Form<RegisterRequest>,
) -> Response {
    if data.username.len() < 3 {
        return RegisterResponse::UsernameIsTooShort.into_response();
    }
    if data.password.len() < 6 {
        return RegisterResponse::PasswordIsTooShort.into_response();
    }

    if data.username.len() > 20 {
        return RegisterResponse::InvalidUsername.into_response();
    }

    let account = utilities::database::get_account_by_username(&db, &data.username).await;
    if account.is_some() {
        return RegisterResponse::AccountExists.into_response();
    }

    let password = utilities::crypto::hash_password(&data.password).await;
    let gjp2 = utilities::crypto::sha1_salt(&data.password, "mI29fmAnxgTs");

    let result = sqlx::query!(
        r#"
        INSERT INTO accounts (username, password, email, is_active, gjp2)
        VALUES ($1, $2, $3, $4, $5)
    "#,
        data.username,
        password,
        data.email,
        true,
        gjp2
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => RegisterResponse::Success.into_response(),
        Err(_) => RegisterResponse::InvalidRequest.into_response(),
    }
}

async fn login_account(
    Extension(db): Extension<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(data): Form<LoginRequest>,
) -> Response {
    let account = match sqlx::query!(
        "SELECT account_id, password, gjp2, is_active FROM accounts WHERE username = $1",
        data.username
    )
    .fetch_optional(&db)
    .await
    {
        Ok(Some(acc)) => acc,
        _ => return LoginResponse::InvalidRequest.into_response(),
    };

    if data.hash != account.gjp2.unwrap_or_default() {
        return LoginResponse::WrongCredentials.into_response();
    }

    let user_id = match utilities::database::get_user_id(
        &db,
        &account.account_id.to_string(),
        &data.username,
        &addr.ip(),
    )
    .await
    {
        Ok(user_id) => user_id,
        _ => return LoginResponse::InvalidRequest.into_response(),
    };

    if data.id.parse::<i64>().is_err() {
        if let Some(old_user_id) =
            sqlx::query_scalar!("SELECT user_id FROM users WHERE ext_id = $1", data.id)
                .fetch_optional(&db)
                .await
                .unwrap_or(None)
        {
            let _ = sqlx::query!(
                "UPDATE levels SET user_id = $1, ext_id = $2 WHERE user_id = $3",
                user_id,
                account.account_id.to_string(),
                old_user_id
            )
            .execute(&db)
            .await;
        }
    }

    format!("{},{}", account.account_id, user_id).into_response()
}

async fn get_account_url() -> Response {
    "https://rustyserver.local".into_response()
}

async fn backup_account(
    Extension(db): Extension<PgPool>,
    Form(data): Form<BackupRequest>,
) -> Response {
    if data.secret != "Wmfv3899gc9" {
        return CommonResponse::InvalidRequest.into_response();
    }

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return BackupResponse::WrongCredentials.into_response();
    }

    let mut save_data = data.save_data.splitn(2, ";");
    let compressed_data = save_data.next().unwrap();

    let save_data = utilities::crypto::decode_base64_url_raw(compressed_data);

    let mut decoder = GzDecoder::new(save_data.as_slice());
    let mut decompressed_data = String::new();

    match decoder.read_to_string(&mut decompressed_data) {
        Ok(_) => (),
        Err(e) => {
            error!("{e:?}");
            return BackupResponse::SomethingWentWrong.into_response();
        }
    };

    let orbs = decompressed_data
        .split("</s><k>14</k><s>")
        .nth(1)
        .and_then(|part| part.split("</s>").next())
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    let levels = decompressed_data
        .split("<k>GS_value</k>")
        .nth(1)
        .and_then(|part| part.split("</s><k>4</k><s>").nth(1))
        .and_then(|part| part.split("</s>").next())
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();

    sqlx::query!(
        "UPDATE users SET orbs = $1, completed_lvls = $2 WHERE ext_id = $3",
        orbs,
        levels,
        &data.account_id.to_string()
    )
    .execute(&db)
    .await
    .unwrap();

    match write(format!("data/saves/{}", data.account_id), data.save_data).await {
        Ok(_) => CommonResponse::Success.into_response(),
        Err(_) => BackupResponse::InvalidRequest.into_response(),
    }
}

async fn sync_account(Extension(db): Extension<PgPool>, Form(data): Form<SyncRequest>) -> Response {
    if data.secret != "Wmfv3899gc9" {
        return CommonResponse::InvalidRequest.into_response();
    }

    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return BackupResponse::WrongCredentials.into_response();
    }

    let save_data = match read_to_string(format!("data/saves/{}", data.account_id)).await {
        Ok(s) => s,
        Err(e) => {
            error!("{e:?}");
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    format!("{};21;30;a;a", save_data).into_response()
}

pub fn init() -> Router {
    Router::new()
        .route("/database/accounts/loginGJAccount.php", post(login_account))
        .route(
            "/database/accounts/registerGJAccount.php",
            post(register_account),
        )
        .route(
            "/database/accounts/backupGJAccountNew.php",
            post(backup_account),
        )
        .route(
            "/database/accounts/syncGJAccountNew.php",
            post(sync_account),
        )
        .route("/database/getAccountURL.php", post(get_account_url))
}
