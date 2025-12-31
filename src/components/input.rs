use std::ops::Deref;

use anathema::{component::Component, state::State};

pub struct Input;

impl Component for Input {
    type State = InputState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let mouse_position = mouse.pos();
        children
            .elements()
            .at_position(mouse_position)
            .first(|_, _| {
                let ae_input = context.components.by_name("ae_input");

                ae_input.focus();
            });
    }

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "on_enter" => {
                let value = event.data::<anathema_extras::Text>().deref().to_owned();

                context.publish("on_enter", value);
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct InputState {}

impl InputState {
    pub fn new() -> Self {
        Self {}
    }
}
