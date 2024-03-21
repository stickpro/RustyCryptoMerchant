use fake::Dummy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::error::AppResponseError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum AppResultResponse<R> {
    Err(AppResponseError),
    Ok(R),
}

impl<R> AppResultResponse<R> {
    #[allow(dead_code)]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, AppResultResponse::Ok(_))
    }
    #[allow(dead_code)]
    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Dummy, Clone)]
pub struct ServiceStatusResponse {
    pub db: bool,
    pub redis: bool,
    pub email: bool,
}