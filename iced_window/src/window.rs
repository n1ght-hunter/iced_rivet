use crate::drag_window::DragWindow;
use crate::resize_handler::{resizer, ResizeEvent, Resizer};
use crate::svgs;
use iced::widget::{container, horizontal_space, text};
use iced::window::{self, Id};
use iced::{
    command,
    widget::{button, row, svg},
    Command, Element,
};
use iced::{Length, Point, Rectangle, Size};

pub struct Window;

#[derive(Debug, Clone)]
pub enum TitleEvents {
    Minimize,
    Restore,
    Close,
}

#[derive(Debug, Clone)]
pub enum WindowEvents {
    ResizeEvent(ResizeEvent),
    TitleEvent(TitleEvents),
    DragWindow,
}

impl Window {
    pub fn view<'a, Message, Theme, Renderer, F>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        menu_bar: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
        event_handler: F,
        title: Option<&'a str>,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Message: 'a + Clone,
        Renderer: 'a
            + iced::advanced::svg::Renderer
            + iced::advanced::Renderer
            + iced::advanced::text::Renderer,
        Theme: 'a
            + iced::widget::button::StyleSheet
            + iced::widget::container::StyleSheet
            + iced::widget::text::StyleSheet
            + iced::widget::svg::StyleSheet,
        F: 'a + Clone + Fn(WindowEvents) -> Message,
    {
        let event_handler2 = event_handler.clone();
        let title_bar_buttons = row![
            button(svg(svgs::minimize_svg()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::OtherMenu)
                .on_press((event_handler)(WindowEvents::TitleEvent(
                    TitleEvents::Minimize
                ))),
            button(svg(svgs::restore()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::OtherMenu)
                .on_press((event_handler)(WindowEvents::TitleEvent(
                    TitleEvents::Restore
                ))),
            button(svg(svgs::close_svg()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::Close)
                .on_press((event_handler)(WindowEvents::TitleEvent(
                    TitleEvents::Close
                ))),
        ];

        let menu_bar = container(row![
            if let Some(menu_bar) = menu_bar {
                menu_bar.into()
            } else {
                horizontal_space().into()
            },
            DragWindow::with_width(Length::Fill, (event_handler)(WindowEvents::DragWindow))
                .set_title(title),
            title_bar_buttons
        ])
        .height(35.0);

        let window = resizer(text("text"), move |e| {
            (event_handler2)(WindowEvents::ResizeEvent(e))
        });
        window.into()
    }
}

impl Window {
    pub fn event_handler<Message>(event: WindowEvents) -> Command<Message> {
        match event {
            WindowEvents::ResizeEvent(re) => match re {
                ResizeEvent::ResizeXY(size) => {
                    return window::resize(Id::MAIN, size);
                }
                ResizeEvent::ResizeWindow(rec) => {
                    let Rectangle {
                        x,
                        y,
                        width,
                        height,
                    } = rec;
                    return Command::batch(vec![
                        window::resize(Id::MAIN, Size::new(width, height)),
                        window::move_to(Id::MAIN, Point::new(x, y)),
                    ]);
                }
            },
            WindowEvents::TitleEvent(te) => match te {
                TitleEvents::Minimize => {
                    return window::minimize(Id::MAIN, true);
                }
                TitleEvents::Restore => {
                    return window::minimize(Id::MAIN, false);
                }
                TitleEvents::Close => {
                    return window::close(Id::MAIN);
                }
            },
            WindowEvents::DragWindow => {
                return window::drag(Id::MAIN);
            }
        }
    }
}
