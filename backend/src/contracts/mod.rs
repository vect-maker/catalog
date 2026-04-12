use serde::{Deserialize, Serialize};

use crate::error::AppError;
use libsql::{Row, Rows};
pub mod product_dto;
pub mod user_dto;

#[derive(Serialize)]
pub struct CreateResponseId {
    pub id: String,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default)]
    pub page: u32,
}

#[derive(Serialize)]
pub struct PaginatedItems<T> {
    pub page_size: u32,
    pub page: u32,
    pub is_last: bool,
    pub items: Vec<T>,
}

impl<T> PaginatedItems<T>
where
    for<'a> T: TryFrom<&'a Row, Error = libsql::Error> + Serialize,
    AppError: From<libsql::Error>,
{
    pub async fn new(mut rows: Rows, page: u32, page_size: u32) -> Result<Self, AppError> {
        // expect one extra element in rows as the max
        let extra_capacity = (page_size + 1) as usize;
        let mut items = Vec::with_capacity(extra_capacity);

        while let Some(row) = rows.next().await? {
            items.push(T::try_from(&row)?);
        }

        let is_last = items.len() < extra_capacity;

        // truncate the extra element use for the next page check
        items.truncate(page_size as usize);

        Ok(Self {
            page_size,
            page,
            items,
            is_last,
        })
    }
}
