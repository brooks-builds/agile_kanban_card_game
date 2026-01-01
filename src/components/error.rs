use std::{thread, time::Duration};

use anathema::{
    component::{Component, Emitter},
    state::{List, State, Value},
    store::slab::Key,
};

pub struct ErrorComponent;

impl Component for ErrorComponent {
    type State = ErrorState;

    type Message = ErrorMessage;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            ErrorMessage::Message(message) => {
                let id = context.widget_id;
                let emitter = context.emitter;

                state.messages.push(message);
                send_message_remove(id, emitter.clone());
                state.have_messages.set(true);
            }
            ErrorMessage::RemoveMessage => {
                state.messages.pop_front();

                if state.messages.is_empty() {
                    state.have_messages.set(false);
                }
            }
        }
    }
}

#[derive(Debug, State)]
pub struct ErrorState {
    pub messages: Value<List<String>>,
    pub have_messages: Value<bool>,
}

impl ErrorState {
    pub fn new() -> Self {
        let messages = Value::new(List::empty());
        let have_messages = Value::new(false);

        Self {
            messages,
            have_messages,
        }
    }
}

fn send_message_remove(id: Key, emitter: Emitter) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        emitter.try_emit(id, ErrorMessage::RemoveMessage).unwrap();
    });
}

pub enum ErrorMessage {
    Message(String),
    RemoveMessage,
}
