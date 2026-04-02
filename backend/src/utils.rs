use crate::AppError;
use axum::body::Bytes;
use image::ImageFormat;
use std::io::Cursor;
use tokio::task;

pub async fn convert_to_webp(raw_bytes: Bytes) -> Result<Vec<u8>, AppError> {
    let webp_bytes = task::spawn_blocking(move || {
        let img = image::load_from_memory(&raw_bytes).map_err(|_| AppError::InvalidImageFormat)?;

        let mut buffer = Cursor::new(Vec::new());

        img.write_to(&mut buffer, ImageFormat::WebP)
            .map_err(|_| AppError::EncodingFailed)?;

        Ok::<Vec<u8>, AppError>(buffer.into_inner())
    })
    .await
    .map_err(|_| AppError::ThreadPanic)??;
    Ok(webp_bytes)
}
