use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    service::authentication_service::AuthenticationServiceRequest, web_server::BackendResponse,
    ServiceHandles,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct UsernameSignup {
    username: String,
    password_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleSignup {
    username: String,
    google_access_token: String,
}

pub async fn google(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<GoogleSignup>,
) -> impl IntoResponse {
    match services
        .auth_service
        .request(AuthenticationServiceRequest::GoogleSignup {
            username: data.username,
            google_access_token: data.google_access_token,
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return (err.clone().into(), err.into()),
    };

    (StatusCode::OK, Json(BackendResponse::Ok))
}

pub async fn username(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<UsernameSignup>,
) -> impl IntoResponse {
    match services
        .auth_service
        .request(AuthenticationServiceRequest::DefaultSignup {
            username: data.username,
            password_hash: data.password_hash,
        })
        .await
    {
        Ok(res) => res,
        Err(err) => return (err.clone().into(), err.into()),
    };
    (StatusCode::OK, Json(BackendResponse::Ok))
}
