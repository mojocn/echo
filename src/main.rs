use axum::{routing::get, AddExtensionLayer, Router};
use std::{env, net::SocketAddr};

use sqlx;
use sqlx::mysql::MySqlPool;
mod dao;
mod handler;
mod model;
mod schema;

#[tokio::main]
async fn main() {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = MySqlPool::connect(&url).await.expect("数据库连接失败");

    let app = Router::new()
        .route("/links", get(handler::link_list).post(handler::link_create))
        .route("/", get(handler::home_page))
        .layer(AddExtensionLayer::new(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
