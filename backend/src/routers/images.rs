use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, header},
    response::IntoResponse,
    routing::get,
};

use crate::{AppError, AppState};
use libsql::params;

pub async fn get_image_handler(
    State(state): State<AppState>,
    Path(image_id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let mut rows = state
        .db_conn
        .query(
            "SELECT content_type, image_data FROM images WHERE id = ?1 LIMIT 1",
            params![image_id],
        )
        .await?;

    let row = rows.next().await?.ok_or(AppError::ImageNotFound)?;
    let content_type: String = row.get(0)?;
    let image_bytes: Vec<u8> = row.get(1)?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());

    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=31536000, immutable".parse().unwrap(),
    );

    Ok((headers, image_bytes))
}

pub fn use_routes() -> Router<AppState> {
    Router::new().route("/images/{image_id}", get(get_image_handler))
}
