use std::any::Any;
use axum::http::{header, Response, StatusCode};
use crate::core::errors::error::{AppError, AppErrorEnum};
use http_body_util::Full;
use bytes::Bytes;

pub fn throw_error(err: AppError) {
    eprintln!("Error: {}", err);
}

pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Full<Bytes>> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };
    let app_error = AppError::new(AppErrorEnum::InternalServerError, details);
    eprintln!("Panic Error: {}", app_error);
    let body = serde_json::json!({
        "error_code": app_error.error_code,
    });
    let body = serde_json::to_string(&body).unwrap();
    
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/ json")
        .body(Full::from(body))
        .unwrap()
}