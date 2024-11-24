use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    service::{
        db_service::{DBServiceRequest, DBServiceResponse},
        farm_service::ServiceHandle,
    },
    web_server::BackendResponse,
    ServiceHandles,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct IdRequest {
    region: String,
}

pub async fn request_id(
    State(services): State<Arc<ServiceHandles>>,
    Json(data): Json<IdRequest>,
) -> impl IntoResponse {
    let id = match services
        .db_service
        .request(DBServiceRequest::CreateNewDevice {
            region: data.region,
        })
        .await
    {
        Ok(DBServiceResponse::DeviceId(id)) => id,
        Ok(..) => unreachable!(),
        Err(err) => todo!(),
    };
    (
        StatusCode::OK,
        Json(BackendResponse::DeviceId(id.iter().collect::<String>())),
    )
}
