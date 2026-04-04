use axum::{
    Router,
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
};
use libsql::params;

use crate::{AppError, AppState, contracts::user_dto::AuthenticateUserDto, utils::verify_password};

async fn authenticate_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthenticateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    let row = state
        .db_conn
        .query(
            "SELECT id, password_hash FROM users WHERE username = ?1",
            params![payload.username],
        )
        .await?
        .next()
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let user_id: i64 = row.get(0)?;
    let stored_hash: String = row.get(1)?;

    verify_password(&payload.password, &stored_hash)?;

    Ok(())
}

pub fn use_routes() -> Router<AppState> {
    Router::new().route("/authenticate", post(authenticate_user_handler))
}
