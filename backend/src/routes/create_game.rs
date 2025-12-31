use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::db::game_queries;

pub async fn create_game(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Json(data): Json<CreateGame>,
) -> Result<impl IntoResponse, StatusCode> {
    let player_name = data.player_name;
    let game_code = generate_game_code();
    let Ok(Some(game_id)) = game_queries::create_game(db_pool, &player_name, game_code).await
    else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok((
        StatusCode::CREATED,
        Json(CreateGameResponse {
            player_name,
            game_id,
            code: game_code,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct CreateGame {
    pub player_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateGameResponse {
    pub player_name: String,
    pub game_id: String,
    pub code: i32,
}

fn generate_game_code() -> i32 {
    let mut rng = rand::rng();
    let code = rng.random_range(1000..10000);

    code
}
