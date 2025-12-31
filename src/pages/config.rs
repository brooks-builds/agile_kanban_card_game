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
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "changing_api_url" => state.changing_api_url.set(true),
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
