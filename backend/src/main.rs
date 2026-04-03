use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    http::{HeaderMap, header},
    response::IntoResponse,
    response::Response,
    routing::{get, post},
};

use axum::http::{HeaderValue, Method};
use libsql::Builder;
use libsql::Row;
use libsql::params;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
mod shutdown;
mod utils;

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

#[derive(Deserialize)]
struct CreateProduct {
    pub title: String,
    pub description: String,
    pub price: f64,
}

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

#[derive(Serialize)]
pub struct ProductDto {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub image_id: Option<i64>,
}

impl TryFrom<&Row> for ProductDto {
    type Error = libsql::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            price: row.get(3)?,
            image_id: row.get(4)?,
        })
    }
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: i64,
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

    // setup cors rules
    let cors = CorsLayer::new()
        .allow_origin(app_env.client_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    // compose app
    let app = Router::new()
        .route("/health/live", get(health_check))
        .route("/health/ready", get(health_check))
        .route("/products", post(create_product_handler))
        .route("/products", get(get_products_handler))
        .route("/products/{product_id}", get(get_product_handler))
        .route("/products/{product_id}/images", post(upload_image_handler))
        .route("/images/{image_id}", get(get_image_handler))
        .with_state(app_state)
        .layer(cors);

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

async fn get_products_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let mut rows = state
        .db_conn
        .query(
            "SELECT id, title, description, price, image_id FROM products LIMIT 40",
            (),
        )
        .await?;

    let mut products: Vec<ProductDto> = Vec::new();

    while let Ok(Some(row)) = rows.next().await {
        products.push(ProductDto::try_from(&row)?)
    }

    Ok(Json(products))
}

async fn get_product_handler(
    State(state): State<AppState>,
    Path(product_id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let mut rows = state
        .db_conn
        .query(
            "SELECT id, title, description, price, image_id FROM products WHERE id = ?1 LIMIT 1",
            params![product_id],
        )
        .await?;
    let row = rows.next().await?.ok_or(AppError::ProductNotFound)?;

    let product = ProductDto::try_from(&row)?;

    Ok(Json(product))
}

async fn create_product_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> Result<impl IntoResponse, AppError> {
    let mut rows = state
        .db_conn
        .query(
            "INSERT INTO products (title, description,  price ) VALUES (?1, ?2, ?3) RETURNING id",
            params![payload.title, payload.description, payload.price],
        )
        .await?;

    let row = rows.next().await?.ok_or(AppError::InsertFailed)?;
    let new_product_id: i64 = row.get(0)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateResponse { id: new_product_id }),
    ))
}

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

pub async fn upload_image_handler(
    State(state): State<AppState>,
    Path(product_id): Path<u32>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let field = multipart.next_field().await?.ok_or(AppError::MissingFile)?;
    let content_type = "image/webp";
    let image_bytes = utils::convert_to_webp(field.bytes().await?).await?;

    let tx = state.db_conn.transaction().await?;

    // remove current image data
    let mut rows = state
        .db_conn
        .query(
            "SELECT image_id FROM products WHERE id = ?1 LIMIT 1",
            params![product_id],
        )
        .await?;
    let row = rows.next().await?.ok_or(AppError::ProductNotFound)?;
    let current_image_id: Option<i64> = row.get(0)?;

    if let Some(current_image_id) = current_image_id {
        tx.execute(
            "DELETE FROM images WHERE id = ?1",
            params![current_image_id],
        )
        .await?;
    }

    // insert image data
    let mut rows = tx
        .query(
            "INSERT INTO images (content_type, image_data) VALUES (?1, ?2) RETURNING id",
            params![content_type, image_bytes],
        )
        .await?;

    let row = rows.next().await?.ok_or(AppError::InsertFailed)?;
    let new_image_id: i64 = row.get(0)?;

    // set the id of the new image of the product
    tx.execute(
        "UPDATE products SET image_id = ?1 WHERE id = ?2",
        params![new_image_id, product_id],
    )
    .await?;

    tx.commit().await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateResponse { id: new_image_id }),
    ))
}
