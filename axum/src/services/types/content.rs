use crate::types::datetime_serialization;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub markdown: String,
    pub meta_description: Option<String>,
    pub version: i32,
    pub status: ContentStatus,
    #[serde(with = "datetime_serialization::option")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "datetime_serialization")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub updated_by: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContent {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub markdown: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContentPublic {
    pub title: String,
    pub markdown: String,
    pub meta_description: Option<String>,
}
#[derive(sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum ContentStatus {
    Draft,
    Published,
}

impl FromStr for ContentStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(ContentStatus::Draft),
            "published" => Ok(ContentStatus::Published),
            _ => Err(anyhow::anyhow!("Invalid user role: {}", s)),
        }
    }
}
