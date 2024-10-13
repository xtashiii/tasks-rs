use actix_web::{web, HttpResponse, Responder};
use crate::{database, utils::Task};
use serde_json::json;

pub async fn create_task(name: web::Path<String>) -> impl Responder {
    let connection = database::start_db();

    let task_name = name.into_inner();
    let new_task = Task::new(task_name.clone());
    
    if let Err(_) = connection.execute(
        "INSERT INTO task (name, completed) VALUES (?1, ?2)",
        rusqlite::params![new_task.name.trim(), new_task.completed]
    ) {
        return HttpResponse::InternalServerError().finish();
    }

    let task_id = connection.last_insert_rowid();
    
    let created_task = Task {
        id: task_id as i32, 
        name: new_task.name,
        completed: new_task.completed,
    };
    
    HttpResponse::Ok().json(json!({
        "id": created_task.id,
        "name": created_task.name,
        "completed": created_task.completed,
    }))
}
