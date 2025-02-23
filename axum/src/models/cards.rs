use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

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
pub struct DeckWithCards {
    pub deck: DeckBody,
    pub cards: Vec<CardBody>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckWithCardsUpdate {
    pub deck: DeckUpdate,
    pub cards: Vec<CardUpdate>
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckBody {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub shared: bool,
    pub created_by: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckCreateBody {
    pub name: String,
    pub description: Option<String>,
    pub shared: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub shared: Option<bool>
}

use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CardProgress {
    pub id: String,  // nanoid
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
}

#[derive(Debug, Deserialize)]
pub struct CardProgressCreate {
    pub card_id: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReviewPayload {
    pub card_id: String,
    #[validate(range(min = 0, max = 5))]
    pub quality: i32,
}