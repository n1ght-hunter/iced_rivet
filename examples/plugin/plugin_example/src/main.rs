use iced::{widget::{self, button, container, text}, Application};
use iced_plugin::{MessageType, PluginHandler, PluginLoader};

fn main() {
    let mut plugins = PluginHandler::new();

    plugins.load("target/release/plugin_1").unwrap();
    plugins.load("target/release/plugin_2").unwrap();

    App::run(iced::Settings::with_flags(plugins)).unwrap();
}

struct App {
    plugins: PluginHandler<Message>,
    plugin_id: u16,
}

#[derive(Debug, Clone)]
enum Message {
    Plugin(u16, MessageType),
    SetActive(u16),
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

    type Flags = PluginHandler<Message>;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                plugins: flags,
                plugin_id: 0,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "Plugin Example".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Plugin(id, message) => {
                return self.plugins.plugin_update(id, message);
            }
            Message::SetActive(i) => self.plugin_id = i,
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let header = self
            .plugins
            .plugin_info()
            .iter()
            .fold(iced::widget::Row::new(), |row, (id, name)| {
                row.push(button(text(name.clone())).on_press(Message::SetActive(id.clone())))
            });

        let page = self.plugins.plugin_view(self.plugin_id);

        let content = match page {
            Some(page) => page,
            None => iced::widget::Text::new("No plugin selected").into(),
        };

        container(widget::column![
            header,
            content,
        ]).into()
    }
}
