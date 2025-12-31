use anathema::component::Component;

use crate::pages::home::Screen;

pub struct Nav;

impl Component for Nav {
    type State = ();

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "open_config" => context.publish("nav_to", Screen::Config),
            _ => (),
        }
    }
}
