use crate::auth::jwt::Claims;
use crate::db::init::AppState;
use crate::models::cards_decks::{DeckBody, DeckCreateBody, CardBody, DeckWithCards, DeckWithCardsUpdate};
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use super::error::APIError;


pub async fn create_deck(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<DeckCreateBody>,
) -> Result<Json<DeckBody>, APIError> {

    let visibility = if payload.assignee.is_some() {
        payload.visibility.unwrap_or("assigned".to_string())
    } else {
        payload.visibility.unwrap_or("private".to_string())
    };

    let deck = sqlx::query_as!(
        DeckBody,
        r#"
        INSERT INTO decks (id, created_by, name, description, visibility, assignee)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        nanoid::nanoid!(),
        claims.sub,
        payload.name,
        payload.description,
        visibility,
        payload.assignee
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(deck))
}

pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<Json<Option<DeckWithCards>>, APIError> {
    let result = sqlx::query!(
        r#"
        SELECT * FROM decks 
        WHERE id = $1 AND (
            created_by = $2
            OR assignee = $2
            OR visibility = 'public'
        )
        "#,
        deck_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;

    match result {
        Some(deck_data) => {
            let cards = sqlx::query_as!(
                CardBody,
                r#"
                SELECT * FROM cards
                WHERE deck_id = $1
                ORDER BY created_at DESC
                "#,
                deck_id
            )
            .fetch_all(&state.db)
            .await?;

            Ok(Json(Some(DeckWithCards {
                deck: DeckBody {
                    id: deck_data.id,
                    name: deck_data.name,
                    description: deck_data.description,
                    visibility: deck_data.visibility,
                    created_by: deck_data.created_by,
                    assignee: deck_data.assignee,
                    created_at: deck_data.created_at,
                },
                cards
            })))
        },
        None => Ok(Json(None))
    }
}
pub async fn fetch_deck_list(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<DeckBody>>, APIError> {
    let decks = sqlx::query_as!(
        DeckBody,
        r#"
        SELECT id, name, description, visibility, assignee, created_by, created_at
        FROM decks
        WHERE created_by = $1 
           OR assignee = $1
           OR visibility = 'public'
        ORDER BY created_at DESC
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(decks))
}


pub async fn update_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
    Json(payload): Json<DeckWithCardsUpdate>,
) -> Result<Json<DeckWithCards>, APIError> {
    // Step 1: Update the deck
    let deck = sqlx::query_as!(
        DeckBody,
        "UPDATE decks 
         SET 
            name = COALESCE($1, name),
            description = COALESCE($2, description), 
            visibility = COALESCE($3, visibility),
            assignee = COALESCE($4, assignee)
         WHERE id = $5 AND created_by = $6
         RETURNING *",
        payload.deck.name,
        payload.deck.description,
        payload.deck.visibility,
        payload.deck.assignee,
        deck_id,
        claims.sub
    )
    .fetch_one(&state.db)
    .await?;

    let mut tx = state.db.begin().await?;

    // Step 2: Remove Cards That Are No Longer in Payload
    sqlx::query!(
        r#"
        DELETE FROM cards
        WHERE deck_id = $1 AND id NOT IN (
            SELECT UNNEST($2::text[])
        )
        "#,
        deck_id,
        &payload.cards.iter()
        .filter_map(|i| i.id.clone()).collect::<Vec<String>>()
    )
    .execute(&mut *tx)
    .await?;

    // Step 3: Upsert (Insert or Update) Cards
    for card in &payload.cards {
    let card_id = card.id.clone().unwrap_or_else(|| nanoid::nanoid!());
    sqlx::query!(
        r#"
        INSERT INTO cards (id, deck_id, front, back, media_url)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET 
            front = EXCLUDED.front,
            back = EXCLUDED.back,
            deck_id = EXCLUDED.deck_id,
            media_url = EXCLUDED.media_url
        RETURNING *
        "#,
        card_id,
        deck_id,
        card.front,
        card.back,
        card.media_url,
    )
    .fetch_one(&mut *tx)
    .await?;
}

    tx.commit().await?;

    // Fetch updated cards
    let updated_cards = sqlx::query_as!(
        CardBody,
        r#"
        SELECT * FROM cards WHERE deck_id = $1
        "#,
        deck_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(DeckWithCards { deck, cards: updated_cards }))
}


pub async fn delete_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<Json<DeckBody>, APIError> {
    let deck = sqlx::query_as!(
        DeckBody,
        r#"
        DELETE FROM decks
        WHERE id = $1 AND created_by = $2
        RETURNING id, name, description, created_by, visibility, assignee, created_at
        "#,
        deck_id,
        claims.sub
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(deck))
}

