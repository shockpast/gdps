use std::net::SocketAddr;

use axum::{Extension, Form, Router, extract::ConnectInfo, response::IntoResponse, routing::post};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use crate::utilities::{crypto, database::get_user_id};

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

// Possible Responses:
//  -99      = Emails do not match
//  -9       = Too short. Minimum 3 characters (username)
//  -8       = Too short. Minimum 6 characters (password)
//  -7       = Passwords do not match
//  -6       = Emails is invalid
//  -5       = Password is invalid
//  -4       = Username is invalid
//  -3       = Email is already in use
//  -2       = Username is already in use
//  default  = Something went wrong.
async fn register_account(
    Extension(db): Extension<PgPool>,
    Form(data): Form<RegisterRequest>,
) -> impl IntoResponse {
    if data.username.len() < 3 {
        return "-9".into_response();
    }
    if data.password.len() < 6 {
        return "-8".into_response();
    }

    if data.username.len() > 20 {
        return "-4".into_response();
    }

    let account = sqlx::query!(
        "SELECT count(*) FROM accounts WHERE username = $1",
        data.username
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if account.unwrap().count.unwrap_or_default() != 0 {
        return "-2".into_response();
    }

    let password = crypto::hash_password(&data.password);
    let gjp2 = crypto::sha1_salt(&data.password, "mI29fmAnxgTs");

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
        Ok(_) => "1".into_response(),
        Err(_) => "0".into_response(),
    }
}

// Possible Responses:
//  -13      = Already linked to different Steam account
//  -12      = Account has been disabled
//  -10      = Already linked to different account
//  -9       = Too short. Minimum 3 characters (username)
//  -8       = Too short. Minimum 6 characters (password)
//  default  = Login failed
async fn login_account(
    Extension(db): Extension<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(data): Form<LoginRequest>,
) -> impl IntoResponse {
    let account = match sqlx::query!(
        "SELECT account_id, password, gjp2, is_active FROM accounts WHERE username = $1",
        data.username
    )
    .fetch_optional(&db)
    .await
    {
        Ok(Some(acc)) => acc,
        _ => return "-1".into_response(),
    };

    if data.hash != account.gjp2.unwrap_or_default() {
        return "0".into_response();
    }

    let user_id = match get_user_id(
        &db,
        &account.account_id.to_string(),
        &data.username,
        &addr.ip(),
    )
    .await
    {
        Ok(user_id) => user_id,
        _ => return "-1".into_response(),
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

pub fn init() -> Router {
    Router::new()
        .route("/database/accounts/loginGJAccount.php", post(login_account))
        .route(
            "/database/accounts/registerGJAccount.php",
            post(register_account),
        )
}
