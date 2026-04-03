use serde::Serialize;

pub mod product_dto;

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: i64,
}
