mod endpoints;
mod utilities;

use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(env!(
            "DATABASE_URL",
            "DATABASE_URL is not set, but .env exists."
        ))
        .await
        .unwrap();

    let router = Router::new()
        .route("/", get(index))
        .merge(endpoints::database::accounts::init())
        .merge(endpoints::database::user::init())
        .merge(endpoints::database::rewards::init())
        .merge(endpoints::database::levels::init())
        .merge(endpoints::database::mods::init())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn index() -> &'static str {
    "🦀 https://github.com/shockpast/gdps"
}
