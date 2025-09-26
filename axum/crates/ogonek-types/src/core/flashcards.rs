use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::datetime_serialization;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SimpleStats {
    pub cards_studied_today: i32,
    pub current_streak: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
}
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardUpsert {
    pub id: Option<String>,
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
}

// DECK STRUCTS HERE
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCards {
    pub deck: DeckFull,
    pub cards: Vec<Card>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCardsUpdate {
    pub deck: DeckUpdate,
    pub cards: Vec<CardUpsert>,
}

#[derive(Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckFull {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub visibility: String,
    pub assignee: Option<String>,
    pub is_subscribed: Option<bool>,
    pub created_by: String,
    pub card_count: i64,

    #[serde(with = "datetime_serialization")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DeckSmall {
    pub id: String,
    pub title: String,
    pub assignee_name: Option<String>,
    pub is_subscribed: Option<bool>,
    pub seen: Option<bool>,
    pub visibility: String,
    pub card_count: i64,
    pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct DeckPublic {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckCreate {
    pub title: String,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub assignee: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub assignee: Option<String>,
}

use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardProgressWithFields {
    pub id: String,
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
}

#[derive(Serialize, Clone, Validate, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardProgress {
    pub id: String,
    pub user_id: String,
    pub card_id: String,
    #[validate(range(min = 0))]
    pub review_count: i32,
    #[serde(with = "datetime_serialization::option")]
    pub last_reviewed: Option<DateTime<Utc>>,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
    #[validate(range(min = 1.3, max = 5.0))]
    pub ease_factor: f64,
    #[validate(range(min = 1))]
    pub interval: i32,
}

#[derive(ToSchema, Deserialize)]
pub struct UpdateCardProgress {
    pub review_count: i32,
    #[serde(with = "datetime_serialization")]
    pub due_date: DateTime<Utc>,
    pub ease_factor: f64,
    pub interval: i32,
    #[serde(with = "datetime_serialization")]
    pub last_reviewed: DateTime<Utc>,
}

#[derive(ToSchema, Deserialize, Validate)]
pub struct ReviewPayload {
    #[validate(range(min = 0, max = 5))]
    pub quality: i32,
}
