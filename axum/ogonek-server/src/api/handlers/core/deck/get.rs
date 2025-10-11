use axum::{
    Json,
    extract::{Path, Query, State},
};
use ogonek_db::{core::flashcards, tracking::mark_as_seen};
use ogonek_types::{
    DeckPaginationParams, DeckPublic, DeckSmall, DeckWithCards, ModelType, PaginatedDecks,
    PaginatedResponse, SortField, SortOrder, Visibility,
};

use crate::{AppError, AppState, Claims, api::DECK_TAG};

/// Retrieves a single deck with all its cards
///
/// Returns deck details including all associated flashcards and marks it as seen.
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Deck ID")
    ),
    responses(
        (status = 200, description = "Deck retrieved", body = DeckWithCards),
        (status = 404, description = "Deck not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_deck(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<Json<DeckWithCards>, AppError> {
    let deck_with_cards =
        flashcards::deck::read_deck_with_cards(&state.db, &id, &claims.sub).await?;

    mark_as_seen(&state.db, &claims.sub, &id, ModelType::Deck).await?;

    Ok(Json(deck_with_cards))
}

/// Retrieves a paginated list of decks accessible to the user
///
/// Returns decks the user owns or has been assigned with pagination and filtering support.
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("assignee" = Option<String>, Query, description = "Filter by assignee"),
        ("visibility" = Option<Visibility>, Query),
        ("sort_by" = Option<SortField>, Query),
        ("sort_order" = Option<SortOrder>, Query)
    ),
    responses(
        (status = 200, description = "User decks retrieved", body = PaginatedDecks),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_decks(
    State(state): State<AppState>,
    Query(params): Query<DeckPaginationParams>,
    claims: Claims,
) -> Result<Json<PaginatedResponse<DeckSmall>>, AppError> {
    let (decks, count) = flashcards::deck::read_all(&state.db, &claims.sub, &params).await?;

    let total_pages = (count as f64 / params.limit() as f64).ceil() as i64;
    Ok(Json(PaginatedResponse {
        data: decks,
        page: params.page(),
        total_pages,
        count,
        per_page: params.limit(),
    }))
}

/// Retrieves all publicly available decks
///
/// Returns a list of all decks marked as public visibility.
#[utoipa::path(
    get,
    tag = DECK_TAG,
    path = "/public",
    responses(
        (status = 200, description = "Public decks retrieved", body=Vec<DeckPublic>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_decks_public(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeckPublic>>, AppError> {
    let decks = flashcards::deck::read_all_public(&state.db).await?;

    Ok(Json(decks))
}
