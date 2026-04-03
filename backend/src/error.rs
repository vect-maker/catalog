use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};

use serde_json::json;

pub enum AppError {
    Database(libsql::Error),
    Multipart(axum::extract::multipart::MultipartError),
    InsertFailed,
    MissingFile,
    ProductNotFound,
    ImageNotFound,
    EncodingFailed,
    ThreadPanic,
    InvalidImageFormat,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                eprintln!("--> [DB ERROR] {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal database failure",
                )
            }
            AppError::InsertFailed => (StatusCode::INTERNAL_SERVER_ERROR, "Insertion error"),
            AppError::MissingFile => (StatusCode::BAD_REQUEST, "Missing file"),
            AppError::Multipart(err) => {
                eprintln!("--> [MULTIPART ERROR] {}", err);
                (StatusCode::BAD_REQUEST, "Malformed upload stream")
            }
            AppError::ProductNotFound => (StatusCode::NOT_FOUND, "Could not find the product"),
            AppError::ImageNotFound => (StatusCode::NOT_FOUND, "Could not find the image"),
            AppError::ThreadPanic => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            AppError::EncodingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not encode the image",
            ),
            AppError::InvalidImageFormat => (StatusCode::BAD_REQUEST, "Invalid image format"),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

impl From<libsql::Error> for AppError {
    fn from(inner: libsql::Error) -> Self {
        AppError::Database(inner)
    }
}
impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(inner: axum::extract::multipart::MultipartError) -> Self {
        AppError::Multipart(inner)
    }
}
