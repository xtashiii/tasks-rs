use actix_web::{web, HttpResponse, Responder};
use crate::database;

pub async fn delete_task(id: web::Path<String>) -> impl Responder {
    let connection = database::start_db();
    if let Err(_) = connection.execute("DELETE FROM task WHERE id = (?1)", rusqlite::params![id.trim()]) {
        return HttpResponse::InternalServerError().finish();
    };

    HttpResponse::Ok().body("Task deleted successfully")
}