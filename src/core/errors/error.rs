use std::fmt;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::core::errors::auth_errors::AuthError;
use crate::core::errors::user_errors::UserError;

pub enum AppError {
    IoError(std::io::Error),    // Standard IO error
    DatabaseQueryError(String),
    DatabaseConnectionError(String),
    InvalidInputError(String),
    NotFoundError(String),
    UnauthorizedError(String),
    InternalServerError(String),
    BadRequestError(String),
    UnknownError(String),
}

impl AppError {
    fn format_message(&self) -> String {
        match self {
            AppError::IoError(e) => format!("I/O Error: {}", e),
            AppError::DatabaseQueryError(msg) => format!("Database Query Error: {}", msg),
            AppError::DatabaseConnectionError(msg) => format!("Database Connection Error: {}", msg),
            AppError::InvalidInputError(msg) => format!("Invalid Input Error: {}", msg),
            AppError::NotFoundError(msg) => format!("Not Found Error: {}", msg),
            AppError::UnauthorizedError(msg) => format!("Unauthorized Error: {}", msg),
            AppError::InternalServerError(msg) => format!("Internal Server Error: {}", msg),
            AppError::BadRequestError(msg) => format!("Bad Request Error: {}", msg),
            AppError::UnknownError(msg) => format!("Unknown Error: {}", msg),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_message())
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: {}", self.format_message())
    }
}

impl Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::IoError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),    // Standard IO error
            AppError::DatabaseQueryError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            AppError::DatabaseConnectionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            AppError::InvalidInputError(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::NotFoundError(msg) => (StatusCode::NO_CONTENT, msg.to_string()),
            AppError::UnauthorizedError(msg) => (StatusCode::UNAUTHORIZED, msg.to_string()),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            AppError::BadRequestError(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::UnknownError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}