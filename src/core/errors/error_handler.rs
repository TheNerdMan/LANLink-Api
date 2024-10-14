use crate::core::errors::error::AppError;

pub fn throw_error(err: AppError) {
    eprintln!("Error: {}", err);
}