use crate::utils;
use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use libsql::params;

use crate::{
    AppError, AppState,
    contracts::{
        CreateResponse,
        product_dto::{CreateProductDto, ProductDto},
    },
};

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
    Json(payload): Json<CreateProductDto>,
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

pub fn use_routes() -> Router<AppState> {
    Router::new()
        .route("/products", get(get_products_handler))
        .route("/products", post(create_product_handler))
        .route("/products/{product_id}", get(get_product_handler))
        .route("/products/{product_id}/images", post(upload_image_handler))
}
