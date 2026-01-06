use crate::components::message::MessageWrapper;
use anathema::{
    component::Component,
    state::{State, Value},
};

pub struct Splash;

impl Component for Splash {
    type State = SplashState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "set_player_name" => {
                let player_name = event.data::<String>();

                state.player_name.set(player_name.to_owned());
            }
            "create_game" => {
                let player_name = state.player_name.to_ref().to_owned();

                context.publish("create_game", player_name.to_owned());
                event.stop_propagation();
            }
            "join_game" => {
                let game_code = match event.data_checked::<String>() {
                    Some(code) => code,
                    None => {
                        let error_message = "Code missing or not a string".to_owned();

                        context
                            .components
                            .by_name("message")
                            .send(MessageWrapper::Error(error_message));

                        return;
                    }
                };
                let code = match game_code.parse::<i32>() {
                    Ok(code) => code,
                    Err(error) => {
                        let message =
                            MessageWrapper::Error(format!("Code must be a number: {error}"));

                        context.components.by_name("message").send(message);
                        return;
                    }
                };
                let player_name = state.player_name.to_ref().clone();
                let game = JoinGameEventData {
                    game_code: code,
                    player_name,
                };

                context.publish("join_game", game);
                event.stop_propagation();
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct SplashState {
    pub player_name: Value<String>,
}

impl SplashState {
    pub fn new() -> Self {
        let player_name = Value::new(String::new());

        Self { player_name }
    }
}

#[derive(Debug)]
pub struct JoinGameEventData {
    pub game_code: i32,
    pub player_name: String,
}
