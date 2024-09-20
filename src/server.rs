use std::net::{IpAddr, Ipv4Addr};
use actix_web::{web::{delete, get, post, put}, App, HttpServer};
use crate::routes::{index, create_task, delete_task, get_tasks, update_task};

pub async fn start() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    HttpServer::new(move || {
        App::new()
            .route("/", get().to(index))
            .route("/tasks", get().to(get_tasks))
            .route("/tasks/{name}", post().to(create_task))
            .route("/tasks/{id}", delete().to(delete_task))
            .route("/tasks/{id}", put().to(update_task))
    })
    .bind((localhost, PORT))?
    .run().await
}