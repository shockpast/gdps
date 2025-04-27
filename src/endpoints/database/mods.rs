use axum::{Extension, Router, response::IntoResponse, routing::post};
use axum_extra::extract::Form;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{types::response::CommonResponse, utilities};

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct RequestUserAccessRequest {
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
    secret: String,
}

async fn request_user_access(
    Extension(db): Extension<PgPool>,
    Form(data): Form<RequestUserAccessRequest>,
) -> impl IntoResponse {
    let account = match utilities::database::get_account_by_id(&db, data.account_id).await {
        Some(account) => account,
        None => {
            return CommonResponse::InvalidRequest.into_response();
        }
    };

    if account.gjp2.unwrap_or_default() != data.hash {
        return CommonResponse::InvalidRequest.into_response();
    }

    let role_assign = sqlx::query!(
        "SELECT role_id FROM role_assign WHERE account_id = $1",
        data.account_id as i64
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    if role_assign.is_none() {
        return CommonResponse::InvalidRequest.into_response();
    }

    let role = sqlx::query!(
        "SELECT action_request_mod, mod_badge_level FROM roles WHERE role_id = $1",
        role_assign.unwrap().role_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    if role.action_request_mod == 0 {
        return CommonResponse::InvalidRequest.into_response();
    }

    format!("{}", role.mod_badge_level.max(2)).into_response()
}

pub fn init() -> Router {
    Router::new().route("/database/requestUserAccess.php", post(request_user_access))
}
