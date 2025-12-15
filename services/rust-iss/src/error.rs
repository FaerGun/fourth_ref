use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub enum ApiError {
    Database(sqlx::Error),
    Http(reqwest::Error),
    Validation(String),
    NotFound(String),
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Database(e) => write!(f, "Database error: {}", e),
            ApiError::Http(e) => write!(f, "HTTP error: {}", e),
            ApiError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub ok: bool,
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub trace_id: String,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub ok: bool,
    pub data: T,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self { ok: true, data }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::Database(_) => (
                StatusCode::OK,
                "DATABASE_ERROR".to_string(),
                "Database operation failed".to_string(),
            ),
            ApiError::Http(e) => {
                let code = if e.is_timeout() {
                    "UPSTREAM_TIMEOUT".to_string()
                } else if e.status().map(|s| s.as_u16() == 403).unwrap_or(false) {
                    "UPSTREAM_403".to_string()
                } else if e.status().map(|s| s.as_u16() == 404).unwrap_or(false) {
                    "UPSTREAM_404".to_string()
                } else {
                    "UPSTREAM_ERROR".to_string()
                };
                (StatusCode::OK, code, format!("External API error: {}", e))
            },
            ApiError::Validation(msg) => (
                StatusCode::OK,
                "VALIDATION_ERROR".to_string(),
                msg,
            ),
            ApiError::NotFound(msg) => (
                StatusCode::OK,
                "NOT_FOUND".to_string(),
                msg,
            ),
            ApiError::Internal(msg) => (
                StatusCode::OK,
                "INTERNAL_ERROR".to_string(),
                msg,
            ),
        };

        let trace_id = Uuid::new_v4().to_string();
        let error_response = ErrorResponse {
            ok: false,
            error: ErrorDetail {
                code,
                message,
                trace_id,
            },
        };

        (status, Json(error_response)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Database(err)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Http(err)
    }
}


