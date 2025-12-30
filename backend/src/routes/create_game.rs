use std::{thread, time::Duration};

use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

pub async fn create_game(Json(data): Json<CreateGame>) -> (StatusCode, Json<CreateGameResponse>) {
    let player_name = data.player_name;
    let game_id = 15;
    let response_code = StatusCode::CREATED;
    let response = CreateGameResponse {
        player_name,
        game_id,
    };

    (response_code, Json(response))
}

#[derive(Debug, Deserialize)]
pub struct CreateGame {
    pub player_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateGameResponse {
    pub player_name: String,
    pub game_id: usize,
}
