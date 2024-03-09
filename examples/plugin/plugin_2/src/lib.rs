use iced::{
    advanced::graphics::core::Element,
    widget::{self, text},
};
use iced_plugin::{MessageType, PluginFunction};

#[no_mangle]
pub fn new_plugin() -> PluginFunction {
    Box::new(MyPlugin::default())
}

pub struct MyPlugin {
    counter: u16,
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Add,
    Remove,
}

impl iced_plugin::Plugin for MyPlugin {
    fn view(&self) -> iced::Element<'_, MessageType> {
        Element::from(widget::column![
            widget::button(text("Add")).on_press(Message::Add),
            widget::button(text("Remove")).on_press(Message::Remove),
            widget::text(self.counter.to_string()),
        ])
        .map(|m| MessageType::new(m))
    }

    fn update(&mut self, message: MessageType) -> iced::Command<MessageType> {
        let message = message.downcast::<Message>().unwrap();
        match message {
            Message::Add => self.counter += 1,
            Message::Remove => self.counter -= 1,
        }
        iced::Command::none()
    }
}
