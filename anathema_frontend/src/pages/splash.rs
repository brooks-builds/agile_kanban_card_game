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
        mut children: anathema::component::Children<'_, '_>,
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
