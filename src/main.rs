use axum::{routing::get, AddExtensionLayer, Router};
use std::{env, net::SocketAddr};

use sqlx;
use sqlx::mysql::MySqlPool;
use echo::handler::link::{home_page, link_create, link_list};


#[tokio::main]
async fn main() {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = MySqlPool::connect(&url).await.expect("数据库连接失败");

    let app = Router::new()
        .route("/links", get(link_list).post(link_create))
        .route("/", get(home_page))
        .layer(AddExtensionLayer::new(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
