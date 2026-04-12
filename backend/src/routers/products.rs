use crate::{contracts::CreateResponseId, extractors::AuthUser, utils};
use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use libsql::params;

use crate::{
    AppError, AppState,
    contracts::{
        product_dto::{CreateProductDto, ProductDto},
    },
    utils::generate_uuid
};

async fn get_products_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = state.db.connect()?;

    let mut rows = db_conn
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
    Path(product_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
   let db_conn = state.db.connect()?; 

    let mut rows = 
        db_conn
        .query(
            "SELECT id, title, description, price, COALESCE((SELECT json_group_array(image_id) FROM product_images WHERE product_id = p.id), '[]') AS image_ids FROM products AS p WHERE id = ?1 LIMIT 1",
            params![product_id.clone()],
        )
        .await?;
    let row = rows.next().await?.ok_or(AppError::ProductNotFound)?;

    let product = ProductDto::try_from(&row)?;

    Ok(Json(product))
}

async fn delete_product_handler(
    user: AuthUser,
    State(state): State<AppState>,
    Path(product_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = state.db.connect()?;
    let tx = db_conn.transaction().await?;

    // delete associated images
    tx.execute(
        "
        DELETE FROM images 
        WHERE id IN  (
           SELECT image_id FROM product_images WHERE product_id = (
            SELECT id FROM products WHERE id = ?1 AND published_by = ?2
           )
        )
            ",
        params![product_id.clone(), user.id.clone()],
    )
    .await?;
    // delete the product

    let rows_affected = tx
        .execute(
            "DELETE FROM products WHERE id = ?1 AND published_by = ?2",
            params![product_id.clone(), user.id.clone()],
        )
        .await?;

    // check ownership
    if rows_affected == 0 {
        return Err(AppError::ProductNotFound);
    }

    tx.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_product_handler(
    user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateProductDto>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = state.db.connect()?;

    let product_id = generate_uuid();

     
    let rows_affected = db_conn
        .execute(
            "INSERT INTO products (id, title, description,  price,  published_by ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![product_id.clone(), payload.title, payload.description, payload.price, user.id.clone()],
        )
        .await?;

    if rows_affected == 0 {
        return Err(AppError::InsertFailed);
    }

    Ok((
        StatusCode::CREATED,
        Json(CreateResponseId { id: product_id  }),
    ))
}

pub async fn upload_image_handler(
    user: AuthUser,
    State(state): State<AppState>,
    Path(product_id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let field = multipart.next_field().await?.ok_or(AppError::MissingFile)?;
    let content_type = "image/webp";
    let image_bytes = utils::convert_to_webp(field.bytes().await?).await?;

    let db_conn = state.db.connect()?;

    let tx = db_conn.transaction().await?;

    let image_id = generate_uuid();

    // insert image data
    tx
        .execute(
            "INSERT INTO images (id, content_type, image_data, uploaded_by) VALUES (?1, ?2, ?3, ?4)",
            params![image_id.clone(), content_type, image_bytes, user.id.clone()],
        )
        .await?;


    let rows_affected = tx
        .execute(
            "INSERT INTO product_images (product_id, image_id) 
            SELECT id, ?2 FROM products WHERE published_by = ?3 AND id = ?1",
            params![product_id.clone(), image_id.clone(), user.id.clone()],
        )
        .await?;
    if rows_affected == 0 {
        return Err(AppError::ProductNotFound);
    }

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(CreateResponseId { id: image_id })))
}

pub fn use_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_products_handler))
        .route("/", post(create_product_handler))
        .route("/{product_id}", get(get_product_handler))
        .route("/{product_id}/images", post(upload_image_handler))
        .route("/{product_id}", delete(delete_product_handler))
}
