use anathema::component::Component;

pub struct Home;

impl Component for Home {
    type State = ();

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}
