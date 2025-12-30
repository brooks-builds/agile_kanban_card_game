use std::thread::spawn;

use anathema::{
    component::Component,
    state::{State, Value},
};
use serde_json::json;

use crate::api::{self, CreateGameResponse};

pub struct Home;

impl Component for Home {
    type State = HomeState;

    type Message = CreateGameResponse;

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

                let emitter = context.emitter.clone();
                let component_id = context.components.by_name("home");

                api::create_game(player_name.clone(), context.components.by_name("home"));

                state.player_name.set(player_name);
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
        let game_id = message.game_id;

        state.game_id.set(game_id);
    }
}

#[derive(Debug, State)]
pub struct HomeState {
    player_name: Value<String>,
    game_id: Value<usize>,
}

impl HomeState {
    pub fn new() -> Self {
        let player_name = Value::new(String::new());
        let game_id = Value::new(0);

        Self {
            player_name,
            game_id,
        }
    }
}
