use std::fmt; // format module

use actix_web::{http::StatusCode, App, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    Internal,
    Unknown,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

/**
 * compares all the various implementation to return the message
 */
impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                cause: _,
                error_type: _,
            } => message.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::DbError,
            } => "the requested item was not found".to_string(),
            _ => "an unexpected error occurred".to_string(), // default
        }
    }

    pub fn db_error(error_place: &str, err: impl ToString) -> AppError {
        eprintln!("ERROR - {} - {}", error_place, err.to_string()); // simple error LOG in terminal
        AppError {
            message: Some("Database Error".to_string()), // general title showed to end-user
            cause: Some(err.to_string()),                // real err cause
            error_type: AppErrorType::DbError,
        }
    }

    pub fn internal_error(error_place: &str, err: impl ToString) -> AppError {
        eprintln!("ERROR - {} - {}", error_place, err.to_string());
        AppError {
            message: Some("Internal Error".to_string()),
            cause: Some(err.to_string()),
            error_type: AppErrorType::Internal,
        }
    }

    pub fn unknown(error_place: &str, err: impl ToString) -> AppError {
        eprintln!("ERROR - {} - {}", error_place, err.to_string());
        AppError {
            message: Some("Internal Error".to_string()),
            cause: Some("Unknown Error".to_string()),
            error_type: AppErrorType::Unknown,
        }
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

/*
    Mandatory:
        - implement Display => std::fmt::Display for AppError
        - implement Debug => #[derive(Debug)]
*/

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(), // AppError::message() line 23
        })
    }
}
