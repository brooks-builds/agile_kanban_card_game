use anathema::{component::Emitter, store::slab::Key};
use serde::{Deserialize, Serialize};
use std::thread;

const BASE_URL: &str = "http://backend:3000/api";

pub fn create_game(player_name: String, emitter: Emitter, send_to: Key) {
    thread::spawn(move || {
        let data = CreateGameData { player_name };
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{BASE_URL}/games"))
            .json(&data)
            .send()
            .unwrap()
            .json::<CreateGameResponse>()
            .unwrap();

        emitter.try_emit(send_to, response).unwrap();
    });
}

#[derive(Debug, Serialize)]
pub struct CreateGameData {
    pub player_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameResponse {
    pub player_name: String,
    pub game_id: String,
    pub code: i32,
}
