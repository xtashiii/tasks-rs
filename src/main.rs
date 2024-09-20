use tasks::{server, utils};
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    utils::clear_terminal();
    println!("Server running at http://localhost:8080/");
    server::start().await
}