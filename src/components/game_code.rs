use anathema::{component::Component, state::State};
use arboard::Clipboard;

use crate::components::message::MessageWrapper;

pub struct GameCode;

#[derive(Debug, State)]
pub struct GameCodeState {}

impl GameCodeState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for GameCode {
    type State = GameCodeState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "copy" => {
                if let Some(code) = context.attributes.get_as::<i32>("code") {
                    let mut clipboard = match Clipboard::new() {
                        Ok(clipboard) => clipboard,
                        Err(error) => {
                            let error = MessageWrapper::Error(format!("{error}"));
                            context.components.by_name("message").send(error);
                            return;
                        }
                    };

                    if let Err(error) = clipboard.set_text(code.to_string()) {
                        let message = MessageWrapper::Error(format!(
                            "Error copying the code to the clipboard: {error}"
                        ));

                        context.components.by_name("message").send(message);
                        return;
                    }

                    let message = MessageWrapper::Success(String::from("Copied code to clipboard"));

                    context.components.by_name("message").send(message);
                } else {
                    let error = MessageWrapper::Error(String::from(
                        "game code component doesn't have code as an attribute, or it's not a &str",
                    ));

                    context.components.by_name("message").send(error);
                }

                event.stop_propagation();
            }
            _ => (),
        }
    }
}
