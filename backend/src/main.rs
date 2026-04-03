use axum::{Router, http::StatusCode, routing::get};

use libsql::Builder;

use serde::Deserialize;
use tokio::net::TcpListener;

mod contracts;
mod cors;
mod error;
mod routers;
mod shutdown;
mod utils;

use crate::cors::get_cors_layer;
use crate::error::AppError;

#[derive(Clone, Deserialize)]
pub struct AppEnv {
    pub db_url: String,
    pub db_token: String,
    pub client_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db_conn: libsql::Connection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get app env
    let app_env = envy::from_env::<AppEnv>().expect("FATAL: Missing required ENV variables");
    // create datbase
    let db = Builder::new_remote(app_env.db_url, app_env.db_token)
        .build()
        .await?;
    let db_conn = db.connect()?;

    match db_conn
        .execute_batch(include_str!("sql/create-tables.sql"))
        .await
    {
        Ok(_) => println!("Schema applied successfully."),
        Err(e) => eprintln!("Transaction failed and rolled back: {}", e),
    }

    // build app state
    let app_state = AppState { db_conn };

    // compose app
    let app = Router::new()
        .route("/health/live", get(health_check))
        .route("/health/ready", get(health_check))
        .merge(routers::products::use_routes())
        .merge(routers::images::use_routes())
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
