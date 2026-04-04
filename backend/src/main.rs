use axum::{Router, http::StatusCode, routing::get};
use libsql::Builder;
use libsql::params;
use tokio::net::TcpListener;

mod contracts;
mod cors;
mod env;
mod error;
mod extractors;
mod routers;
mod shutdown;
mod utils;

use crate::cors::get_cors_layer;
use crate::env::get_env;
use crate::error::AppError;
use crate::utils::hash_password;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: libsql::Connection,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get app env
    let app_env = get_env();
    // create datbase
    let db = Builder::new_remote(app_env.db_url, app_env.db_token)
        .build()
        .await?;
    let db_conn = db.connect()?;

    db_conn.execute("PRAGMA foreign_keys = ON;", ()).await?;

    match db_conn
        .execute_batch(include_str!("sql/create-tables.sql"))
        .await
    {
        Ok(_) => println!("Schema applied successfully."),
        Err(e) => eprintln!("Transaction failed and rolled back: {}", e),
    };

    // create default user

    db_conn
        .execute(
            "
        INSERT INTO users (name, password_hash)
        VALUES (?1, ?2)
        ON CONFLICT (name) DO NOTHING;
        ",
            params!["admin", hash_password("1234")],
        )
        .await?;

    // build app state
    let app_state = AppState {
        db_conn,
        jwt_secret: app_env.secret_key.clone(),
    };

    // compose app
    let app = Router::new()
        .route("/health/live", get(health_check))
        .route("/health/ready", get(health_check))
        .nest("/products", routers::products::use_routes())
        .nest("/images", routers::images::use_routes())
        .nest("/users", routers::users::use_routes())
        .with_state(app_state)
        .layer(get_cors_layer(&app_env.client_url));

    let listener = TcpListener::bind("0.0.0.0:80").await?;
    println!("server running on http://{}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await?;

    Ok(())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
