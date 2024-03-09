use iced_plugin::{PluginFunction, MessageType};

#[no_mangle]
pub fn new_plugin() -> PluginFunction {
    Box::new(MyPlugin)
}

pub struct MyPlugin;

#[derive(Debug)]
pub enum Message {}

impl iced_plugin::Plugin for MyPlugin {
    fn view(&self) -> iced::Element<'_, MessageType> {
        iced::widget::text("Hello, world!").into()
    }

    fn update(&mut self, message: MessageType) -> iced::Command<MessageType> {
        let message = message.downcast_ref::<MessageType>().unwrap();
        match message {
            _ => {}
        }
        iced::Command::none()
    }
}
