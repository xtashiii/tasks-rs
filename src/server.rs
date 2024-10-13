use std::net::{IpAddr, Ipv4Addr};
use actix_files as fs;
use actix_cors::Cors;
use actix_web::{web::{delete, get, post, put}, App, HttpRequest, HttpResponse, HttpServer};
use crate::routes::{index, create_task, delete_task, get_tasks, update_task};

pub async fn start() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .route("/api", get().to(index))
            .route("/api/tasks", get().to(get_tasks))
            .route("/api/tasks/{name}", post().to(create_task))
            .route("/api/tasks/{id}", delete().to(delete_task))
            .route("/api/tasks/{id}", put().to(update_task))
            .service(fs::Files::new("/assets", "./dist/assets").show_files_listing())
            .route("/", get().to(fallback))
    })
    .bind((localhost, PORT))?
    .run().await
}

async fn fallback(req: HttpRequest) -> HttpResponse {
    match fs::NamedFile::open_async("./dist/index.html").await {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

