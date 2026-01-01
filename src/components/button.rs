use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::components::message::MessageWrapper;

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

                state.cached_background_color.set(background_color.clone());

                *background_color = "#005500".to_owned();
            });
        }

        if mouse.left_up() {
            children.elements().at_position(mouse.pos()).first(|_, _| {
                let mut background_color = state.background_color.to_mut();
                let cached_background_color = state.cached_background_color.to_ref();

                *background_color = cached_background_color.clone();
                context.publish("on_click", ());
            });
        }
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(style) = context.attribute("style") {
            let Some(style) = style.as_str() else {
                context
                    .components
                    .by_name("message")
                    .send(MessageWrapper::Error(
                        "style attribute for button must be a string".to_owned(),
                    ));

                return;
            };

            match style {
                "none" => state.background_color.set("".to_owned()),
                "success" => state.background_color.set("#00aa00".to_owned()),
                "danger" => state.background_color.set("#e9a14f".to_owned()),
                _ => state.background_color.set("green".to_owned()),
            };
        }
    }
}

#[derive(Debug, State)]
pub struct ButtonState {
    background_color: Value<String>,
    cached_background_color: Value<String>,
}

impl ButtonState {
    pub fn new() -> Self {
        let background_color = Value::new("".to_owned());
        let cached_background_color = Value::new("".to_owned());

        Self {
            background_color,
            cached_background_color,
        }
    }
}
