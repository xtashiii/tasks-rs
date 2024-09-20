use actix_web::{web, HttpResponse, Responder};
use crate::{database, utils::UpdateTaskData};

pub async fn update_task(
    id: web::Path<String>,
    data: web::Json<UpdateTaskData>
) -> impl Responder {
    let connection = database::start_db();

    let task_id = id.into_inner();
    let task_data = data.into_inner();

   if let Some(new_name) = task_data.name {
    if let Err(_) = connection.execute(
        "UPDATE task SET name = (?1) WHERE id = (?2)",
        (new_name.trim(), task_id.clone())
    ) {
        return HttpResponse::InternalServerError().finish();
    }
   }

   if let Some(completed) = task_data.completed {
    if let Err(_) = connection.execute(
        "UPDATE  TASK set completed = (?1) WHERE id = (?2)",
        rusqlite::params![completed as i32, task_id]
    ) {
        return HttpResponse::InternalServerError().finish();
    }
   }  
    HttpResponse::Ok().body("Task updated successfully")
}