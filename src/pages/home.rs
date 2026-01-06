use crate::{
    api::{self, CreateGameResponse, LobbyPlayer},
    components::message::MessageWrapper,
    pages::splash,
};
use anathema::{
    component::Component,
    state::{List, State, Value},
};
use std::fmt::Display;
pub struct Home;

impl Component for Home {
    type State = HomeState;

    type Message = HomeMessage;

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "create_game" => {
                let player_name = event.data::<String>().to_owned();
                let api_url = state.api_url.to_ref();
                let emitter = context.emitter.clone();
                let key = context.widget_id;
                let message_key = match children
                    .components()
                    .by_name("message")
                    .first(|key, _, _| key)
                {
                    Some(key) => key,
                    None => {
                        context
                            .components
                            .by_name("message")
                            .send(MessageWrapper::Error(format!(
                                "can't find the message component in home to send messages to"
                            )));
                        return;
                    }
                };

                api::create_game(
                    player_name.clone(),
                    emitter,
                    key,
                    api_url.as_str(),
                    message_key,
                );

                state.player_name.set(player_name);
            }
            "nav_to" => {
                let screen_destination = event.data::<Screen>();
                let screen = screen_destination.to_string();

                state.screen.set(screen.clone());
                state.screen_history.push_back(screen);
            }
            "new_api_url" => {
                let new_api_url = event.data::<String>().to_owned();

                state.api_url.set(new_api_url);
            }
            "nav_back" => {
                state.screen_history.pop();
                let destination = match state.screen_history.pop() {
                    Some(previous) => previous.to_ref().to_string(),
                    None => Screen::Splash.to_string(),
                };

                state.screen.set(destination);
            }
            "exit" => {
                context.stop_runtime();
            }
            "join_game" => {
                event.stop_propagation();

                let Some(game_to_join) = event.data_checked::<splash::JoinGameEventData>() else {
                    context
                        .components
                        .by_name("message")
                        .send(MessageWrapper::Error(
                            "don't have game data to create a game".to_owned(),
                        ));

                    return;
                };
                let player_name = &game_to_join.player_name;
                let emitter = context.emitter;
                let key = context.widget_id;
                let base_api_url = state.api_url.to_ref().clone();
                let message_key = match children
                    .components()
                    .by_name("message")
                    .first(|key, _, _| key)
                {
                    Some(key) => key,
                    None => {
                        context
                            .components
                            .by_name("message")
                            .send(MessageWrapper::Error(format!(
                                "can't find the message component in home to send messages to"
                            )));
                        return;
                    }
                };

                state.player_name.set(player_name.clone());

                api::join_game(
                    game_to_join.game_code,
                    game_to_join.player_name.clone(),
                    emitter.clone(),
                    key,
                    &base_api_url,
                    message_key,
                );

                api::get_players_in_lobby(
                    &base_api_url,
                    game_to_join.game_code,
                    emitter.clone(),
                    message_key,
                    context.widget_id,
                );
            }
            _ => (),
        }
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            HomeMessage::GameCreated(create_game_response) => {
                let mut screen = state.screen.to_mut();

                match Screen::from(screen.as_str()) {
                    Screen::Splash => {
                        let game_id = create_game_response.game_id;
                        let game_code = create_game_response.code;
                        let lobby = Screen::Lobby.to_string();
                        let api_url = state.api_url.to_ref();
                        let message_key = match children
                            .components()
                            .by_name("message")
                            .first(|key, _, _| key)
                        {
                            Some(key) => key,
                            None => {
                                context
                                    .components
                                    .by_name("message")
                                    .send(MessageWrapper::Error(format!(
                                        "can't find the message component in home to send messages to"
                                    )));
                                return;
                            }
                        };

                        state.game_id.set(game_id);
                        state.game_code.set(game_code);
                        state.screen_history.push(lobby.clone());

                        *screen = lobby;

                        api::get_players_in_lobby(
                            api_url.as_str(),
                            game_code,
                            context.emitter.clone(),
                            message_key,
                            context.widget_id,
                        );
                    }
                    Screen::Lobby => (),
                    Screen::Config => (),
                }
            }
            HomeMessage::GameJoined { game_code } => {
                let mut current_screen = state.screen.to_mut();

                state.game_code.set(game_code);

                match Screen::from(current_screen.as_str()) {
                    Screen::Splash => {
                        let lobby_screen = Screen::Lobby.to_string();

                        *current_screen = lobby_screen;
                    }
                    Screen::Lobby => todo!(),
                    Screen::Config => todo!(),
                }
            }
            HomeMessage::LobbyPlayerUpdated(lobby_players) => {
                let list =
                    List::from_iter(lobby_players.into_iter().map(|player| player.player_name));

                state.other_player_names.set(list);
            }
        }
    }
}

#[derive(Debug, State)]
pub struct HomeState {
    player_name: Value<String>,
    game_id: Value<String>,
    game_code: Value<i32>,
    screen: Value<String>,
    api_url: Value<String>,
    screen_history: Value<List<String>>,
    other_player_names: Value<List<String>>,
}

impl HomeState {
    pub fn new() -> Self {
        let player_name = Value::new(String::new());
        let game_id = Value::new(String::new());
        let game_code = Value::new(0);
        let screen = Value::new(Screen::Splash.to_string());
        let api_url = Value::new("http://localhost:3000".to_owned());
        let screen_history = Value::new(List::empty());
        let other_player_names = Value::new(List::empty());

        Self {
            player_name,
            game_id,
            game_code,
            screen,
            api_url,
            screen_history,
            other_player_names,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Splash,
    Lobby,
    Config,
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Splash => "splash",
            Self::Lobby => "lobby",
            Self::Config => "config",
        };

        write!(f, "{text}")
    }
}

impl From<&str> for Screen {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "splash" => Self::Splash,
            "lobby" => Self::Lobby,
            "config" => Self::Config,
            _ => Self::Splash,
        }
    }
}

pub enum HomeMessage {
    GameCreated(CreateGameResponse),
    GameJoined { game_code: i32 },
    LobbyPlayerUpdated(Vec<LobbyPlayer>),
}
