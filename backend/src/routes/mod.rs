mod create_game;
pub mod healthcheck;

use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::{create_game::create_game, healthcheck::healthcheck};

pub fn create_routes() -> Router {
    let router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/games", post(create_game));

    Router::new().nest("/api", router)
}
