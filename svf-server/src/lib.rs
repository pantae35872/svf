use std::{env, sync::Arc};

use axum::{
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use service::{
    authentication_service::{AuthenticationService, AuthenticationServiceHandle},
    db_service::{DBService, DBServiceHandle},
    serve_service, Service,
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use wait_pool::WaitPool;

pub mod login;
pub mod service;
pub mod signup;
pub mod utils;
pub mod wait_pool;
pub mod web_server;

pub fn is_production() -> bool {
    match env::var("PROD") {
        Ok(prod) => prod == "1",
        Err(_) => false,
    }
}

pub fn build_cors() -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
    if !is_production() {
        base.allow_origin("http://localhost:5123".parse::<HeaderValue>().unwrap())
    } else {
        base.allow_origin(
            "https://strawberryvisionfarm.web.app"
                .parse::<HeaderValue>()
                .unwrap(),
        )
    }
}

pub struct ServiceHandles {
    pub db_service: DBServiceHandle,
    pub auth_service: AuthenticationServiceHandle,
}

pub fn router() -> Router<Arc<ServiceHandles>> {
    Router::new()
        .route("/login/username", post(login::username))
        .route("/login/google", post(login::google))
        .route("/login/password-challenge", post(login::password_challenge))
        .route("/signup/username", post(signup::username))
        .route("/signup/google", post(signup::google))
        .layer(ServiceBuilder::new().layer(build_cors()))
        .fallback(notfound_handler)
}

async fn notfound_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}

pub async fn init_services(wait_pool: &mut WaitPool) -> ServiceHandles {
    let db_service = DBService::new().await;
    let auth_service = AuthenticationService::new(db_service.get());
    let handles = ServiceHandles {
        db_service: db_service.get(),
        auth_service: auth_service.get(),
    };
    wait_pool.add(serve_service(db_service));
    wait_pool.add(serve_service(auth_service));
    handles
}
