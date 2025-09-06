use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub id: String,
    pub unsplash_id: String,
    pub urls: serde_json::Value,
    pub alt_description: Option<String>,
    pub photographer_name: String,
    pub photographer_username: String,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpsertPhoto {
    pub unsplash_id: String,
    pub alt_description: Option<String>,
    pub urls: URLs,
    pub user: UnsplashUser,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct URLs {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UnsplashUser {
    pub name: String,
    pub username: String,
}
