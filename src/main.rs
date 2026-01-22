use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;

use tower_http::services::ServeDir;

use dotenv::dotenv;
use std::{env, process};

mod handlers;
mod repository;

use crate::handlers::todo::todo_router;
use crate::repository::conf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Парсинг данных с .env файла
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Не нашёл в .env DATABASE_URL");
    let port: u16 = env::var("PORT")
        .expect("Cannot find PORT var in .env")
        .parse()
        .unwrap_or(0);

    let ip_addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(ip_addr).await?;

    // В случаи если порт не задан в .env он задаётся рандомно и указывается актуальный
    // адрес здесь
    let actuall_ip = listener.local_addr()?;

    let pool = match conf::configure_db(&db_url).await {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Cannot connect to database: {err}");
            process::exit(1);
        }
    };

    let state = AppState { db: pool };

    println!("Server is connect to: {}", actuall_ip);

    axum::serve(listener, app(state)).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    db: sqlx::MySqlPool,
}

fn app(state: AppState) -> Router {
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
        .with_state(state)
}
