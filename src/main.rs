use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use crate::infrastructure::todo_repository::TodoRepositoryImpl;
use crate::presentation::handlers::todo_handler::create_todo_router;
use crate::usecase::todo_usecase::TodoUsecase;

mod domain;
mod infrastructure;
mod presentation;
mod usecase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let todo_repository = TodoRepositoryImpl::new(pool.clone());
    let todo_service = TodoUsecase::new(Arc::new(todo_repository));

    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!!!" }))
        .nest("/api", create_todo_router(todo_service));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
