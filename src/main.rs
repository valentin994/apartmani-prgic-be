use prgic_backend::app;
use prgic_backend::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite.db".to_string());
    let state = AppState::new(&database_url).await?;
    sqlx::migrate!()
        .run(&state.db)
        .await
        .expect("Failed migration");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app(state)).await?;
    Ok(())
}
