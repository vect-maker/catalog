use axum::{
    Router,
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
};
use libsql::params;

use crate::{
    AppError, AppState,
    contracts::user_dto::{AuthTokenResponseDto, AuthenticateUserDto},
    utils::{create_jwt, verify_password},
};

async fn authenticate_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthenticateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = state.db.connect()?;

    let row = db_conn
        .query(
            "SELECT id, password_hash FROM users WHERE name = ?1",
            params![payload.name],
        )
        .await?
        .next()
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let user_id: String = row.get(0)?;
    let stored_hash: String = row.get(1)?;

    verify_password(&payload.password, &stored_hash)?;

    let token = create_jwt(user_id, &state.jwt_secret)?;

    Ok(Json(AuthTokenResponseDto { token }))
}

pub fn use_routes() -> Router<AppState> {
    Router::new().route("/authenticate", post(authenticate_user_handler))
}
