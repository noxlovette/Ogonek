use crate::{
    api::core::task,
    models::{TaskFull, TaskPaginationParams, TaskSmall, TaskUpdate, TaskWithFilesResponse},
};
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
        TaskSmall,
        TaskFull,
        TaskPaginationParams,
        TaskUpdate,
        TaskWithFilesResponse,
    ))
)]
pub struct TaskApi;
