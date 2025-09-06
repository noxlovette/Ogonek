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
        lesson::upsert_photo,
        lesson::delete_photo
    ),
    components(schemas(
        crate::types::LessonFull,
        crate::types::LessonSmall,
        crate::types::LessonUpdate,
    ))
)]
pub struct LessonApi;
