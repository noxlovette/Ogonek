use crate::api::core::task;
use ogonek_types::{TaskFull, TaskSmall, TaskUpdate, TaskWithFilesResponse};
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
    components(schemas(TaskSmall, TaskFull, TaskUpdate, TaskWithFilesResponse,))
)]
pub struct TaskApi;
