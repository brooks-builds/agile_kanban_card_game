use std::{thread, time::Duration};

use anathema::{
    component::{Component, Emitter},
    state::{List, State, Value},
    store::slab::Key,
};

pub struct Message;

impl Component for Message {
    type State = MessageState;

    type Message = MessageWrapper;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            MessageWrapper::Error(message) => {
                let id = context.widget_id;
                let emitter = context.emitter;

                state.errors.push(message);
                send_message_remove(id, emitter.clone(), MessageWrapper::RemoveError);
                state.have_errors.set(true);
            }
            MessageWrapper::RemoveError => {
                state.errors.pop_front();

                if state.errors.is_empty() {
                    state.have_errors.set(false);
                }
            }
            MessageWrapper::Success(message) => {
                let id = context.widget_id;
                let emitter = context.emitter;

                state.successes.push(message);
                send_message_remove(id, emitter.clone(), MessageWrapper::RemoveSuccess);
                state.have_successes.set(true);
            }
            MessageWrapper::RemoveSuccess => {
                state.successes.pop_front();

                if state.successes.is_empty() {
                    state.have_successes.set(false);
                }
            }
        }
    }
}

#[derive(Debug, State)]
pub struct MessageState {
    pub errors: Value<List<String>>,
    pub successes: Value<List<String>>,
    pub have_errors: Value<bool>,
    pub have_successes: Value<bool>,
}

impl MessageState {
    pub fn new() -> Self {
        let errors = Value::new(List::empty());
        let successes = Value::new(List::empty());
        let have_errors = Value::new(false);
        let have_successes = Value::new(false);

        Self {
            errors,
            successes,
            have_errors,
            have_successes,
        }
    }
}

fn send_message_remove(id: Key, emitter: Emitter, message: MessageWrapper) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        emitter.try_emit(id, message).unwrap();
    });
}

pub enum MessageWrapper {
    Error(String),
    RemoveError,
    Success(String),
    RemoveSuccess,
}
