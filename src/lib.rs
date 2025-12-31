pub mod api;
mod components;
mod pages;

use crate::{
    components::{
        button::{Button, ButtonState},
        input::{Input, InputState},
    },
    pages::{
        home::{Home, HomeState},
        lobby::Lobby,
        splash::{Splash, SplashState},
    },
};
use anathema::{
    prelude::{Backend, Document, TuiBackend},
    runtime::{Error, Runtime},
};

pub fn run() -> Result<(), Error> {
    let document = Document::new("@home");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    backend.finalize();

    let mut builder = Runtime::builder(document, &backend);

    builder.default::<anathema_extras::Input>("ae_input", anathema_extras::Input::template())?;

    builder.prototype("input", "templates/input.aml", || Input, InputState::new)?;
    builder.prototype(
        "button",
        "templates/button.aml",
        || Button,
        ButtonState::new,
    )?;

    builder.component("home", "templates/home.aml", Home, HomeState::new())?;
    builder.component("splash", "templates/splash.aml", Splash, SplashState::new())?;
    builder.component("lobby", "templates/lobby.aml", Lobby, ())?;

    builder.finish(&mut backend, |runtime, backend| runtime.run(backend))
}
