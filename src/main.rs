mod endpoints;
mod utilities;

use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use endpoints::database;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:admin@localhost/gdps")
        .await
        .unwrap();

    let router = Router::new()
        .route("/", get(index))
        .merge(database::accounts::init())
        .merge(database::user::init())
        .merge(database::rewards::init())
        .merge(database::levels::init())
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
    "Hello, Rust!"
}
