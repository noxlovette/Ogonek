use actix_web::{web, Responder, HttpResponse, HttpRequest, post};
use rust::db::tasks::create_task;
use serde::Deserialize;
use rust::middleware::check_sudo;

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


#[post("/create_task")]
pub async fn create_task_endpoint(req: HttpRequest, user_request: web::Json<TaskCreationRequest>) -> impl Responder {
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
        &user_request.assignee_id
    ) {
        Ok(task) => HttpResponse::Ok().json(web::Json(serde_json::json!({"task": task}))),
        Err(e) => {
            eprintln!("Failed to create task: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create task")
        },
    }
}