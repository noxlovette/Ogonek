use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use rust::db::tasks::{create_task, update_task};
use rust::middleware::check_sudo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaskCreationRequest {
    title: String,
    content: String,
    priority: i16,
    completed: bool,
    due_date: Option<chrono::DateTime<chrono::Utc>>,
    file: Option<String>,
    assignee_id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct TaskUpdateRequest {
    task_id: String,
    title: Option<String>,
    content: Option<String>,
    priority: Option<i16>,
    completed: Option<bool>,
    due_date: Option<chrono::DateTime<chrono::Utc>>,
    file: Option<String>,
    assignee_id: Option<uuid::Uuid>,
}

#[post("/create_task")]
pub async fn create_task_endpoint(
    req: HttpRequest,
    user_request: web::Json<TaskCreationRequest>,
) -> impl Responder {
    if let Err(response) = check_sudo(&req) {
        return response;
    }
    let user_request = user_request.into_inner(); // Convert Web JSON to struct

    match create_task(
        &user_request.title,
        &user_request.content,
        &user_request.priority,
        &user_request.completed,
        &user_request.due_date,
        &user_request.file,
        &user_request.assignee_id,
    ) {
        Ok(task) => HttpResponse::Ok().json(web::Json(serde_json::json!({"task": task}))),
        Err(e) => {
            eprintln!("Failed to create task: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create task")
        }
    }
}

#[post("/update_task")]
pub async fn update_task_endpoint(
    req: HttpRequest,
    user_request: web::Json<TaskUpdateRequest>,
) -> impl Responder {
    if let Err(response) = check_sudo(&req) {
        return response;
    }
    let user_request = user_request.into_inner(); // Convert Web JSON to struct

    match update_task(
        &user_request.task_id,
        user_request.title.as_deref(), // Convert Option<String> to Option<&str>
        user_request.content.as_deref(),
        user_request.priority.as_ref(),
        user_request.completed.as_ref(),
        user_request.due_date.as_ref(),
        user_request.file.as_deref(),
        user_request.assignee_id.as_ref(),
    ) {
        Ok(task) => HttpResponse::Ok().json(web::Json(serde_json::json!({"task": task}))),
        Err(e) => {
            eprintln!("Failed to update task: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update task")
        }
    }
}
