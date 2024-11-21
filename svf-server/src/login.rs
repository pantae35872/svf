use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use reqwest::header::SET_COOKIE;
use serde::{Deserialize, Serialize};

use crate::{
    service::authentication_service::{
        AuthenticationServiceRequest, AuthenticationServiceResponse,
    },
    web_server::BackendResponse,
    ServiceHandles,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct UsernameLogin {
    username: String,
    password_challenge_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleLogin {
    google_access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PasswordChallenge {
    username: String,
}

pub async fn password_challenge(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<PasswordChallenge>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(BackendResponse::Ok))
}

pub async fn google(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<GoogleLogin>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let access_token = match match services
        .auth_service
        .request(AuthenticationServiceRequest::GoogleLogin {
            google_access_token: data.google_access_token,
        })
        .await
    {
        Ok(access_token) => access_token,
        Err(err) => return (err.clone().into(), headers, err.into()),
    } {
        AuthenticationServiceResponse::AccessToken(token) => token,
        _ => unreachable!(),
    };
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "accessToken={}; SameSite=None; Secure; Partitioned",
            access_token.iter().collect::<String>()
        ))
        .unwrap(),
    );
    (StatusCode::OK, headers, Json(BackendResponse::Ok))
}

pub async fn username(Json(data): Json<UsernameLogin>) -> impl IntoResponse {
    //let mut headers = HeaderMap::new();
    //headers.insert(
    //    SET_COOKIE,
    //    HeaderValue::from_str(&format!("accessToken={}", "aa")).unwrap(),
    //);
    dbg!(data);
    (StatusCode::OK, Json(BackendResponse::Ok))
}