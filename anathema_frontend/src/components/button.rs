use anathema::{
    component::Component,
    state::{State, Value},
};

pub struct Button;

impl Component for Button {
    type State = ButtonState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if mouse.left_down() {
            children.elements().at_position(mouse.pos()).first(|_, _| {
                let mut background_color = state.background_color.to_mut();

                *background_color = "#005500".to_owned();
            });
        }

        if mouse.left_up() {
            children.elements().at_position(mouse.pos()).first(|_, _| {
                let mut background_color = state.background_color.to_mut();

                *background_color = "#00aa00".to_owned();
                context.publish("on_click", ());
            });
        }
    }
}

#[derive(Debug, State)]
pub struct ButtonState {
    background_color: Value<String>,
}

impl ButtonState {
    pub fn new() -> Self {
        let background_color = "#00aa00".to_owned().into();

        Self { background_color }
    }
}
