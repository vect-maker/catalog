use libsql::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ProductDto {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub images: Vec<u64>,
}

impl TryFrom<&Row> for ProductDto {
    type Error = libsql::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let image_ids_str: String = row.get(4)?;
        let image_ids: Vec<u64> = serde_json::from_str(&image_ids_str).unwrap_or_default();
        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            price: row.get(3)?,
            images: image_ids,
        })
    }
}

#[derive(Deserialize)]
pub struct CreateProductDto {
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
}
