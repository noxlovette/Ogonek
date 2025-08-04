use crate::api::core::lesson;
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
    components(schemas(
        crate::models::LessonFull,
        crate::models::LessonSmall,
        crate::models::LessonUpdate,
    ))
)]
pub struct LessonApi;
