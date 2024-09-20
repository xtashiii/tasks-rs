use actix_web::{HttpResponse, Responder};
use crate::{database, utils::Task};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("api is working 🐭")
}

pub async fn get_tasks() -> impl Responder {
    let connection = database::start_db();

    let mut stmt = connection.prepare("SELECT id, name, completed FROM task").unwrap();

    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            completed: row.get(2)?,
        })
    }).unwrap();

    let mut tasks = Vec::new();
    for task in tasks_iter {
        tasks.push(task.unwrap());
    }

    HttpResponse::Ok().json(tasks)
}
