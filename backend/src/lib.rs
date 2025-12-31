mod db;
mod routes;

use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::routes::create_routes;

pub async fn run() -> eyre::Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let app = create_routes(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
