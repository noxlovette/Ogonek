use crate::auth::jwt::Claims;
use crate::db::init::AppState;
use crate::models::cards_decks::{DeckBody, DeckCreateBody, CardBody, DeckWithCards, DeckWithCardsAndSubscription, DeckWithCardsUpdate};
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
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
        RETURNING id, name, description, visibility, assignee, created_by, created_at
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
) -> Result<Json<Option<DeckWithCardsAndSubscription>>, APIError> {
    let result = sqlx::query!(
        r#"
        SELECT id, name, description, visibility, assignee, created_by, created_at FROM decks 
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

    let is_subscribed = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM deck_subscriptions 
            WHERE deck_id = $1 AND user_id = $2
        ) as "is_subscribed!"
        "#,
        deck_id,
        claims.sub
    )
    .fetch_one(&state.db)
    .await?
    .is_subscribed;

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

            Ok(Json(Some(DeckWithCardsAndSubscription {
                deck: DeckBody {
                    id: deck_data.id,
                    name: deck_data.name,
                    description: deck_data.description,
                    visibility: deck_data.visibility,
                    created_by: deck_data.created_by,
                    assignee: deck_data.assignee,
                    created_at: deck_data.created_at,
                },
                cards,
                is_subscribed
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
         RETURNING id, name, description, visibility, assignee, created_by, created_at",
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

pub async fn subscribe_to_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,

) -> Result<StatusCode, APIError> {
    let deck_exists = sqlx::query!(
        r#"
        SELECT id FROM decks
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
    .await?
    .is_some();

    if !deck_exists {
        return Err(APIError::NotFound("Deck not found or access denied".into()));
    }

    sqlx::query!(
        r#"
        INSERT INTO deck_subscriptions (deck_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (deck_id, user_id) DO NOTHING
        "#,
        deck_id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    // Initialize card progress for all existing cards in the deck
    let mut tx = state.db.begin().await?;
    
    let cards = sqlx::query!(
        r#"
        SELECT c.id FROM cards c
        LEFT JOIN card_progress cp 
            ON cp.card_id = c.id 
            AND cp.user_id = $1
        WHERE c.deck_id = $2 
        AND cp.id IS NULL
        "#,
        claims.sub,
        deck_id
    )
    .fetch_all(&mut *tx)
    .await?;

    for card in cards {
        sqlx::query!(
            r#"
            INSERT INTO card_progress 
                (id, user_id, card_id, review_count, due_date, ease_factor, interval)
            VALUES 
                ($1, $2, $3, 0, CURRENT_TIMESTAMP, 2.5, 1)
            ON CONFLICT (user_id, card_id) DO NOTHING
            "#,
            nanoid::nanoid!(),
            claims.sub,
            card.id,
        )
        .execute(&mut *tx)
        .await?;
    }
    
    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}


pub async fn unsubscribe_from_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(deck_id): Path<String>,
) -> Result<StatusCode, APIError> {
    // Remove subscription
    sqlx::query!(
        r#"
        DELETE FROM deck_subscriptions
        WHERE deck_id = $1 AND user_id = $2
        "#,
        deck_id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    // Optionally, you can also clean up card progress
    sqlx::query!(
        r#"
        DELETE FROM card_progress cp
        USING cards c
        WHERE cp.card_id = c.id
        AND c.deck_id = $1
        AND cp.user_id = $2
        "#,
        deck_id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}