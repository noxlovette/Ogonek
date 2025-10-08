use crate::{DeckSmall, SortField, SortOrder, Visibility};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::{default_page, default_per_page};
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct DeckPaginationParams {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u32,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_per_page")]
    pub per_page: u32,

    #[serde(default)]
    pub search: Option<String>,

    #[serde(default)]
    pub sort_by: SortField,

    #[serde(default)]
    pub sort_order: SortOrder,

    #[serde(default)]
    pub assignee: Option<String>,

    #[serde(default)]
    pub visibility: Option<Visibility>,

    #[serde(default)]
    pub subscribed_only: Option<bool>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedDecks {
    pub data: Vec<DeckSmall>,
    pub page: i64,
    pub count: i64,
    pub total_pages: i64,
    pub per_page: i64,
}

impl DeckPaginationParams {
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }

    pub fn page(&self) -> i64 {
        self.page as i64
    }
}
