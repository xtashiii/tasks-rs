use actix_web::{web, HttpResponse, Responder};
use crate::{database, utils::Task};

pub async fn create_task(name: web::Path<String>) -> impl Responder {
    let connection = database::start_db();

    let task = Task::new(name.into_inner());

    if let Err(_) = connection.execute(
        "INSERT INTO task (name, completed) VALUES (?1, ?2)",
        rusqlite::params![task.name.trim(), false]
    ) {
        return HttpResponse::InternalServerError().finish();
    }
   
    HttpResponse::Ok().body("Task created successfully")
}
