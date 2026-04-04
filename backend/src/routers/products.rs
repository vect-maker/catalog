use crate::{extractors::AuthUser, utils};
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
            "SELECT id, title, description, price, COALESCE((SELECT json_group_array(image_id) FROM product_images WHERE product_id = p.id), '[]') AS image_ids FROM products AS p LIMIT 20",
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
            "SELECT id, title, description, price, COALESCE((SELECT json_group_array(image_id) FROM product_images WHERE product_id = p.id), '[]') AS image_ids FROM products AS p WHERE id = ?1 LIMIT 1",
            params![product_id],
        )
        .await?;
    let row = rows.next().await?.ok_or(AppError::ProductNotFound)?;

    let product = ProductDto::try_from(&row)?;

    Ok(Json(product))
}

async fn create_product_handler(
    user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateProductDto>,
) -> Result<impl IntoResponse, AppError> {
    let mut rows = state
        .db_conn
        .query(
            "INSERT INTO products (title, description,  price,  published_by ) VALUES (?1, ?2, ?3, ?4) RETURNING id",
            params![payload.title, payload.description, payload.price, user.id],
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
    user: AuthUser,
    State(state): State<AppState>,
    Path(product_id): Path<u32>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let field = multipart.next_field().await?.ok_or(AppError::MissingFile)?;
    let content_type = "image/webp";
    let image_bytes = utils::convert_to_webp(field.bytes().await?).await?;

    let tx = state.db_conn.transaction().await?;

    // insert image data
    let mut rows = tx
        .query(
            "INSERT INTO images (content_type, image_data, uploaded_by) VALUES (?1, ?2, ?3) RETURNING id",
            params![content_type, image_bytes, user.id],
        )
        .await?;

    let row = rows.next().await?.ok_or(AppError::InsertFailed)?;
    let image_id: i64 = row.get(0)?;

    // set the id of the new image of the product
    tx.execute(
        "INSERT INTO  product_images (product_id, image_id) VALUES (?1, ?2)",
        params![product_id, image_id],
    )
    .await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(CreateResponse { id: image_id })))
}

pub fn use_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_products_handler))
        .route("/", post(create_product_handler))
        .route("/{product_id}", get(get_product_handler))
        .route("/{product_id}/images", post(upload_image_handler))
}
