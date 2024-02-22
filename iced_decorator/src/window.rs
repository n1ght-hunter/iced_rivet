use crate::drag_window::DragWindow;
use crate::resizer::{resizer, ResizeEvent, ResizeState};
use crate::{svgs, WindowHandler};
use iced::advanced::widget::Operation;
use iced::{
    widget::{button, container, horizontal_space, row, svg},
    window::{self, Id},
    Command, Element, Length, Point, Rectangle, Size,
};
use lazy_static::lazy_static;

pub struct Window;

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

impl Window {
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

    pub fn view<'a, Message, Theme, Renderer>(
        _content: impl Into<Element<'a, Message, Theme, Renderer>>,
        menu_bar: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
        title: Option<&'a str>,
    ) -> Element<'a, Message, Theme, Renderer>
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

        let menu_bar = container(row![
            if let Some(menu_bar) = menu_bar {
                menu_bar.into()
            } else {
                horizontal_space().into()
            },
            DragWindow::with_width(
                Length::Fill,
                Message::event_handler(WindowEvents::DragWindow)
            )
            .set_title(title),
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
}

lazy_static! {
    static ref RESIZER_ID: iced::advanced::widget::Id = iced::advanced::widget::Id::new("rsizer");
}
struct SetState {
    id: iced::advanced::widget::Id,
    position: Option<Point>,
    size: Option<Size>,
}

impl SetState {
    pub fn with_position(position: Point) -> Self {
        SetState {
            id: RESIZER_ID.clone(),
            position: Some(position),
            size: None,
        }
    }

    pub fn with_size(size: Size) -> Self {
        SetState {
            id: RESIZER_ID.clone(),
            position: None,
            size: Some(size),
        }
    }
}

impl<T> Operation<T> for SetState {
    fn container(
        &mut self,
        _id: Option<&iced::advanced::widget::Id>,
        _bounds: Rectangle,
        operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
    ) {
        operate_on_children(self)
    }

    fn custom(&mut self, state: &mut dyn std::any::Any, id: Option<&iced::advanced::widget::Id>) {
        if Some(&self.id) == id {
            if let Some(state) = state.downcast_mut::<ResizeState>() {
                if let Some(position) = self.position {
                    state.window_position = position;
                }
                if let Some(size) = self.size {
                    state.window_size = size;
                }
            }
        }
    }
}
