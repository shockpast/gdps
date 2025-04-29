use sqlx::PgPool;

use crate::types::database::{Account, Level, Role, User};

pub async fn get_account_by_username(db: &PgPool, username: &str) -> Option<Account> {
    sqlx::query_as!(
        Account,
        "SELECT * FROM accounts WHERE username = $1",
        username
    )
    .fetch_optional(db)
    .await
    .unwrap()
}

pub async fn get_account_by_id(db: &PgPool, id: i32) -> Option<Account> {
    sqlx::query_as!(Account, "SELECT * FROM accounts WHERE account_id = $1", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn get_user_by_id(db: &PgPool, id: i32) -> Option<User> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE ext_id = $1 OR user_id = $1::INT",
        id.to_string()
    )
    .fetch_optional(db)
    .await
    .unwrap()
}

// pub async fn get_user_by_username(db: &PgPool, username: &str) -> Option<User> {
//     sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
//         .fetch_optional(db)
//         .await
//         .unwrap()
// }

pub async fn search_user_by_username(db: &PgPool, username: &str, limit: i64) -> Vec<User> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username ILIKE $1 LIMIT $2",
        format!("%{}%", username),
        limit
    )
    .fetch_all(db)
    .await
    .unwrap()
}

pub async fn get_level_by_id(db: &PgPool, id: i32) -> Option<Level> {
    sqlx::query_as!(
        Level,
        "SELECT * FROM levels WHERE level_id = $1 AND is_deleted = 0",
        id
    )
    .fetch_optional(db)
    .await
    .unwrap()
}

pub async fn get_user_role(db: &PgPool, id: i32) -> Option<Role> {
    let account = get_account_by_id(db, id).await.unwrap();

    let role_id = sqlx::query_scalar!(
        "SELECT role_id FROM role_assign WHERE account_id = $1",
        account.account_id as i64
    )
    .fetch_optional(db)
    .await
    .unwrap()
    .unwrap_or(0);

    sqlx::query_as!(Role, "SELECT * FROM roles WHERE role_id = $1", role_id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn ban_user(db: &PgPool, id: i32) {
    sqlx::query!("UPDATE users SET is_banned = 1 WHERE user_id = $1", id)
        .execute(db)
        .await
        .unwrap();
}

pub async fn get_friends(db: &PgPool, id: i32) -> Vec<i32> {
    let mut ids = Vec::new();
    let friends = sqlx::query!(
        "SELECT person1, person2 FROM friendships WHERE person1 = $1 OR person2 = $1",
        id
    )
    .fetch_all(db)
    .await
    .unwrap();

    for friend in friends {
        if friend.person1 == id {
            ids.push(friend.person2)
        } else {
            ids.push(friend.person1)
        }
    }

    ids
}
