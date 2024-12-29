use sha1::{Digest, Sha1};
use std::fmt;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde_json::json;

pub struct AppError {
    pub error_type: AppErrorEnum,
    pub message: String,
    pub error_code: String
}

pub enum AppErrorEnum {
    IoError,    // Standard IO error
    DatabaseQueryError,
    DatabaseConnectionError,
    InvalidInputError,
    NotFoundError,
    UnauthorizedError,
    InternalServerError,
    BadRequestError,
    UnknownError,
    UserMismatch,
    InsufficientPermissions,
}

impl AppError {
    pub fn new(error_type: AppErrorEnum, message: String) -> Self {
        // amazonq-ignore-next-line
        let mut hasher = Sha1::new();
        hasher.update(Utc::now().to_string());
        let error_code =  format!("{:x}", hasher.finalize());

        AppError {
            error_type,
            message,
            error_code
        }
    }

    fn format_message(&self) -> String {
        match self.error_type {
            AppErrorEnum::IoError => format!("I/O Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::DatabaseQueryError => format!("Database Query Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::DatabaseConnectionError => format!("Database Connection Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::InvalidInputError => format!("Invalid Input Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::NotFoundError => format!("Not Found Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::UnauthorizedError => format!("Unauthorized Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::InternalServerError => format!("Internal Server Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::BadRequestError => format!("Bad Request Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::UnknownError => format!("Unknown Error [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::UserMismatch => format!("User Mismatch [{}]: {1}", self.error_code, self.message),
            AppErrorEnum::InsufficientPermissions => format!("Insufficient Permissions [{}]: {1}", self.error_code, self.message),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.error_type {
            AppErrorEnum::IoError => (StatusCode::INTERNAL_SERVER_ERROR, self.error_code),
            AppErrorEnum::DatabaseQueryError => (StatusCode::INTERNAL_SERVER_ERROR, self.error_code),
            AppErrorEnum::DatabaseConnectionError => (StatusCode::INTERNAL_SERVER_ERROR, self.error_code),
            AppErrorEnum::InvalidInputError => (StatusCode::BAD_REQUEST, self.error_code),
            AppErrorEnum::NotFoundError => (StatusCode::NOT_FOUND, self.error_code),
            AppErrorEnum::UnauthorizedError => (StatusCode::UNAUTHORIZED, self.error_code),
            AppErrorEnum::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.error_code),
            AppErrorEnum::BadRequestError => (StatusCode::BAD_REQUEST, self.error_code),
            AppErrorEnum::UnknownError => (StatusCode::INTERNAL_SERVER_ERROR, self.error_code),
            AppErrorEnum::UserMismatch => (StatusCode::UNAUTHORIZED, self.error_code),
            AppErrorEnum::InsufficientPermissions => (StatusCode::UNAUTHORIZED, self.error_code),
        };
        let body = Json(json!({
            "error_code": error_message,
        }));
        (status, body).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_message())
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: {}", self.format_message())
    }
}

impl Error for AppError {}