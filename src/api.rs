use crate::{components::message::MessageWrapper, pages::home::HomeMessage};
use anathema::{component::Emitter, store::slab::Key};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read},
    thread,
    time::Duration,
};

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

        emitter
            .try_emit(send_to, HomeMessage::GameCreated(created_game))
            .ok();
    });
}

pub fn join_game(
    game_code: i32,
    player_name: String,
    emitter: Emitter,
    send_to: Key,
    base_api_url: &str,
    message_id: Key,
) {
    let url = format!("{base_api_url}/api/games/{game_code}");

    thread::spawn(move || {
        let data = JoinGameData { player_name };
        let client = reqwest::blocking::Client::new();
        let response = match client.post(url).json(&data).send() {
            Ok(response) => response,
            Err(error) => {
                let message = MessageWrapper::Error(format!("Error joining game: {error:?}"));

                emitter.try_emit(message_id, message).ok();
                return;
            }
        };
        let status_code = response.status();

        if matches!(status_code, StatusCode::CREATED) {
            let message = MessageWrapper::Success("Game joined".to_owned());
            let home_message = HomeMessage::GameJoined { game_code };

            emitter.try_emit(message_id, message).ok();
            emitter.try_emit(send_to, home_message).ok();
        }
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
    pub player_id: String,
}

#[derive(Debug, Serialize)]
pub struct JoinGameData {
    pub player_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LobbyPlayer {
    pub player_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetPlayersInLobbyResponse {
    pub data: Vec<LobbyPlayer>,
}

pub fn get_players_in_lobby(
    base_api_url: &str,
    game_code: i32,
    emitter: Emitter,
    message_id: Key,
    send_to: Key,
) {
    let api_url = format!("{base_api_url}/api/games/{game_code}/lobby");

    thread::spawn(move || {
        emitter
            .try_emit(
                message_id,
                MessageWrapper::Success(format!("About to get players in lobby")),
            )
            .ok();

        let client = reqwest::blocking::Client::new();
        let response = match client.get(api_url).send() {
            Ok(response) => response,
            Err(error) => {
                emitter
                    .try_emit(
                        message_id,
                        MessageWrapper::Error(format!(
                            "Error sending get players in lobby request: {error:?}"
                        )),
                    )
                    .ok();
                return;
            }
        };
        let status_code = response.status();

        emitter
            .try_emit(
                message_id,
                MessageWrapper::Success(format!("got a {status_code} response from the lobby")),
            )
            .ok();

        let mut response_reader = BufReader::new(response);

        loop {
            // stripping the leading data from the line. This is added by axum Event.
            let mut header = [0u8; 6];
            response_reader.read_exact(&mut header).unwrap();

            let mut line = String::new();
            if let Err(error) = response_reader.read_line(&mut line) {
                emitter
                    .try_emit(
                        message_id,
                        MessageWrapper::Error(format!(
                            "Error reading line from lobby api call: {error:?}"
                        )),
                    )
                    .ok();
                break;
            }

            let players = match serde_json::from_str::<Vec<LobbyPlayer>>(&line) {
                Ok(lobby_players) => lobby_players,
                Err(error) => {
                    emitter
                        .try_emit(
                            message_id,
                            MessageWrapper::Error(format!(
                                "Error converting player lobby from json: {error:?}"
                            )),
                        )
                        .ok();

                    return;
                }
            };

            emitter
                .try_emit(send_to, HomeMessage::LobbyPlayerUpdated(players))
                .ok();
        }
    });
}
