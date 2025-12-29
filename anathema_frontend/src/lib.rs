mod pages;

use crate::pages::home::Home;
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

    builder.component("home", "templates/home.aml", Home, ())?;

    builder.finish(&mut backend, |runtime, backend| runtime.run(backend))
}
