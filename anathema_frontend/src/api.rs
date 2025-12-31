use anathema::{
    component::{ComponentId, Emitter},
    store::slab::Key,
    widgets::components::deferred::QueryBuilder,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::IntoKeys,
    sync::mpsc::{channel, sync_channel},
    thread,
};

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

        println!("Game has been created: {response:?}");
        emitter.try_emit(send_to, response);
    });
}

#[derive(Debug, Serialize)]
pub struct CreateGameData {
    pub player_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameResponse {
    pub player_name: String,
    pub game_id: usize,
}
