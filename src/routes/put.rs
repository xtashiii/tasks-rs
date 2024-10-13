use actix_web::{web, HttpResponse, Responder};
use crate::{database, utils::UpdateTaskData};

pub async fn update_task(
    id: web::Path<String>,
    data: web::Json<UpdateTaskData>
) -> impl Responder {
    let connection = database::start_db();
    let task_id = id.into_inner();
    let task_data = data.into_inner();
    println!("{:#?}", task_data);

    if let Some(completed) = task_data.completed {
        if completed {
            if let Err(_) = connection.execute(
                "DELETE FROM task WHERE id = ?1",
                rusqlite::params![task_id]
            ) {
                return HttpResponse::InternalServerError().finish();
            }
            return HttpResponse::Ok().json("Task was completed and deleted successfully");
        }
         else {
            if let Err(e) = connection.execute(
                "UPDATE task SET completed = ?1 WHERE id = ?2",
                rusqlite::params![completed as i32, task_id]
            ) {
                println!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::BadRequest().json("Completed status is required")
}
