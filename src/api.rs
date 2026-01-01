use anathema::{component::Emitter, store::slab::Key};
use serde::{Deserialize, Serialize};
use std::thread;

pub fn create_game(player_name: String, emitter: Emitter, send_to: Key, base_api_url: &str) {
    let base_url = format!("{base_api_url}/api");

    thread::spawn(move || {
        let data = CreateGameData { player_name };
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{base_url}/games"))
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
