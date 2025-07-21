use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
pub struct DeckFilterParams {
    pub search: Option<String>,
    pub assignee: Option<String>,
}
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SimpleStats {
    pub cards_studied_today: i32,
    pub current_streak: i32,
}
#[derive(ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LearnDataDashboard {
    pub stats: SimpleStats,
    pub due_cards: Option<i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
    pub deck_id: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardCreate {
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdate {
    pub id: Option<String>,
    pub front: Option<String>,
    pub back: Option<String>,
    pub media_url: Option<String>,
}

// DECK STRUCTS HERE

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCardsAndSubscription {
    pub deck: DeckFull,
    pub cards: Vec<Card>,
    pub is_subscribed: bool,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCardsUpdate {
    pub deck: DeckUpdate,
    pub cards: Vec<CardUpdate>,
}

#[derive(Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckFull {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub visibility: String,
    pub assignee: Option<String>,
    pub is_subscribed: Option<bool>,
    pub created_by: String,

    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DeckSmall {
    pub id: String,
    pub name: String,
    pub assignee_name: Option<String>,
    pub is_subscribed: Option<bool>,
    pub seen: Option<bool>,
    pub visibility: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct DeckPublic {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckCreate {
    pub name: String,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub assignee: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeckUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub assignee: Option<String>,
}

use utoipa::ToSchema;
// LEARNING HERE
use validator::Validate;

#[derive(Deserialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardProgressWithFields {
    pub id: String,
    pub user_id: String,
    pub card_id: String,
    #[validate(range(min = 0))]
    pub review_count: i32,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    #[validate(range(min = 1.3, max = 5.0))]
    pub ease_factor: f64,
    #[validate(range(min = 1))]
    pub interval: i32,
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
}

#[derive(Serialize, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CardProgress {
    pub id: String,
    pub user_id: String,
    pub card_id: String,
    #[validate(range(min = 0))]
    pub review_count: i32,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    #[validate(range(min = 1.3, max = 5.0))]
    pub ease_factor: f64,
    #[validate(range(min = 1))]
    pub interval: i32,
}

#[derive(ToSchema, Deserialize)]
pub struct UpdateCardProgress {
    pub review_count: i32,
    pub due_date: DateTime<Utc>,
    pub ease_factor: f64,
    pub interval: i32,
    pub last_reviewed: DateTime<Utc>,
}

#[derive(ToSchema, Deserialize, Validate)]
pub struct ReviewPayload {
    #[validate(range(min = 0, max = 5))]
    pub quality: i32,
}
