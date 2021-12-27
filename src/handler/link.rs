use crate::model::link::{Link as LinkModel,BaseResult};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::Serialize;
use sqlx;
use sqlx::mysql::MySqlPool;

pub async fn home_page() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

pub async fn link_create(Json(payload): Json<LinkModel>) -> impl IntoResponse {
    let user = LinkModel {
        id: 3389,
        url: payload.url,
        tiny: "payload.username".to_owned(),
    };
    (StatusCode::CREATED, Json(user))
}

pub async fn link_list(Extension(db): Extension<MySqlPool>) -> impl IntoResponse {
    let recs = sqlx::query!(r#"SELECT id,tiny, url FROM kae_links ORDER BY id"#)
        .fetch_all(&db)
        .await
        .expect("查询失败");
    let mut results: Vec<LinkModel> = Vec::new();

    // NOTE: Booleans in MySQL are stored as `TINYINT(1)` / `i8` 0 = false, non-0 = true
    for rec in recs {
        let user = LinkModel {
            id: rec.id,
            tiny: rec.tiny.expect("ok").to_owned(),
            url: rec.url.expect("url is null").to_owned(),
        };
        results.push(user);
    }
    json_data(results)
}

fn json_data<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::OK, Json(BaseResult::ok(data)))
}
