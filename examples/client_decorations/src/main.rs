use iced::{
    widget::{container, text},
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

impl iced_decorator::WindowHandler for Message {
    fn event_handler(event: iced_decorator::window::WindowEvents) -> Self {
        Message::WindowEvents(event)
    }
}

impl Application for MenuTester {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                title: "Menu Tester".to_string(),
            },
            iced_decorator::window::init::<Self::Message>(),
        )
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dracula
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::WindowEvents(event) => return iced_decorator::window::event_handler(event),
        };
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let content = container(text("Hello, World!"))
            .height(Length::Fill)
            .center_x()
            .center_y();

        iced_decorator::window::Window::new()
            .title(self.title.clone())
            .content(content).panel_theme(iced_decorator::PanelTheme::DARK)
            .into()
    }
}
