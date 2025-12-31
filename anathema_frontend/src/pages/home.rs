use crate::api::{self, CreateGameResponse};
use anathema::{
    component::Component,
    state::{State, Value},
};

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
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "create_game" => {
                let player_name = event.data::<String>().to_owned();

                let emitter = context.emitter.clone();
                let key = context.widget_id;

                api::create_game(player_name.clone(), emitter, key);

                state.player_name.set(player_name);
            }
            _ => (),
        }
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let game_id = message.game_id;
        let game_code = message.code;

        state.game_id.set(game_id);
        state.game_code.set(game_code);
    }
}

#[derive(Debug, State)]
pub struct HomeState {
    player_name: Value<String>,
    game_id: Value<String>,
    game_code: Value<i32>,
}

impl HomeState {
    pub fn new() -> Self {
        let player_name = Value::new(String::new());
        let game_id = Value::new(String::new());
        let game_code = Value::new(0);

        Self {
            player_name,
            game_id,
            game_code,
        }
    }
}
