use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};

use tower_http::services::ServeDir;

use dotenv::dotenv;
use std::env;

mod handlers;
mod repository;
mod service;

use crate::handlers::todo::todo_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Парсинг данных с .env файла
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Cannot find in .env variable DATABASE_URL");
    let port: u16 = env::var("PORT")
        .expect("Cannot find PORT var in .env")
        .parse()
        .unwrap_or(0);

    let ip_addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(ip_addr).await?;

    // В случаи если порт не задан в .env он задаётся рандомно и указывается актуальный
    // адрес здесь
    let actuall_ip = listener.local_addr()?;

    let todo_service: Arc<service::todo::Service> =
        Arc::new(service::todo::Service::new(&db_url).await?);

    println!("Server is connect to: {}", actuall_ip);

    axum::serve(listener, app(todo_service)).await?;

    Ok(())
}

fn app(service: Arc<service::todo::Service>) -> Router {
    let static_folder = ServeDir::new("static/");
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/", get(handlers::hello_api))
                .route("/health", get(handlers::health))
                .nest("/todo/", todo_router()),
        )
        .fallback_service(static_folder)
        .with_state(service)
}
