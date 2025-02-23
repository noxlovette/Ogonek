use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::cards::{DeckBody, DeckCreateBody, CardBody, DeckWithCards, DeckWithCardsUpdate};
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;


pub async fn create_deck(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<DeckCreateBody>,
) -> Result<Json<DeckBody>, DbError> {
    let deck = sqlx::query_as!(
        DeckBody,
        r#"
        INSERT INTO decks (id, created_by, name, description, shared)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        nanoid::nanoid!(),
        claims.sub,
        payload.name,
        payload.description,
        payload.shared
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(deck))
}

pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<Json<Option<DeckWithCards>>, DbError> {
    // First attempt: fetch owned deck
    let deck = sqlx::query_as!(
        DeckBody,
        r#"
        SELECT * FROM decks
        WHERE id = $1 AND created_by = $2
        "#,
        deck_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;

    // Second attempt: fetch shared deck
    let deck = match deck {
        Some(d) => d,
        None => {
            let shared = sqlx::query_as!(
                DeckBody,
                r#"
                SELECT d.*
                FROM decks d
                JOIN teacher_student ts
                    ON (ts.teacher_id = d.created_by AND ts.student_id = $1)
                    OR (ts.student_id = d.created_by AND ts.teacher_id = $1)
                WHERE d.id = $2 AND d.shared = TRUE
                "#,
                claims.sub,
                deck_id
            )
            .fetch_optional(&state.db)
            .await?;
            
            match shared {
                Some(d) => d,
                None => return Ok(Json(None))
            }
        }
    };

    // Fetch associated cards
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

    Ok(Json(Some(DeckWithCards { deck, cards })))
}


pub async fn fetch_deck_list(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<DeckBody>>, DbError> {
    let decks = sqlx::query_as!(
        DeckBody,
        r#"
        SELECT d.id, d.name, 
               d.description, 
               d.shared, 
               d.created_by, 
               d.created_at
        FROM decks d
        LEFT JOIN teacher_student ts
            ON (ts.teacher_id = d.created_by AND ts.student_id = $1)
            OR (ts.student_id = d.created_by AND ts.teacher_id = $1)
        WHERE d.created_by = $1 OR (d.shared = TRUE AND ts.teacher_id IS NOT NULL)

        ORDER BY d.created_at DESC
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(decks))
}

#[axum::debug_handler]
pub async fn update_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
    Json(payload): Json<DeckWithCardsUpdate>,
) -> Result<Json<DeckWithCards>, DbError> {
    // Step 1: Update the deck
    let deck = sqlx::query_as!(
        DeckBody,
        "UPDATE decks 
         SET 
            name = COALESCE($1, name),
            description = COALESCE($2, description), 
            shared = COALESCE($3, shared)
         WHERE id = $4 AND created_by = $5
         RETURNING *",
        payload.deck.name,
        payload.deck.description,
        payload.deck.shared,
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
) -> Result<Json<DeckBody>, DbError> {
    let deck = sqlx::query_as!(
        DeckBody,
        r#"
        DELETE FROM decks
        WHERE id = $1 AND created_by = $2
        RETURNING *
        "#,
        deck_id,
        claims.sub
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(deck))
}
