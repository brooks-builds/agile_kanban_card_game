use anathema::{
    component::Component,
    state::{State, Value},
};
pub struct Config;

impl Component for Config {
    type State = ConfigState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "changing_api_url" => state.changing_api_url.set(true),
            "new_api_url" => {
                let new_api_text = event.data::<String>().to_string();

                event.stop_propagation();
                context.publish("new_api_url", new_api_text);
                state.changing_api_url.set(false);
            }
            "close" => {
                event.stop_propagation();
                context.publish("nav_back", ());
            }
            "new_api_cancel" => {
                event.stop_propagation();
                state.changing_api_url.set(false);
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct ConfigState {
    changing_api_url: Value<bool>,
}

impl ConfigState {
    pub fn new() -> Self {
        let changing_api_url = Value::new(false);

        Self { changing_api_url }
    }
}
