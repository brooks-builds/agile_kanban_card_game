mod create_game;
pub mod healthcheck;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

use crate::routes::{create_game::create_game, healthcheck::healthcheck};

pub fn create_routes(db_pool: Pool<Postgres>) -> Router {
    let router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/games", post(create_game))
        .layer(Extension(db_pool));

    Router::new().nest("/api", router)
}
