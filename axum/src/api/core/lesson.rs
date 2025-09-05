use crate::api::LESSON_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::lesson;
use crate::db::crud::photo;
use crate::db::crud::tracking::{self, activity::log_activity};
use crate::notifications::messages::NotificationType;
use crate::schema::AppState;
use crate::types::{
    ActionType, LessonSmall, LessonUpdate, LessonWithPhoto, ModelType, PaginatedLessons,
    PaginatedResponse, PaginationParams, Photo, UpsertPhoto,
};
use axum::extract::Path;
use axum::extract::State;
use axum::extract::{Json, Query};
use axum::http::StatusCode;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(
        list_lessons,
        create_lesson,
        delete_lesson,
        update_lesson,
        upsert_photo,
    ))
}

/// Fetches lesson by id
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Lesson ID")
    ),
    tag = LESSON_TAG,responses(
        (status = 200, description = "Lesson retrieved successfully", body = LessonWithPhoto),
        (status = 404, description = "Lesson not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<LessonWithPhoto>, APIError> {
    let lesson = lesson::find_by_id(&state.db, &id, &claims.sub).await?;
    tracking::seen::mark_as_seen(&state.db, &claims.sub, &id, ModelType::Lesson).await?;

    let mut photo: Option<Photo> = None;

    if let Some(photo_id) = &lesson.photo_id {
        photo = photo::find_by_id(&state.db, photo_id).await?;
    }

    Ok(Json(LessonWithPhoto { lesson, photo }))
}
/// Lessons belonging to a user
#[utoipa::path(
    get,
    path = "",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("per_page" = Option<u32>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("assignee" = Option<String>, Query, description = "Filter by assignee")
    ),
    tag = LESSON_TAG,responses(
        (status = 200, description = "Lessons retrieved successfully", body = PaginatedLessons),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
    claims: Claims,
) -> Result<Json<PaginatedResponse<LessonSmall>>, APIError> {
    let lessons = lesson::find_all(&state.db, &claims.sub, &params).await?;

    Ok(Json(PaginatedResponse {
        data: lessons,
        page: params.page(),
        per_page: params.limit(),
    }))
}

/// Creates a lesson with user defaults specified elsewhere
#[utoipa::path(
    post,
    path = "",
    tag = LESSON_TAG,responses(
        (status = 201, description = "Lesson created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_lesson(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<String>, APIError> {
    let id = lesson::create_with_defaults(&state.db, &claims.sub).await?;

    log_activity(
        &state.db,
        &claims.sub,
        &id,
        ModelType::Lesson,
        ActionType::Create,
        None,
    )
    .await?;

    Ok(Json(id))
}
/// Deletes lesson
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Lesson ID")
    ),
    tag = LESSON_TAG,responses(
        (status = 204, description = "Lesson deleted successfully"),
        (status = 404, description = "Lesson not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    let assignee = lesson::find_assignee(&state.db, &id, &claims.sub).await?;
    lesson::delete(&state.db, &id, &claims.sub).await?;

    if let Some(user) = assignee {
        // clean up, otherwise you get hanging counts
        tracking::seen::delete_seen(&state.db, &user, &id, ModelType::Lesson).await?;

        // log deletion activity
        log_activity(
            &state.db,
            &claims.sub,
            &id,
            ModelType::Lesson,
            ActionType::Delete,
            Some(&user),
        )
        .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Updates lesson
#[utoipa::path(
    patch,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Lesson ID")
    ),
    request_body = LessonUpdate,
    tag = LESSON_TAG,responses(
        (status = 204, description = "Lesson updated successfully"),
        (status = 404, description = "Lesson not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<LessonUpdate>,
) -> Result<StatusCode, APIError> {
    // fetch assignee before update
    let current_assignee = lesson::find_assignee(&state.db, &id, &claims.sub).await?;

    // update the lesson
    lesson::update(&state.db, &id, &claims.sub, &payload).await?;

    // check if assignee changed
    let new_assignee = payload.assignee.clone();

    if new_assignee != current_assignee {
        // remove seen record for old assignee
        if let Some(old_user) = current_assignee {
            tracking::seen::delete_seen(&state.db, &old_user, &id, ModelType::Lesson).await?;
            // treat as deletion
            log_activity(
                &state.db,
                &claims.sub,
                &id,
                ModelType::Lesson,
                ActionType::Delete,
                Some(&old_user),
            )
            .await?;
        }

        // insert unseen for new assignee
        if let Some(new_user) = new_assignee {
            tracking::seen::insert_as_unseen(&state.db, &new_user, &id, ModelType::Lesson).await?;

            // treat as creation
            log_activity(
                &state.db,
                &claims.sub,
                &id,
                ModelType::Lesson,
                ActionType::Create,
                Some(&new_user),
            )
            .await?;

            let lesson = lesson::find_by_id(&state.db, &id, &claims.sub).await?;
            let _ = state
                .notification_service
                .notify_student(
                    &claims.sub,
                    &new_user,
                    NotificationType::LessonCreated {
                        lesson_id: lesson.id,
                        lesson_topic: lesson.topic,
                    },
                )
                .await;
        }
    } else if let Some(assignee) = current_assignee {
        // treat as update
        log_activity(
            &state.db,
            &claims.sub,
            &id,
            ModelType::Lesson,
            ActionType::Update,
            Some(&assignee),
        )
        .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Adds a photo to the lesson
#[utoipa::path(
    patch,
    path = "/{id}/photo",
    params(
        ("id" = String, Path, description = "Lesson ID")
    ),
    request_body = UpsertPhoto,
    tag = LESSON_TAG,responses(
        (status = 204, description = "Lesson updated successfully"),
        (status = 404, description = "Lesson not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn upsert_photo(
    State(state): State<AppState>,
    Path(lesson_id): Path<String>,
    claims: Claims,
    Json(payload): Json<UpsertPhoto>,
) -> Result<StatusCode, APIError> {
    photo::upsert(&state.db, &lesson_id, &claims.sub, &payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Deletes a photo from the lesson
#[utoipa::path(
    patch,
    path = "/{id}/photo",
    params(
    ("id" = String, Path, description = "Lesson ID")
    ),
    tag = LESSON_TAG,responses(
        (status = 204, description = "Photo deleted successfully"),
        (status = 404, description = "Lesson not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_photo(
    State(state): State<AppState>,
    Path(lesson_id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    photo::delete(&state.db, &lesson_id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}
