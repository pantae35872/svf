use axum::{
    body::Bytes,
    extract::FromRef,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::{headers::Cookie, TypedHeader};
use serde::{Deserialize, Serialize};
use sqlite::State;
use tokio::{fs::File, io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/test", get(test))
        .fallback(fallback_handler);
    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();
    let db = sqlite::open("svf.db").unwrap();
    let mut statement = db.prepare("SELECT * FROM users WHERE name = :id").unwrap();
    statement.bind((":id", "Hello")).unwrap();
    while let Ok(State::Row) = statement.next() {
        println!("{}", statement.read::<String, _>("name").unwrap());
        println!("{}", statement.read::<i64, _>("age").unwrap());
    }
    axum::serve(listener, app).await.unwrap();
}

async fn fallback_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}

async fn test(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(EEE {
            aaa: "aaa".to_string(),
        }),
    )
}

#[derive(Deserialize, Serialize)]
struct EEE {
    aaa: String,
}
