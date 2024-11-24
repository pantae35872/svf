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
    let challenge = match match services
        .auth_service
        .request(AuthenticationServiceRequest::PasswordChallenge {
            username: data.username,
        })
        .await
    {
        Ok(access_token) => access_token,
        Err(err) => return (err.clone().into(), err.into()),
    } {
        AuthenticationServiceResponse::PasswordChallenge(challenge) => challenge,
        _ => unreachable!(),
    };
    (
        StatusCode::OK,
        Json(BackendResponse::PasswordChallenge(
            challenge.iter().collect::<String>(),
        )),
    )
}

pub async fn google(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<GoogleLogin>,
) -> impl IntoResponse {
    let access_token = match match services
        .auth_service
        .request(AuthenticationServiceRequest::GoogleLogin {
            google_access_token: data.google_access_token,
        })
        .await
    {
        Ok(access_token) => access_token,
        Err(err) => return (err.clone().into(), err.into()),
    } {
        AuthenticationServiceResponse::AccessToken(token) => token,
        _ => unreachable!(),
    };
    (
        StatusCode::OK,
        Json(BackendResponse::AccessToken(
            access_token.iter().collect::<String>(),
        )),
    )
}

pub async fn username(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<UsernameLogin>,
) -> impl IntoResponse {
    let access_token = match match services
        .auth_service
        .request(AuthenticationServiceRequest::DefaultLogin {
            username: data.username,
            password_hash: data.password_challenge_hash,
        })
        .await
    {
        Ok(access_token) => access_token,
        Err(err) => return (err.clone().into(), err.into()),
    } {
        AuthenticationServiceResponse::AccessToken(token) => token,
        _ => unreachable!(),
    };
    (
        StatusCode::OK,
        Json(BackendResponse::AccessToken(
            access_token.iter().collect::<String>(),
        )),
    )
}
