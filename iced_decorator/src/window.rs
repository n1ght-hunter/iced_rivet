use crate::drag_window::DragWindow;
use crate::helpers::reizer_operation::{SetState, RESIZER_ID};
use crate::resizer::{resizer, ResizeEvent};
use crate::{svgs, WindowHandler};
use iced::advanced::graphics::core::Element;

use iced::{
    widget::{button, container, horizontal_space, row, svg},
    window::{self, Id},
    Command, Length, Point, Rectangle, Size,
};

#[derive(Debug, Clone)]
pub enum TitleEvents {
    Minimize,
    Restore,
    Close,
}

#[derive(Debug, Clone)]
pub enum UpdateResizerState {
    Size(Size),
    Position(Option<Point>),
}

#[derive(Debug, Clone)]
pub enum WindowEvents {
    ResizeEvent(ResizeEvent),
    TitleEvent(TitleEvents),
    UpdateResizerState(UpdateResizerState),
    DragWindow,
}

/// window decorations with title bar, menu bar, and resizer
pub struct Window<'a, Message, Theme, Renderer> {
    /// content on the left of the menu bar
    content: Option<Element<'a, Message, Theme, Renderer>>,
    /// put title into the middle of the menu bar
    title: Option<String>,
}

impl<'a, Message, Theme, Renderer> Window<'a, Message, Theme, Renderer> {
    pub fn new() -> Self {
        Window {
            content: None,
            title: None,
        }
    }

    pub fn content(mut self, content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn view(self) -> Element<'a, Message, Theme, Renderer>
    where
        Message: 'a + Clone + WindowHandler,
        Renderer: 'a
            + iced::advanced::svg::Renderer
            + iced::advanced::Renderer
            + iced::advanced::text::Renderer,
        Theme: 'a
            + iced::widget::button::StyleSheet
            + iced::widget::container::StyleSheet
            + iced::widget::text::StyleSheet
            + iced::widget::svg::StyleSheet,
    {
        let title_bar_buttons = row![
            button(svg(svgs::MINIMIZE_SVG.clone()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::OtherMenu)
                .on_press(Message::event_handler(WindowEvents::TitleEvent(
                    TitleEvents::Minimize
                ))),
            button(svg(svgs::RESTORE_SVG.clone()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::OtherMenu)
                .on_press(Message::event_handler(WindowEvents::TitleEvent(
                    TitleEvents::Restore
                ))),
            button(svg(svgs::CLOSE_SVG.clone()).height(30.0))
                .width(50.0)
                // .style(menu_theme::Button::Close)
                .on_press(Message::event_handler(WindowEvents::TitleEvent(
                    TitleEvents::Close
                ))),
        ];

        let left_content = if let Some(content) = self.content {
            content
        } else {
            horizontal_space().into()
        };

        let menu_bar = container(row![
            left_content,
            DragWindow::with_width(
                Length::Fill,
                Message::event_handler(WindowEvents::DragWindow)
            )
            .set_title(self.title),
            title_bar_buttons
        ])
        .height(35.0);

        let window = resizer(menu_bar, |e| {
            Message::event_handler(WindowEvents::ResizeEvent(e))
        })
        .id(RESIZER_ID.clone());
        window.into()
    }
}

impl<'a, Message, Theme, Renderer> Into<Element<'a, Message, Theme, Renderer>>
    for Window<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone + WindowHandler,
    Renderer: 'a
        + iced::advanced::svg::Renderer
        + iced::advanced::Renderer
        + iced::advanced::text::Renderer,
    Theme: 'a
        + iced::widget::button::StyleSheet
        + iced::widget::container::StyleSheet
        + iced::widget::text::StyleSheet
        + iced::widget::svg::StyleSheet,
{
    fn into(self) -> Element<'a, Message, Theme, Renderer> {
        self.view()
    }
}

pub fn init<Message: WindowHandler>() -> Command<Message> {
    Command::batch(vec![
        window::fetch_size(Id::MAIN, |size| {
            Message::event_handler(WindowEvents::UpdateResizerState(UpdateResizerState::Size(
                size,
            )))
        }),
        window::fetch_position(Id::MAIN, |position| {
            Message::event_handler(WindowEvents::UpdateResizerState(
                UpdateResizerState::Position(position),
            ))
        }),
    ])
}

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
                return window::toggle_maximize(Id::MAIN);
            }
            TitleEvents::Close => {
                return window::close(Id::MAIN);
            }
        },
        WindowEvents::UpdateResizerState(urs) => match urs {
            UpdateResizerState::Size(size) => {
                return Command::widget(SetState::with_size(size));
            }
            UpdateResizerState::Position(position) => {
                if let Some(position) = position {
                    return Command::widget(SetState::with_position(position));
                }
            }
        },
        WindowEvents::DragWindow => {
            return window::drag(Id::MAIN);
        }
    }
    Command::none()
}
