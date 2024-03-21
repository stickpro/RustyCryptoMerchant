use axum::Json;
use axum_macros::debug_handler;
use crate::dto::response::{MessageResponse, ServiceStatusResponse};
use crate::error::AppResult;

// Health check
#[utoipa::path(
    get,
    path = "/api/v1/server/health_check",
    responses(
        (status = 200, description = "check service is up", body = [MessageResponse])
    )
)]
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Ok")))
}

pub async fn server_state() -> AppResult<Json<ServiceStatusResponse>> {
    let resp = ServiceStatusResponse {
        db: true,
        redis: true,
        email: true,
    };
    Ok(Json(resp))
}