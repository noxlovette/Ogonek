use crate::{api::core::lesson, models::LessonSmall};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        lesson::list_lessons,
        lesson::fetch_lesson,
        lesson::create_lesson,
        lesson::update_lesson,
        lesson::delete_lesson,
    ),
    components(
        schemas(
            crate::models::LessonFull,
            crate::models::LessonSmall,
            crate::models::LessonUpdate,
            crate::models::PaginatedResponse<LessonSmall>,
        )
    ),
  
)]
pub struct LessonApi;
