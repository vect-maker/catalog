use serde::Serialize;

pub mod product_dto;
pub mod user_dto;

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: i64,
}

#[derive(Serialize)]
pub struct CreateResponseId {
    pub id: String,
}
