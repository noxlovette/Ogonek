use crate::api::core::task;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        task::list_tasks,
        task::fetch_task,
        task::create_task,
        task::toggle_task,
        task::update_task,
        task::delete_task,
    ),
    components(schemas(
        crate::models::TaskSmall,
        crate::models::TaskFull,
        crate::models::TaskPaginationParams,
        crate::models::TaskUpdate,
        crate::models::TaskWithFilesResponse,
    ))
)]
pub struct TaskApi;
