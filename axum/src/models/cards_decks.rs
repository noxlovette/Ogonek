use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct DeckFilterParams {
    pub search: Option<String>,
    pub assignee: Option<String>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardBody {
    pub id: String,
    pub front: String,
    pub back: String,
    pub media_url: Option<String>,
    pub deck_id: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardCreateBody {
    pub front: String,
    pub back: String,
    pub media_url: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdate {
    pub id: Option<String>,
    pub front: Option<String>,
    pub back: Option<String>,
    pub media_url: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCardsAndSubscription {
    pub deck: DeckBody,
    pub cards: Vec<CardBody>,
    pub is_subscribed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCards {
    pub deck: DeckBody,
    pub cards: Vec<CardBody>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCardsUpdate {
    pub deck: DeckUpdate,
    pub cards: Vec<CardUpdate>
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DeckBody {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub visibility: String,
    pub assignee: Option<String>,
    pub created_by: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DeckBodySmall {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckCreateBody {
    pub name: String,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub assignee: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub assignee: Option<String>
}

use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CardProgressWithFields {
    pub id: String,
    pub user_id: String,
    pub card_id: String,
    #[validate(range(min = 0))]
    pub review_count: i32,
    pub last_reviewed: Option<OffsetDateTime>,
    pub due_date: Option<OffsetDateTime>,
    #[validate(range(min = 1.3, max = 5.0))]
    pub ease_factor: f64,
    #[validate(range(min = 1))]
    pub interval: i32,
    pub front: String,
    pub back: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CardProgress {
    pub id: String,
    pub user_id: String,
    pub card_id: String,
    #[validate(range(min = 0))]
    pub review_count: i32,
    pub last_reviewed: Option<OffsetDateTime>,
    pub due_date: Option<OffsetDateTime>,
    #[validate(range(min = 1.3, max = 5.0))]
    pub ease_factor: f64,
    #[validate(range(min = 1))]
    pub interval: i32
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReviewPayload {
    #[validate(range(min = 0, max = 5))]
    pub quality: i32,
}