use anathema::{component::Emitter, store::slab::Key};
use serde::{Deserialize, Serialize};
use std::thread;

use crate::components::message::MessageWrapper;

pub fn create_game(
    player_name: String,
    emitter: Emitter,
    send_to: Key,
    base_api_url: &str,
    message_id: Key,
) {
    let base_url = format!("{base_api_url}/api");

    thread::spawn(move || {
        let data = CreateGameData { player_name };
        let client = reqwest::blocking::Client::new();
        let response = match client.post(format!("{base_url}/games")).json(&data).send() {
            Ok(response) => response,
            Err(error) => {
                let message = MessageWrapper::Error(format!("{error}"));

                emitter.try_emit(message_id, message).ok();
                return;
            }
        };
        let created_game = match response.json::<CreateGameResponse>() {
            Ok(created_game) => created_game,
            Err(error) => {
                let message = MessageWrapper::Error(format!("{error}"));

                emitter.try_emit(message_id, message).ok();
                return;
            }
        };

        emitter.try_emit(send_to, created_game).ok();
    });
}

pub fn join_game(
    game_code: &str,
    emitter: Emitter,
    send_to: Key,
    base_api_url: &str,
    message_id: Key,
) {
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
