use iced::{
    widget::{container, row, text},
    Application, Length,
};

fn main() {
    MenuTester::run(iced::Settings {
        window: iced::window::Settings {
            decorations: false,
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    })
    .unwrap();
}

struct MenuTester {
    title: String,
}

#[derive(Debug, Clone)]
enum Message {
    WindowEvents(iced_decorator::window::WindowEvents),
}

impl Application for MenuTester {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                title: "Menu Tester".to_string(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::WindowEvents(event) => {
                return iced_decorator::window::Window::event_handler(event)
            }
        };
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let content = container(text("Hello, World!"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        iced_decorator::window::Window::view(
            row![content],
            Some(text("testing")),
            Message::WindowEvents,
            Some("Menu Tester"),
        )
        .into()
    }
}
