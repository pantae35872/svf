use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

pub async fn get_google_info(
    client: &reqwest::Client,
    access_token: String,
) -> Option<GoogleUserInfo> {
    let response = client
        .get(format!(
            "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
            access_token
        ))
        .send()
        .await
        .ok()?;
    let json = response.bytes().await.ok()?;
    return Json::<GoogleUserInfo>::from_bytes(&json).ok().map(|e| e.0);
}
