use iced::Application;
use iced_plugin::{MessageType, PluginHandler, PluginLoader};

fn main() {
    let mut plugins = PluginHandler::new();

    plugins.load(0, "target/release/my_plugin").unwrap();

    App::run(iced::Settings::with_flags(plugins)).unwrap();
}

struct App {
    plugins: PluginHandler<Message>,
}

#[derive(Debug)]
enum Message {
    Plugin(u16, MessageType),
}

impl PluginLoader for Message {
    fn plugin_message(id: u16, message: MessageType) -> Self {
        Message::Plugin(id, message)
    }
}

impl Application for App {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = PluginHandler<Message> ;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App { plugins: flags }, iced::Command::none())
    }

    fn title(&self) -> String {
        "Plugin Example".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Plugin(id, message) => {
                return self.plugins.plugin_update(id, message);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        self.plugins.plugin_view(0)
    }
}
