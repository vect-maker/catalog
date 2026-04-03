use libsql::Row;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct CreateProductDto {
    pub title: String,
    pub description: String,
    pub price: f64,
}
