use crate::window::WindowEvents;

pub trait WindowHandler {
    fn event_handler(event: WindowEvents) -> Self;
}