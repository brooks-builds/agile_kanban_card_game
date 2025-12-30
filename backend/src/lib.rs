mod routes;

use crate::routes::create_routes;

pub async fn run() -> eyre::Result<()> {
    let app = create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
