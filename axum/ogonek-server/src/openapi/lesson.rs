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
        ogonek_types::LessonFull,
        ogonek_types::LessonSmall,
        ogonek_types::LessonUpdate,
    ))
)]
pub struct LessonApi;
